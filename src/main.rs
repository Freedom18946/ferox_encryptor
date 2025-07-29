// src/main.rs

//! # Ferox Encryptor 主程序入口
//! 
//! 该文件是命令行工具 (CLI) 的主入口点。
//! 它负责：
//! 1. 解析命令行参数。
//! 2. 初始化日志和信号处理。
//! 3. 根据用户提供的子命令，调用核心库中对应的功能。
//! 4. 处理用户交互，如密码输入。
//! 5. 向用户报告操作结果。

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use ferox_encryptor::{
    batch::{
        batch_decrypt_directory, batch_decrypt_files, batch_encrypt_directory, batch_encrypt_files,
        BatchConfig,
    },
    decrypt::run_decryption_flow,
    encrypt::run_encryption_flow,
    keyfile::{validate_keyfile, KeyFile},
    Level,
};
use glob::Pattern;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use zeroize::Zeroize;

/// # Ferox Encryptor CLI
/// 
/// 一个基于 Rust 的高性能、抗暴力破解的本地文件加密工具。
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// 定义所有可用的子命令。
    #[command(subcommand)]
    command: Commands,
}

/// # 子命令枚举
/// 
/// 定义了所有用户可以执行的操作。
#[derive(Subcommand)]
enum Commands {
    /// 加密单个文件或多个指定文件。
    Encrypt {
        /// 要加密的一个或多个文件的路径。
        #[arg(required = true, num_args = 1..)]
        paths: Vec<PathBuf>,

        /// 如果目标文件已存在，则强制覆盖。
        #[arg(short, long)]
        force: bool,

        /// 设置加密的安全级别。
        #[arg(long, value_enum, default_value_t = Level::Moderate)]
        level: Level,

        /// (可选) 提供一个密钥文件以增强安全性。
        #[arg(short, long)]
        keyfile: Option<PathBuf>,
    },
    /// 解密单个或多个文件。
    Decrypt {
        /// 要解密的一个或多个 `.feroxcrypt` 文件的路径。
        #[arg(required = true, num_args = 1..)]
        paths: Vec<PathBuf>,

        /// (可选) 提供加密时使用的密钥文件。
        #[arg(short, long)]
        keyfile: Option<PathBuf>,
    },
    /// 批量加密一个目录中的所有文件。
    BatchEncrypt {
        /// 包含要加密文件的目录。
        #[arg(required = true)]
        directory: PathBuf,

        /// 如果目标文件已存在，则强制覆盖。
        #[arg(short, long)]
        force: bool,

        /// 设置加密的安全级别。
        #[arg(long, value_enum, default_value_t = Level::Moderate)]
        level: Level,

        /// 递归处理所有子目录。
        #[arg(short, long)]
        recursive: bool,

        /// (可选) 用于包含文件的 glob 模式 (例如: "*.txt", "data_*.csv")。
        /// 可以多次使用此参数。
        #[arg(long, name = "include")]
        include_patterns: Vec<String>,

        /// (可选) 用于排除文件的 glob 模式。
        /// 可以多次使用此参数。
        #[arg(long, name = "exclude")]
        exclude_patterns: Vec<String>,

        /// (可选) 提供一个密钥文件以增强安全性。
        #[arg(short, long)]
        keyfile: Option<PathBuf>,
    },
    /// 批量解密一个目录中的所有加密文件。
    BatchDecrypt {
        /// 包含要解密文件的目录。
        #[arg(required = true)]
        directory: PathBuf,

        /// 递归处理所有子目录。
        #[arg(short, long)]
        recursive: bool,

        /// (可选) 提供加密时使用的密钥文件。
        #[arg(short, long)]
        keyfile: Option<PathBuf>,
    },
    /// 生成一个新的、安全的密钥文件。
    GenerateKey {
        /// 新密钥文件的保存路径。
        #[arg(required = true)]
        output: PathBuf,
    },
}

