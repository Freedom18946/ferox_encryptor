// src/encrypt.rs

//! # 加密流程模块
//!
//! 该模块负责执行文件的加密操作。它包含了从用户输入验证、
//! 密钥派生、文件读写到生成最终加密文件的完整逻辑。

use crate::{
    constants::{
        AES_KEY_LEN, BUFFER_LEN, CUSTOM_FILE_EXTENSION, IV_LEN, MASTER_KEY_LEN, SALT_LEN,
    },
    keyfile::{combine_password_and_keyfile, KeyFile},
    Level,
};
use anyhow::{anyhow, bail, Context, Result};
use argon2::{self, Argon2, Params};
use ctr::cipher::{KeyIvInit, StreamCipher};
use hmac::{Hmac, Mac};
use indicatif::{ProgressBar, ProgressStyle};
use rand::{rngs::OsRng, RngCore};
use sha2::Sha256;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use zeroize::Zeroize;

// 定义密码学算法的类型别名，以简化代码
type Aes256Ctr = ctr::Ctr128BE<aes::Aes256>;
type HmacSha256 = Hmac<Sha256>;

/// 执行完整的文件加密流程。
///
/// # 参数
///
/// * `source_path` - 要加密的源文件的路径。
/// * `force_overwrite` - 是否强制覆盖已存在的同名加密文件。
/// * `password` - 用于加密的密码。
/// * `level` - 加密的安全级别，决定了 Argon2 的计算成本。
/// * `keyfile` - (可选) 用于增强安全性的密钥文件。
/// * `temp_file_path` - 一个线程安全的共享变量，用于在程序被中断（如 Ctrl+C）时记录临时文件名，以便清理。
///
/// # 返回
///
/// `Ok(())` 表示成功，否则返回一个描述错误的 `anyhow::Error`。
pub fn run_encryption_flow(
    source_path: &Path,
    force_overwrite: bool,
    password: &str,
    level: Level,
    keyfile: Option<&KeyFile>,
    temp_file_path: Arc<Mutex<Option<PathBuf>>>,
) -> Result<()> {
    // 将核心逻辑包装在一个闭包中，这样可以利用 `?` 操作符进行错误处理，
    // 并在闭包外部统一处理清理逻辑，实现类似 `try...finally` 的效果。
    let result = (|| {
        // --- 1. 输入验证 ---
        if !source_path.exists() {
            bail!("文件不存在: {}", source_path.display());
        }
        if !source_path.is_file() {
            bail!("提供的路径不是一个文件: {}", source_path.display());
        }
        // 检查文件是否已经加密
        if source_path
            .extension()
            .is_some_and(|s| s == CUSTOM_FILE_EXTENSION)
        {
            bail!(
                "文件看起来已经被加密过了 (以 .{} 结尾)",
                CUSTOM_FILE_EXTENSION
            );
        }

        // --- 2. 准备路径和文件名 ---
        let original_filename = source_path
            .file_name()
            .context("无法获取文件名")?
            .to_str()
            .context("文件名包含无效的UTF-8字符")?;

        // 构建目标加密文件的路径
        let target_path_str = format!(
            "{}.{}",
            source_path.display(),
            CUSTOM_FILE_EXTENSION
        );
        let target_path = Path::new(&target_path_str).to_path_buf();

        // 如果目标文件已存在且未设置强制覆盖，则报错
        if !force_overwrite && target_path.exists() {
            bail!(
                "目标文件 {} 已存在。如需覆盖，请使用 --force 标志。",
                target_path.display()
            );
        }

        log::info!("加密后的文件将保存为: {}", target_path.display());
        log::info!("使用 {:?} 安全级别进行加密", level);

        // 在开始写入前，将目标路径存入共享状态，以便中断时可以清理
        *temp_file_path.lock().unwrap() = Some(target_path.clone());

        // --- 3. 打开文件流 ---
        let source_file = File::open(source_path).context("无法打开源文件")?;
        let source_size = source_file.metadata()?.len();
        let mut reader = BufReader::with_capacity(BUFFER_LEN, source_file);
        let target_file = File::create(&target_path).context("无法创建目标文件")?;
        let mut writer = BufWriter::with_capacity(BUFFER_LEN, target_file);

        // --- 4. 生成密码学参数 ---
        // 生成随机的盐和初始化向量 (IV)
        let mut salt = [0u8; SALT_LEN];
        OsRng.fill_bytes(&mut salt);
        let mut iv = [0u8; IV_LEN];
        OsRng.fill_bytes(&mut iv);

        // --- 5. 密钥派生 ---
        log::info!("正在从密码派生密钥...");
        // 根据选择的安全级别获取 Argon2 参数
        let (m_cost, t_cost, p_cost) = level.argon2_params();
        let argon2_params = Params::new(m_cost, t_cost, p_cost, Some(MASTER_KEY_LEN))
            .map_err(|e| anyhow!("创建 Argon2 参数失败: {}", e))?;
        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2_params,
        );

        // 根据是否有密钥文件，选择不同的密码材料
        let mut password_material = if let Some(kf) = keyfile {
            log::info!("使用密钥文件增强安全性。");
            combine_password_and_keyfile(password, kf)?
        } else {
            password.as_bytes().to_vec()
        };

        // 使用 Argon2 进行密钥派生
        let mut master_key = [0u8; MASTER_KEY_LEN];
        argon2
            .hash_password_into(&password_material, &salt, &mut master_key)
            .map_err(|e| anyhow!("Argon2密钥派生失败: {}", e))?;
        
        // 安全地擦除内存中的密码材料
        password_material.zeroize();
        log::info!("密钥派生完成。");

        // --- 6. 分割主密钥并初始化加密器和 MAC ---
        // 主密钥的前半部分用于 AES 加密，后半部分用于 HMAC 认证
        let (aes_key, hmac_key) = master_key.split_at(AES_KEY_LEN);
        let mut cipher = Aes256Ctr::new(aes_key.into(), &iv.into());
        let mut mac =
            HmacSha256::new_from_slice(hmac_key).context("无法创建HMAC实例")?;

        // --- 7. 写入文件头 ---
        // 文件头包含了恢复原始文件名和进行解密所需的所有元数据。
        // 顺序: 文件名长度 -> 文件名 -> 盐 -> IV -> Argon2参数
        let filename_bytes = original_filename.as_bytes();
        if filename_bytes.len() > u16::MAX as usize {
            bail!("文件名太长了 (超过65535字节)");
        }
        // 写入原始文件名的长度 (2字节, 小端序)
        writer.write_all(&(filename_bytes.len() as u16).to_le_bytes())?;
        // 写入原始文件名
        writer.write_all(filename_bytes)?;
        // 写入盐
        writer.write_all(&salt)?;
        // 写入IV
        writer.write_all(&iv)?;
        // 写入 Argon2 参数 (m_cost, t_cost, p_cost)，共12字节
        writer.write_all(&m_cost.to_le_bytes())?;
        writer.write_all(&t_cost.to_le_bytes())?;
        writer.write_all(&p_cost.to_le_bytes())?;

        // --- 8. 流式加密和认证 ---
        log::info!("开始流式加密文件...");
        // 初始化进度条
        let pb = ProgressBar::new(source_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"));

        let mut buffer = vec![0u8; BUFFER_LEN];
        loop {
            // 从源文件读取数据块
            let bytes_read = reader.read(&mut buffer).context("读取源文件失败")?;
            if bytes_read == 0 {
                break; // 文件读取完毕
            }
            pb.inc(bytes_read as u64);
            let chunk = &mut buffer[..bytes_read];
            
            // Encrypt-then-MAC 模式:
            // 1. 加密数据块
            cipher.apply_keystream(chunk);
            // 2. 将加密后的数据块（密文）送入 HMAC 进行认证
            mac.update(chunk);
            // 3. 将加密后的数据块写入目标文件
            writer.write_all(chunk).context("写入目标文件失败")?;
        }

        // --- 9. 写入认证标签并完成 ---
        // 在所有数据都处理完毕后，生成最终的 HMAC 认证标签
        let tag = mac.finalize().into_bytes();
        // 将标签写入文件的末尾
        writer.write_all(&tag)?;
        // 确保所有缓冲数据都已写入磁盘
        writer.flush().context("刷新文件缓冲区失败")?;
        pb.finish_with_message("加密完成");

        log::info!("--- ✅ 加密成功! ---");
        // 安全地擦除内存中的主密钥
        master_key.zeroize();
        Ok(())
    })();

    // 无论成功或失败，都在函数返回前清理共享状态
    *temp_file_path.lock().unwrap() = None;

    result
}