// src/interactive.rs

//! # 交互式用户界面模块
//!
//! 该模块提供了一个用户友好的交互式命令行界面，允许用户在运行时
//! 通过菜单和提示进行文件加密和解密操作。
//!
//! *This module provides a user-friendly interactive command-line interface
//! that allows users to perform file encryption and decryption operations
//! through menus and prompts at runtime.*

use crate::{
    batch::{batch_decrypt_directory, batch_decrypt_files, batch_encrypt_directory, batch_encrypt_files, BatchConfig},
    keyfile::{validate_keyfile, KeyFile},
    Level,
};
use anyhow::{Context, Result};
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use zeroize::Zeroize;

/// # 主菜单选项
///
/// 定义交互式界面的主要操作选项
#[derive(Debug, Clone)]
enum MainMenuOption {
    EncryptFile,
    DecryptFile,
    BatchEncryptDirectory,
    BatchDecryptDirectory,
    GenerateKeyFile,
    Help,
    Exit,
}

impl MainMenuOption {
    /// 获取菜单选项的显示文本
    fn display_text(&self) -> &'static str {
        match self {
            Self::EncryptFile => "🔒 加密文件 (Encrypt Files)",
            Self::DecryptFile => "🔓 解密文件 (Decrypt Files)",
            Self::BatchEncryptDirectory => "📁 批量加密目录 (Batch Encrypt Directory)",
            Self::BatchDecryptDirectory => "📂 批量解密目录 (Batch Decrypt Directory)",
            Self::GenerateKeyFile => "🔑 生成密钥文件 (Generate Key File)",
            Self::Help => "❓ 帮助信息 (Help)",
            Self::Exit => "🚪 退出程序 (Exit)",
        }
    }

    /// 获取所有菜单选项
    fn all_options() -> Vec<Self> {
        vec![
            Self::EncryptFile,
            Self::DecryptFile,
            Self::BatchEncryptDirectory,
            Self::BatchDecryptDirectory,
            Self::GenerateKeyFile,
            Self::Help,
            Self::Exit,
        ]
    }
}

/// # 交互式CLI主入口
///
/// 启动交互式用户界面，提供菜单驱动的操作体验
pub fn run_interactive_mode() -> Result<()> {
    let term = Term::stdout();
    let theme = ColorfulTheme::default();

    // 显示欢迎信息
    display_welcome_banner(&term)?;

    // 创建临时文件路径共享状态
    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));

    loop {
        // 显示主菜单
        let options = MainMenuOption::all_options();
        let option_texts: Vec<&str> = options.iter().map(|opt| opt.display_text()).collect();

        term.write_line("")?;
        term.write_line(&style("📋 请选择操作 (Please select an operation):").bold().to_string())?;

        let selection = Select::with_theme(&theme)
            .items(&option_texts)
            .default(0)
            .interact_on(&term)?;

        let selected_option = &options[selection];

        // 处理用户选择
        match selected_option {
            MainMenuOption::EncryptFile => {
                if let Err(e) = handle_encrypt_files(&term, &theme, Arc::clone(&temp_file_path)) {
                    display_error(&term, &e)?;
                }
            }
            MainMenuOption::DecryptFile => {
                if let Err(e) = handle_decrypt_files(&term, &theme, Arc::clone(&temp_file_path)) {
                    display_error(&term, &e)?;
                }
            }
            MainMenuOption::BatchEncryptDirectory => {
                if let Err(e) = handle_batch_encrypt_directory(&term, &theme, Arc::clone(&temp_file_path)) {
                    display_error(&term, &e)?;
                }
            }
            MainMenuOption::BatchDecryptDirectory => {
                if let Err(e) = handle_batch_decrypt_directory(&term, &theme, Arc::clone(&temp_file_path)) {
                    display_error(&term, &e)?;
                }
            }
            MainMenuOption::GenerateKeyFile => {
                if let Err(e) = handle_generate_keyfile(&term, &theme) {
                    display_error(&term, &e)?;
                }
            }
            MainMenuOption::Help => {
                display_help(&term)?;
            }
            MainMenuOption::Exit => {
                term.write_line(&style("👋 感谢使用 Ferox Encryptor! (Thank you for using Ferox Encryptor!)").green().to_string())?;
                break;
            }
        }

        // 询问是否继续
        if !matches!(selected_option, MainMenuOption::Exit | MainMenuOption::Help) {
            term.write_line("")?;
            let continue_prompt = Confirm::with_theme(&theme)
                .with_prompt("是否继续使用? (Continue?)")
                .default(true)
                .interact_on(&term)?;

            if !continue_prompt {
                term.write_line(&style("👋 感谢使用 Ferox Encryptor! (Thank you for using Ferox Encryptor!)").green().to_string())?;
                break;
            }
        }
    }

    Ok(())
}

