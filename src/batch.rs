// src/batch.rs

//! # 批量处理模块
//!
//! 该模块提供了对多个文件进行加密或解密的功能。
//! 它支持目录的递归遍历、按模式包含/排除文件，并能报告详细的处理结果。

use crate::{decrypt::run_decryption_flow, encrypt::run_encryption_flow, keyfile::KeyFile, Level};
use anyhow::Result;
use glob::Pattern;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;

/// # 批量操作配置
///
/// 定义了批量处理任务的所有可配置参数。
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// 加密时使用的安全级别。
    pub level: Level,
    /// 是否强制覆盖已存在的目标文件。
    pub force_overwrite: bool,
    /// 是否递归处理子目录。
    pub recursive: bool,
    /// 用于包含文件的 glob 模式列表 (例如, "*.txt", "data_*.csv")。
    pub include_patterns: Vec<Pattern>,
    /// 用于排除文件的 glob 模式列表。
    pub exclude_patterns: Vec<Pattern>,
}

impl Default for BatchConfig {
    /// 提供一个默认的 `BatchConfig` 实例。
    fn default() -> Self {
        Self {
            level: Level::Moderate,
            force_overwrite: false,
            recursive: false,
            // 默认包含所有文件
            include_patterns: vec![Pattern::new("*").unwrap()],
            exclude_patterns: Vec::new(),
        }
    }
}

/// # 批量操作结果
///
/// 存储批量处理任务完成后的统计信息。
#[derive(Debug)]
pub struct BatchResult {
    /// 成功处理的文件数量。
    pub success_count: usize,
    /// 处理失败的文件数量。
    pub failure_count: usize,
    /// 失败文件的列表，包含文件路径和具体的错误信息。
    pub failures: Vec<(PathBuf, String)>,
    /// 成功处理的总字节数。
    pub total_bytes: u64,
}

impl BatchResult {
    /// 创建一个新的、空的 `BatchResult`。
    fn new() -> Self {
        Self {
            success_count: 0,
            failure_count: 0,
            failures: Vec::new(),
            total_bytes: 0,
        }
    }

    /// 记录一次成功操作。
    fn add_success(&mut self, file_size: u64) {
        self.success_count += 1;
        self.total_bytes += file_size;
    }

    /// 记录一次失败操作。
    fn add_failure(&mut self, path: PathBuf, error: String) {
        self.failure_count += 1;
        self.failures.push((path, error));
    }
}

/// 批量加密指定目录中的文件。
pub fn batch_encrypt_directory(
    directory: &Path,
    password: &str,
    keyfile: Option<&KeyFile>,
    config: &BatchConfig,
) -> Result<BatchResult> {
    // 首先，收集所有符合条件的文件
    let files = collect_files(directory, config, false)?;
    // 然后，对收集到的文件列表执行加密
    batch_encrypt_files(&files, password, keyfile, config)
}

/// 批量加密一个具体的文件列表。
pub fn batch_encrypt_files(
    files: &[PathBuf],
    password: &str,
    keyfile: Option<&KeyFile>,
    config: &BatchConfig,
) -> Result<BatchResult> {
    let mut result = BatchResult::new();
    // 创建用于中断清理的共享状态
    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));

    log::info!("开始批量加密 {} 个文件...", files.len());

    for (index, file_path) in files.iter().enumerate() {
        log::info!(
            "正在处理文件 {}/{}: {}",
            index + 1,
            files.len(),
            file_path.display()
        );

        // 对每个文件调用单独的加密处理函数
        match process_single_encryption(
            file_path,
            password,
            keyfile,
            config,
            Arc::clone(&temp_file_path),
        ) {
            Ok(file_size) => {
                result.add_success(file_size);
                log::info!("✅ 成功加密: {}", file_path.display());
            }
            Err(e) => {
                let error_msg = format!("{e:#}");
                result.add_failure(file_path.clone(), error_msg.clone());
                log::error!("❌ 加密失败 {}: {}", file_path.display(), error_msg);
            }
        }
    }

    log::info!(
        "批量加密完成: {} 个成功, {} 个失败。",
        result.success_count,
        result.failure_count
    );

    Ok(result)
}

/// 批量解密指定目录中的文件。
pub fn batch_decrypt_directory(
    directory: &Path,
    password: &str,
    keyfile: Option<&KeyFile>,
    config: &BatchConfig,
) -> Result<BatchResult> {
    // 收集所有符合条件的已加密文件
    let files = collect_files(directory, config, true)?;
    // 对收集到的文件列表执行解密
    batch_decrypt_files(&files, password, keyfile)
}