/// 主函数入口。
fn main() -> Result<()> {
    // 初始化日志记录器，默认日志级别为 "info"
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // 创建一个线程安全的共享变量，用于在程序中断时传递临时文件名。
    // `Arc` 用于多线程所有权，`Mutex` 用于安全地修改数据。
    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
    let handler_path_ref = Arc::clone(&temp_file_path);

    // 设置 Ctrl+C 信号处理器。
    // 当用户按下 Ctrl+C 时，这个闭包会被执行。
    ctrlc::set_handler(move || {
        log::info!("\n接收到 Ctrl+C 信号，正在准备退出...");
        // 检查共享变量中是否有临时文件名
        if let Some(path) = handler_path_ref.lock().unwrap().as_ref() {
            if path.exists() {
                log::warn!("检测到操作被中断，正在清理不完整的输出文件: {}", path.display());
                // 尝试删除不完整的文件
                if let Err(e) = std::fs::remove_file(path) {
                    log::error!("清理文件 {} 失败: {}", path.display(), e);
                } else {
                    log::info!("清理完成。");
                }
            }
        }
        // 以标准的中断退出码 (130) 退出程序
        std::process::exit(130);
    })
    .context("设置 Ctrl-C 处理器时出错")?;

    // 解析命令行参数
    let cli = Cli::parse();

    // 使用 match 语句处理不同的子命令
    let result = match &cli.command {
        // --- 加密命令 ---
        Commands::Encrypt { paths, force, level, keyfile } => {
            let mut password = rpassword::prompt_password("请输入密码 (输入时不可见): ")
                .context("无法读取密码")?;
            
            let loaded_keyfile = load_keyfile_if_provided(keyfile)?;

            let config = BatchConfig {
                level: *level,
                force_overwrite: *force,
                ..Default::default()
            };

            let result = batch_encrypt_files(paths, &password, loaded_keyfile.as_ref(), &config)?;
            print_batch_result(&result, "加密");
            
            password.zeroize();
            Ok(())
        }
        // --- 解密命令 ---
        Commands::Decrypt { paths, keyfile } => {
            let mut password = rpassword::prompt_password("请输入密码 (输入时不可见): ")
                .context("无法读取密码")?;

            let loaded_keyfile = load_keyfile_if_provided(keyfile)?;

            let result = batch_decrypt_files(paths, &password, loaded_keyfile.as_ref())?;
            print_batch_result(&result, "解密");

            password.zeroize();
            Ok(())
        }
        // --- 批量加密命令 ---
        Commands::BatchEncrypt { directory, force, level, recursive, include_patterns, exclude_patterns, keyfile } => {
            let mut password = rpassword::prompt_password("请输入密码 (输入时不可见): ")
                .context("无法读取密码")?;

            let loaded_keyfile = load_keyfile_if_provided(keyfile)?;

            let config = BatchConfig {
                level: *level,
                force_overwrite: *force,
                recursive: *recursive,
                include_patterns: parse_patterns(include_patterns, "include")?,
                exclude_patterns: parse_patterns(exclude_patterns, "exclude")?,
            };
            
            let result = batch_encrypt_directory(directory, &password, loaded_keyfile.as_ref(), &config)?;
            print_batch_result(&result, "批量加密");

            password.zeroize();
            Ok(())
        }
        // --- 批量解密命令 ---
        Commands::BatchDecrypt { directory, recursive, keyfile } => {
            let mut password = rpassword::prompt_password("请输入密码 (输入时不可见): ")
                .context("无法读取密码")?;

            let loaded_keyfile = load_keyfile_if_provided(keyfile)?;

            let config = BatchConfig {
                recursive: *recursive,
                ..Default::default()
            };
            
            let result = batch_decrypt_directory(directory, &password, loaded_keyfile.as_ref(), &config)?;
            print_batch_result(&result, "批量解密");

            password.zeroize();
            Ok(())
        }
        // --- 生成密钥文件命令 ---
        Commands::GenerateKey { output } => {
            if output.exists() {
                log::warn!("密钥文件已存在: {}", output.display());
                let confirm = rpassword::prompt_password("是否覆盖? (输入 'yes' 确认): ")?;
                if confirm.to_lowercase() != "yes" {
                    log::info!("操作已取消。");
                    return Ok(());
                }
            }
            
            let keyfile = KeyFile::generate();
            keyfile.save_to_file(output)?;
            log::info!("✅ 密钥文件已成功生成: {}", output.display());
            log::warn!("请务必妥善保管此密钥文件，并制作备份。如果丢失，任何使用此密钥文件加密的数据都将永久无法恢复！");
            
            Ok(())
        }
    };

    // 统一处理所有命令可能返回的错误
    if let Err(e) = result {
        log::error!("操作失败: {e:#}");
        std::process::exit(1);
    }

    Ok(())
}

/// 如果用户提供了密钥文件路径，则加载并验证它。
fn load_keyfile_if_provided(keyfile_path: &Option<PathBuf>) -> Result<Option<KeyFile>> {
    match keyfile_path {
        Some(path) => {
            validate_keyfile(path)?;
            let keyfile = KeyFile::load_from_file(path)?;
            Ok(Some(keyfile))
        }
        None => Ok(None),
    }
}

/// 解析字符串形式的 glob 模式。
fn parse_patterns(patterns_str: &[String], pattern_type: &str) -> Result<Vec<Pattern>> {
    if patterns_str.is_empty() && pattern_type == "include" {
        return Ok(vec![Pattern::new("*")?]);
    }
    patterns_str
        .iter()
        .map(|s| Pattern::new(s).with_context(|| format!("无效的 '{}' 模式: {}", pattern_type, s)))
        .collect()
}

/// 打印批量操作的结果。
fn print_batch_result(result: &ferox_encryptor::BatchResult, operation_name: &str) {
    log::info!(
        "{}完成: {} 个成功, {} 个失败。",
        operation_name,
        result.success_count,
        result.failure_count
    );

    if result.failure_count > 0 {
        log::warn!("失败的文件列表:");
        for (path, error) in &result.failures {
            log::warn!("  - {}: {}", path.display(), error);
        }
    }
}