/// 显示欢迎横幅
fn display_welcome_banner(term: &Term) -> Result<()> {
    term.clear_screen()?;
    term.write_line(&style("🔐 Ferox Encryptor - 交互式模式 (Interactive Mode)").bold().cyan().to_string())?;
    term.write_line(&style("═".repeat(60)).dim().to_string())?;
    term.write_line(&style("高性能文件加密工具 - 交互式用户界面").italic().to_string())?;
    term.write_line(&style("High-performance file encryption tool - Interactive UI").italic().to_string())?;
    term.write_line(&style("═".repeat(60)).dim().to_string())?;
    Ok(())
}

/// 显示错误信息
fn display_error(term: &Term, error: &anyhow::Error) -> Result<()> {
    term.write_line("")?;
    term.write_line(&style("❌ 操作失败 (Operation Failed):").red().bold().to_string())?;
    term.write_line(&style(format!("   {}", error)).red().to_string())?;
    term.write_line("")?;
    Ok(())
}

/// 显示帮助信息
fn display_help(term: &Term) -> Result<()> {
    term.write_line("")?;
    term.write_line(&style("📖 Ferox Encryptor 帮助信息 (Help Information)").bold().cyan().to_string())?;
    term.write_line(&style("─".repeat(50)).dim().to_string())?;
    term.write_line("")?;
    
    term.write_line("🔒 加密功能 (Encryption Features):")?;
    term.write_line("   • 使用 AES-256-CTR + HMAC-SHA256 军用级加密")?;
    term.write_line("   • 支持三种安全级别: Interactive, Moderate, Paranoid")?;
    term.write_line("   • 可选密钥文件双重保护")?;
    term.write_line("")?;
    
    term.write_line("📁 批量处理 (Batch Processing):")?;
    term.write_line("   • 支持目录递归处理")?;
    term.write_line("   • 文件模式过滤 (include/exclude patterns)")?;
    term.write_line("   • 详细的处理结果报告")?;
    term.write_line("")?;
    
    term.write_line("🔑 密钥文件 (Key Files):")?;
    term.write_line("   • 生成安全的随机密钥文件")?;
    term.write_line("   • 提供额外的安全保护层")?;
    term.write_line("   • 即使密码泄露也无法解密")?;
    term.write_line("")?;
    
    term.write_line(&style("💡 提示: 使用方向键导航菜单，回车键确认选择").yellow().to_string())?;
    term.write_line(&style("💡 Tip: Use arrow keys to navigate menus, Enter to confirm").yellow().to_string())?;

    Ok(())
}

/// 处理文件加密操作
fn handle_encrypt_files(
    term: &Term,
    theme: &ColorfulTheme,
    _temp_file_path: Arc<Mutex<Option<PathBuf>>>,
) -> Result<()> {
    term.write_line("")?;
    term.write_line(&style("🔒 文件加密 (File Encryption)").bold().cyan().to_string())?;
    term.write_line(&style("─".repeat(30)).dim().to_string())?;

    // 获取要加密的文件路径
    let file_paths = get_file_paths_input(term, theme, "请输入要加密的文件路径 (Enter file paths to encrypt)")?;

    // 选择安全级别
    let level = select_security_level(term, theme)?;

    // 询问是否使用密钥文件
    let keyfile = get_optional_keyfile(term, theme)?;

    // 询问是否强制覆盖
    let force_overwrite = Confirm::with_theme(theme)
        .with_prompt("如果目标文件已存在，是否强制覆盖? (Force overwrite if target exists?)")
        .default(false)
        .interact_on(term)?;

    // 显示操作预览
    display_operation_preview(term, "加密 (Encryption)", &file_paths, level, keyfile.as_ref(), force_overwrite)?;

    // 确认执行
    let confirm = Confirm::with_theme(theme)
        .with_prompt("确认执行加密操作? (Confirm encryption operation?)")
        .default(true)
        .interact_on(term)?;

    if !confirm {
        term.write_line(&style("操作已取消 (Operation cancelled)").yellow().to_string())?;
        return Ok(());
    }

    // 获取密码
    let mut password = rpassword::prompt_password("请输入密码 (输入时不可见): ")
        .context("无法读取密码")?;

    // 执行加密
    let config = BatchConfig {
        level,
        force_overwrite,
        ..Default::default()
    };

    term.write_line("")?;
    term.write_line(&style("正在执行加密操作... (Executing encryption...)").cyan().to_string())?;

    let result = batch_encrypt_files(&file_paths, &password, keyfile.as_ref(), &config)?;

    // 显示结果
    display_batch_result(term, &result, "加密 (Encryption)")?;

    password.zeroize();
    Ok(())
}