/// 批量解密一个具体的已加密文件列表。
pub fn batch_decrypt_files(
    files: &[PathBuf],
    password: &str,
    keyfile: Option<&KeyFile>,
) -> Result<BatchResult> {
    let mut result = BatchResult::new();
    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));

    log::info!("开始批量解密 {} 个文件...", files.len());

    for (index, file_path) in files.iter().enumerate() {
        log::info!(
            "正在处理文件 {}/{}: {}",
            index + 1,
            files.len(),
            file_path.display()
        );

        // 对每个文件调用单独的解密处理函数
        match process_single_decryption(file_path, password, keyfile, Arc::clone(&temp_file_path)) {
            Ok(file_size) => {
                result.add_success(file_size);
                log::info!("✅ 成功解密: {}", file_path.display());
            }
            Err(e) => {
                let error_msg = format!("{e:#}");
                result.add_failure(file_path.clone(), error_msg.clone());
                log::error!("❌ 解密失败 {}: {}", file_path.display(), error_msg);
            }
        }
    }

    log::info!(
        "批量解密完成: {} 个成功, {} 个失败。",
        result.success_count,
        result.failure_count
    );

    Ok(result)
}

/// 处理单个文件的加密。
fn process_single_encryption(
    file_path: &Path,
    password: &str,
    keyfile: Option<&KeyFile>,
    config: &BatchConfig,
    temp_file_path: Arc<Mutex<Option<PathBuf>>>,
) -> Result<u64> {
    let file_size = fs::metadata(file_path)?.len();

    run_encryption_flow(
        file_path,
        config.force_overwrite,
        password,
        config.level,
        keyfile,
        temp_file_path,
    )?;

    Ok(file_size)
}

/// 处理单个文件的解密。
fn process_single_decryption(
    file_path: &Path,
    password: &str,
    keyfile: Option<&KeyFile>,
    temp_file_path: Arc<Mutex<Option<PathBuf>>>,
) -> Result<u64> {
    let file_size = fs::metadata(file_path)?.len();

    run_decryption_flow(file_path, password, keyfile, temp_file_path)?;

    Ok(file_size)
}

/// 收集目录下符合条件的文件。
///
/// # 参数
///
/// * `directory` - 要搜索的根目录。
/// * `config` - 批量操作配置，用于判断是否递归以及如何过滤。
/// * `encrypted_only` - `true` 表示只收集已加密文件，`false` 表示只收集未加密文件。
///
/// # 返回
///
/// 一个包含所有符合条件的文件路径的向量。
fn collect_files(
    directory: &Path,
    config: &BatchConfig,
    encrypted_only: bool,
) -> Result<Vec<PathBuf>> {
    if !directory.is_dir() {
        anyhow::bail!("提供的路径不是一个目录: {}", directory.display());
    }

    // 使用 walkdir 库来遍历文件，它能很好地处理递归和非递归的情况
    let walker = WalkDir::new(directory).max_depth(if config.recursive { usize::MAX } else { 1 });

    let files: Vec<PathBuf> = walker
        .into_iter()
        .filter_map(Result::ok) // 忽略读取目录中的错误
        .filter(|e| e.file_type().is_file()) // 只关心文件
        .map(|e| e.path().to_path_buf())
        .filter(|path| should_include_file(path, config, encrypted_only)) // 应用过滤规则
        .collect();

    Ok(files)
}

/// 判断一个文件是否应该被包含在批量处理中。
fn should_include_file(path: &Path, config: &BatchConfig, encrypted_only: bool) -> bool {
    // 根据 `encrypted_only` 标志，判断文件是否具有正确的加密状态
    let is_encrypted = path
        .extension()
        .is_some_and(|ext| ext == crate::constants::CUSTOM_FILE_EXTENSION);

    if encrypted_only && !is_encrypted {
        return false; // 需要已加密文件，但当前文件未加密
    }
    if !encrypted_only && is_encrypted {
        return false; // 需要未加密文件，但当前文件已加密
    }

    // 检查文件名是否匹配任何一个 `include` 模式
    let matches_include = config
        .include_patterns
        .iter()
        .any(|pattern| pattern.matches_path(path));

    if !matches_include {
        return false; // 不匹配任何 `include` 模式
    }

    // 检查文件名是否匹配任何一个 `exclude` 模式
    let matches_exclude = config
        .exclude_patterns
        .iter()
        .any(|pattern| pattern.matches_path(path));

    !matches_exclude // 如果不匹配任何 `exclude` 模式，则最终决定包含该文件
}