/// 处理文件解密操作
fn handle_decrypt_files(
    term: &Term,
    theme: &ColorfulTheme,
    _temp_file_path: Arc<Mutex<Option<PathBuf>>>,
) -> Result<()> {
    term.write_line("")?;
    term.write_line(&style("🔓 文件解密 (File Decryption)").bold().cyan().to_string())?;
    term.write_line(&style("─".repeat(30)).dim().to_string())?;

    // 获取要解密的文件路径
    let file_paths = get_file_paths_input(term, theme, "请输入要解密的 .feroxcrypt 文件路径 (Enter .feroxcrypt file paths to decrypt)")?;

    // 验证文件扩展名
    for path in &file_paths {
        if !path.extension().map_or(false, |ext| ext == "feroxcrypt") {
            term.write_line(&style(format!("⚠️  警告: {} 不是 .feroxcrypt 文件", path.display())).yellow().to_string())?;
        }
    }

    // 询问是否使用密钥文件
    let keyfile = get_optional_keyfile(term, theme)?;

    // 显示操作预览
    display_operation_preview(term, "解密 (Decryption)", &file_paths, Level::Moderate, keyfile.as_ref(), false)?;

    // 确认执行
    let confirm = Confirm::with_theme(theme)
        .with_prompt("确认执行解密操作? (Confirm decryption operation?)")
        .default(true)
        .interact_on(term)?;

    if !confirm {
        term.write_line(&style("操作已取消 (Operation cancelled)").yellow().to_string())?;
        return Ok(());
    }

    // 获取密码
    let mut password = rpassword::prompt_password("请输入密码 (输入时不可见): ")
        .context("无法读取密码")?;

    // 执行解密
    term.write_line("")?;
    term.write_line(&style("正在执行解密操作... (Executing decryption...)").cyan().to_string())?;

    let result = batch_decrypt_files(&file_paths, &password, keyfile.as_ref())?;

    // 显示结果
    display_batch_result(term, &result, "解密 (Decryption)")?;

    password.zeroize();
    Ok(())
}

/// 处理批量目录加密操作
fn handle_batch_encrypt_directory(
    term: &Term,
    theme: &ColorfulTheme,
    _temp_file_path: Arc<Mutex<Option<PathBuf>>>,
) -> Result<()> {
    term.write_line("")?;
    term.write_line(&style("📁 批量目录加密 (Batch Directory Encryption)").bold().cyan().to_string())?;
    term.write_line(&style("─".repeat(40)).dim().to_string())?;

    // 获取目录路径
    let directory = get_directory_path_input(term, theme, "请输入要加密的目录路径 (Enter directory path to encrypt)")?;

    // 选择安全级别
    let level = select_security_level(term, theme)?;

    // 询问是否递归处理
    let recursive = Confirm::with_theme(theme)
        .with_prompt("是否递归处理子目录? (Process subdirectories recursively?)")
        .default(true)
        .interact_on(term)?;

    // 询问是否使用密钥文件
    let keyfile = get_optional_keyfile(term, theme)?;

    // 询问是否强制覆盖
    let force_overwrite = Confirm::with_theme(theme)
        .with_prompt("如果目标文件已存在，是否强制覆盖? (Force overwrite if target exists?)")
        .default(false)
        .interact_on(term)?;

    // 获取文件过滤模式
    let (include_patterns, exclude_patterns) = get_file_patterns(term, theme)?;

    // 显示批量操作预览
    display_batch_operation_preview(term, "批量加密 (Batch Encryption)", &directory, recursive, &include_patterns, &exclude_patterns)?;

    // 确认执行
    let confirm = Confirm::with_theme(theme)
        .with_prompt("确认执行批量加密操作? (Confirm batch encryption operation?)")
        .default(true)
        .interact_on(term)?;

    if !confirm {
        term.write_line(&style("操作已取消 (Operation cancelled)").yellow().to_string())?;
        return Ok(());
    }

    // 获取密码
    let mut password = rpassword::prompt_password("请输入密码 (输入时不可见): ")
        .context("无法读取密码")?;

    // 执行批量加密
    let config = BatchConfig {
        level,
        force_overwrite,
        recursive,
        include_patterns: parse_patterns(&include_patterns)?,
        exclude_patterns: parse_patterns(&exclude_patterns)?,
    };

    term.write_line("")?;
    term.write_line(&style("正在执行批量加密操作... (Executing batch encryption...)").cyan().to_string())?;

    let result = batch_encrypt_directory(&directory, &password, keyfile.as_ref(), &config)?;

    // 显示结果
    display_batch_result(term, &result, "批量加密 (Batch Encryption)")?;

    password.zeroize();
    Ok(())
}

/// 处理批量目录解密操作
fn handle_batch_decrypt_directory(
    term: &Term,
    theme: &ColorfulTheme,
    _temp_file_path: Arc<Mutex<Option<PathBuf>>>,
) -> Result<()> {
    term.write_line("")?;
    term.write_line(&style("📂 批量目录解密 (Batch Directory Decryption)").bold().cyan().to_string())?;
    term.write_line(&style("─".repeat(40)).dim().to_string())?;

    // 获取目录路径
    let directory = get_directory_path_input(term, theme, "请输入包含加密文件的目录路径 (Enter directory path containing encrypted files)")?;

    // 询问是否递归处理
    let recursive = Confirm::with_theme(theme)
        .with_prompt("是否递归处理子目录? (Process subdirectories recursively?)")
        .default(true)
        .interact_on(term)?;

    // 询问是否使用密钥文件
    let keyfile = get_optional_keyfile(term, theme)?;

    // 显示批量操作预览
    display_batch_operation_preview(term, "批量解密 (Batch Decryption)", &directory, recursive, &[], &[])?;

    // 确认执行
    let confirm = Confirm::with_theme(theme)
        .with_prompt("确认执行批量解密操作? (Confirm batch decryption operation?)")
        .default(true)
        .interact_on(term)?;

    if !confirm {
        term.write_line(&style("操作已取消 (Operation cancelled)").yellow().to_string())?;
        return Ok(());
    }

    // 获取密码
    let mut password = rpassword::prompt_password("请输入密码 (输入时不可见): ")
        .context("无法读取密码")?;

    // 执行批量解密
    let config = BatchConfig {
        recursive,
        ..Default::default()
    };

    term.write_line("")?;
    term.write_line(&style("正在执行批量解密操作... (Executing batch decryption...)").cyan().to_string())?;

    let result = batch_decrypt_directory(&directory, &password, keyfile.as_ref(), &config)?;

    // 显示结果
    display_batch_result(term, &result, "批量解密 (Batch Decryption)")?;

    password.zeroize();
    Ok(())
}

/// 处理密钥文件生成操作
fn handle_generate_keyfile(term: &Term, theme: &ColorfulTheme) -> Result<()> {
    term.write_line("")?;
    term.write_line(&style("🔑 生成密钥文件 (Generate Key File)").bold().cyan().to_string())?;
    term.write_line(&style("─".repeat(30)).dim().to_string())?;

    // 获取输出路径
    let output_path: String = Input::with_theme(theme)
        .with_prompt("请输入密钥文件保存路径 (Enter key file save path)")
        .with_initial_text("my-secret.key")
        .interact_text_on(term)?;

    let output = PathBuf::from(output_path);

    // 检查文件是否已存在
    if output.exists() {
        term.write_line(&style(format!("⚠️  文件已存在: {}", output.display())).yellow().to_string())?;
        let overwrite = Confirm::with_theme(theme)
            .with_prompt("是否覆盖现有文件? (Overwrite existing file?)")
            .default(false)
            .interact_on(term)?;

        if !overwrite {
            term.write_line(&style("操作已取消 (Operation cancelled)").yellow().to_string())?;
            return Ok(());
        }
    }

    // 显示操作预览
    term.write_line("")?;
    term.write_line(&style("📋 操作预览 (Operation Preview):").bold().to_string())?;
    term.write_line(&format!("   📁 输出路径: {}", output.display()))?;
    term.write_line(&format!("   🔐 密钥类型: 256-bit 随机密钥"))?;
    term.write_line("")?;

    // 确认执行
    let confirm = Confirm::with_theme(theme)
        .with_prompt("确认生成密钥文件? (Confirm key file generation?)")
        .default(true)
        .interact_on(term)?;

    if !confirm {
        term.write_line(&style("操作已取消 (Operation cancelled)").yellow().to_string())?;
        return Ok(());
    }

    // 生成密钥文件
    term.write_line("")?;
    term.write_line(&style("正在生成密钥文件... (Generating key file...)").cyan().to_string())?;

    let keyfile = KeyFile::generate();
    keyfile.save_to_file(&output)?;

    term.write_line(&style("✅ 密钥文件已成功生成! (Key file generated successfully!)").green().bold().to_string())?;
    term.write_line(&format!("📁 保存位置: {}", output.display()))?;
    term.write_line("")?;
    term.write_line(&style("⚠️  重要提醒 (Important Reminder):").yellow().bold().to_string())?;
    term.write_line("   • 请务必妥善保管此密钥文件")?;
    term.write_line("   • 建议制作多个备份副本")?;
    term.write_line("   • 如果丢失，使用此密钥文件加密的数据将永久无法恢复")?;
    term.write_line("   • Please keep this key file safe")?;
    term.write_line("   • Make multiple backup copies")?;
    term.write_line("   • If lost, data encrypted with this key file cannot be recovered")?;

    Ok(())
}

/// 获取文件路径输入
fn get_file_paths_input(term: &Term, theme: &ColorfulTheme, prompt: &str) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    loop {
        let path_input: String = Input::with_theme(theme)
            .with_prompt(if paths.is_empty() {
                prompt
            } else {
                "添加更多文件路径 (按回车完成) (Add more file paths, press Enter to finish)"
            })
            .allow_empty(true)
            .interact_text_on(term)?;

        if path_input.trim().is_empty() {
            if paths.is_empty() {
                term.write_line(&style("❌ 至少需要提供一个文件路径 (At least one file path is required)").red().to_string())?;
                continue;
            } else {
                break;
            }
        }

        let path = PathBuf::from(path_input.trim());

        // 验证路径
        if !path.exists() {
            term.write_line(&style(format!("⚠️  警告: 文件不存在 - {} (Warning: File does not exist)", path.display())).yellow().to_string())?;
            let continue_anyway = Confirm::with_theme(theme)
                .with_prompt("是否仍要添加此路径? (Add this path anyway?)")
                .default(false)
                .interact_on(term)?;

            if !continue_anyway {
                continue;
            }
        }

        paths.push(path);
        term.write_line(&style(format!("✅ 已添加: {} (Added)", paths.last().unwrap().display())).green().to_string())?;
    }

    Ok(paths)
}

/// 获取目录路径输入
fn get_directory_path_input(term: &Term, theme: &ColorfulTheme, prompt: &str) -> Result<PathBuf> {
    loop {
        let path_input: String = Input::with_theme(theme)
            .with_prompt(prompt)
            .interact_text_on(term)?;

        let path = PathBuf::from(path_input.trim());

        if !path.exists() {
            term.write_line(&style(format!("❌ 目录不存在: {} (Directory does not exist)", path.display())).red().to_string())?;
            continue;
        }

        if !path.is_dir() {
            term.write_line(&style(format!("❌ 路径不是目录: {} (Path is not a directory)", path.display())).red().to_string())?;
            continue;
        }

        return Ok(path);
    }
}

/// 选择安全级别
fn select_security_level(term: &Term, theme: &ColorfulTheme) -> Result<Level> {
    let levels = vec![
        ("Interactive - 快速 (19 MiB 内存)", Level::Interactive),
        ("Moderate - 推荐 (64 MiB 内存)", Level::Moderate),
        ("Paranoid - 最安全 (256 MiB 内存)", Level::Paranoid),
    ];

    let level_texts: Vec<&str> = levels.iter().map(|(text, _)| *text).collect();

    term.write_line("")?;
    let selection = Select::with_theme(theme)
        .with_prompt("选择安全级别 (Select security level)")
        .items(&level_texts)
        .default(1) // 默认选择 Moderate
        .interact_on(term)?;

    Ok(levels[selection].1)
}

/// 获取可选的密钥文件
fn get_optional_keyfile(term: &Term, theme: &ColorfulTheme) -> Result<Option<KeyFile>> {
    let use_keyfile = Confirm::with_theme(theme)
        .with_prompt("是否使用密钥文件增强安全性? (Use key file for enhanced security?)")
        .default(false)
        .interact_on(term)?;

    if !use_keyfile {
        return Ok(None);
    }

    loop {
        let keyfile_path: String = Input::with_theme(theme)
            .with_prompt("请输入密钥文件路径 (Enter key file path)")
            .interact_text_on(term)?;

        let path = PathBuf::from(keyfile_path.trim());

        if !path.exists() {
            term.write_line(&style(format!("❌ 密钥文件不存在: {} (Key file does not exist)", path.display())).red().to_string())?;
            continue;
        }

        match validate_keyfile(&path) {
            Ok(_) => {
                let keyfile = KeyFile::load_from_file(&path)?;
                term.write_line(&style("✅ 密钥文件验证成功 (Key file validated successfully)").green().to_string())?;
                return Ok(Some(keyfile));
            }
            Err(e) => {
                term.write_line(&style(format!("❌ 密钥文件验证失败: {} (Key file validation failed)", e)).red().to_string())?;
                let retry = Confirm::with_theme(theme)
                    .with_prompt("是否重试? (Retry?)")
                    .default(true)
                    .interact_on(term)?;

                if !retry {
                    return Ok(None);
                }
            }
        }
    }
}

/// 获取文件过滤模式
fn get_file_patterns(term: &Term, theme: &ColorfulTheme) -> Result<(Vec<String>, Vec<String>)> {
    let use_patterns = Confirm::with_theme(theme)
        .with_prompt("是否设置文件过滤模式? (Set file filtering patterns?)")
        .default(false)
        .interact_on(term)?;

    if !use_patterns {
        return Ok((vec![], vec![]));
    }

    // 获取包含模式
    let mut include_patterns = Vec::new();
    term.write_line("")?;
    term.write_line("📥 包含模式 (Include patterns) - 例如: *.txt, *.doc, data_*")?;

    loop {
        let pattern: String = Input::with_theme(theme)
            .with_prompt(if include_patterns.is_empty() {
                "输入包含模式 (按回车跳过) (Enter include pattern, press Enter to skip)"
            } else {
                "添加更多包含模式 (按回车完成) (Add more include patterns, press Enter to finish)"
            })
            .allow_empty(true)
            .interact_text_on(term)?;

        if pattern.trim().is_empty() {
            break;
        }

        include_patterns.push(pattern.trim().to_string());
        term.write_line(&style(format!("✅ 已添加包含模式: {} (Added include pattern)", include_patterns.last().unwrap())).green().to_string())?;
    }

    // 获取排除模式
    let mut exclude_patterns = Vec::new();
    term.write_line("")?;
    term.write_line("📤 排除模式 (Exclude patterns) - 例如: *.tmp, *.bak, temp_*")?;

    loop {
        let pattern: String = Input::with_theme(theme)
            .with_prompt(if exclude_patterns.is_empty() {
                "输入排除模式 (按回车跳过) (Enter exclude pattern, press Enter to skip)"
            } else {
                "添加更多排除模式 (按回车完成) (Add more exclude patterns, press Enter to finish)"
            })
            .allow_empty(true)
            .interact_text_on(term)?;

        if pattern.trim().is_empty() {
            break;
        }

        exclude_patterns.push(pattern.trim().to_string());
        term.write_line(&style(format!("✅ 已添加排除模式: {} (Added exclude pattern)", exclude_patterns.last().unwrap())).green().to_string())?;
    }

    Ok((include_patterns, exclude_patterns))
}

/// 显示操作预览
fn display_operation_preview(
    term: &Term,
    operation: &str,
    files: &[PathBuf],
    level: Level,
    keyfile: Option<&KeyFile>,
    force_overwrite: bool,
) -> Result<()> {
    term.write_line("")?;
    term.write_line(&style("📋 操作预览 (Operation Preview):").bold().to_string())?;
    term.write_line(&format!("   🔧 操作类型: {}", operation))?;
    term.write_line(&format!("   📁 文件数量: {} 个", files.len()))?;
    term.write_line(&format!("   🔒 安全级别: {:?}", level))?;
    term.write_line(&format!("   🔑 密钥文件: {}", if keyfile.is_some() { "是 (Yes)" } else { "否 (No)" }))?;
    term.write_line(&format!("   ⚡ 强制覆盖: {}", if force_overwrite { "是 (Yes)" } else { "否 (No)" }))?;

    if files.len() <= 5 {
        term.write_line("   📄 文件列表:")?;
        for file in files {
            term.write_line(&format!("      • {}", file.display()))?;
        }
    } else {
        term.write_line("   📄 文件列表 (前5个):")?;
        for file in files.iter().take(5) {
            term.write_line(&format!("      • {}", file.display()))?;
        }
        term.write_line(&format!("      ... 还有 {} 个文件 (and {} more files)", files.len() - 5, files.len() - 5))?;
    }

    term.write_line("")?;
    Ok(())
}

/// 显示批量操作预览
fn display_batch_operation_preview(
    term: &Term,
    operation: &str,
    directory: &Path,
    recursive: bool,
    include_patterns: &[String],
    exclude_patterns: &[String],
) -> Result<()> {
    term.write_line("")?;
    term.write_line(&style("📋 批量操作预览 (Batch Operation Preview):").bold().to_string())?;
    term.write_line(&format!("   🔧 操作类型: {}", operation))?;
    term.write_line(&format!("   📁 目标目录: {}", directory.display()))?;
    term.write_line(&format!("   🔄 递归处理: {}", if recursive { "是 (Yes)" } else { "否 (No)" }))?;

    if !include_patterns.is_empty() {
        term.write_line("   📥 包含模式:")?;
        for pattern in include_patterns {
            term.write_line(&format!("      • {}", pattern))?;
        }
    }

    if !exclude_patterns.is_empty() {
        term.write_line("   📤 排除模式:")?;
        for pattern in exclude_patterns {
            term.write_line(&format!("      • {}", pattern))?;
        }
    }

    term.write_line("")?;
    Ok(())
}

/// 显示批量操作结果
fn display_batch_result(term: &Term, result: &crate::BatchResult, operation: &str) -> Result<()> {
    term.write_line("")?;

    let total_files = result.success_count + result.failure_count;

    if result.failure_count == 0 {
        term.write_line(&style(format!("✅ {}完成! (Completed!)", operation)).green().bold().to_string())?;
        term.write_line(&format!("📊 处理统计: {} 个文件全部成功处理", total_files))?;
    } else {
        term.write_line(&style(format!("⚠️  {}完成，但有部分文件失败 (Completed with some failures)", operation)).yellow().bold().to_string())?;
        term.write_line("📊 处理统计:")?;
        term.write_line(&format!("   ✅ 成功: {} 个文件", result.success_count))?;
        term.write_line(&format!("   ❌ 失败: {} 个文件", result.failure_count))?;
        term.write_line(&format!("   📈 成功率: {:.1}%", (result.success_count as f64 / total_files as f64) * 100.0))?;

        if result.failure_count > 0 {
            term.write_line("")?;
            term.write_line(&style("💥 失败文件详情:").red().bold().to_string())?;
            for (path, error) in &result.failures {
                term.write_line(&format!("   📁 {}", path.display()))?;
                term.write_line(&format!("   🔍 错误: {}", error))?;
                term.write_line("")?;
            }
        }
    }

    if result.total_bytes > 0 {
        term.write_line(&format!("💾 处理数据量: {:.2} MB", result.total_bytes as f64 / 1_048_576.0))?;
    }

    Ok(())
}

/// 解析字符串模式为 glob::Pattern
fn parse_patterns(patterns: &[String]) -> Result<Vec<glob::Pattern>> {
    if patterns.is_empty() {
        return Ok(vec![glob::Pattern::new("*")?]);
    }

    patterns
        .iter()
        .map(|s| glob::Pattern::new(s).with_context(|| format!("无效的模式: {}", s)))
        .collect()
}
