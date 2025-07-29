// src/decrypt.rs

//! # 解密流程模块
//!
//! 该模块负责执行文件的解密操作。它精确地逆转了加密流程：
//! 读取文件头 -> 派生密钥 -> 流式解密和认证 -> 最终验证。

use crate::{
    constants::{
        AES_KEY_LEN, BUFFER_LEN, CUSTOM_FILE_EXTENSION, IV_LEN, MASTER_KEY_LEN, SALT_LEN,
        TAG_LEN,
    },
    keyfile::{combine_password_and_keyfile, KeyFile},
};
use anyhow::{anyhow, bail, Context, Result};
use argon2::{self, Argon2, Params};
use ctr::cipher::{KeyIvInit, StreamCipher};
use hmac::{Hmac, Mac};
use indicatif::{ProgressBar, ProgressStyle};
use sha2::Sha256;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use zeroize::Zeroize;

// 定义密码学算法的类型别名
type Aes256Ctr = ctr::Ctr128BE<aes::Aes256>;
type HmacSha256 = Hmac<Sha256>;

/// 执行完整的文件解密流程。
///
/// # 参数
///
/// * `source_path` - 要解密的 `.feroxcrypt` 文件的路径。
/// * `password` - 用于解密的密码。
/// * `keyfile` - (可选) 用于解密的密钥文件。
/// * `temp_file_path` - 线程安全的共享变量，用于在中断时记录临时文件名以供清理。
///
/// # 返回
///
/// `Ok(())` 表示成功，否则返回一个描述错误的 `anyhow::Error`。
pub fn run_decryption_flow(
    source_path: &Path,
    password: &str,
    keyfile: Option<&KeyFile>,
    temp_file_path: Arc<Mutex<Option<PathBuf>>>,
) -> Result<()> {
    // 同样使用闭包来包裹核心逻辑，以便统一处理清理操作
    let result = (|| {
        // --- 1. 输入验证 ---
        if !source_path.exists() {
            bail!("文件不存在: {}", source_path.display());
        }
        if !source_path.is_file() {
            bail!("提供的路径不是一个文件: {}", source_path.display());
        }
        // 验证文件扩展名是否正确
        if source_path
            .extension()
            .is_none_or(|s| s != CUSTOM_FILE_EXTENSION)
        {
            bail!(
                "文件看起来不是一个有效的加密文件 (必须以 .{} 结尾)",
                CUSTOM_FILE_EXTENSION
            );
        }

        // --- 2. 打开文件并读取文件头 ---
        let source_file = File::open(source_path).context("无法打开源文件")?;
        let file_size = source_file.metadata()?.len();
        let mut reader = BufReader::with_capacity(BUFFER_LEN, source_file);

        // 读取原始文件名的长度 (2字节)
        let mut filename_len_bytes = [0u8; 2];
        reader
            .read_exact(&mut filename_len_bytes)
            .context("无法读取文件名长度")?;
        let filename_len = u16::from_le_bytes(filename_len_bytes) as usize;

        // 读取原始文件名
        let mut filename_bytes = vec![0u8; filename_len];
        reader
            .read_exact(&mut filename_bytes)
            .context("无法读取文件名")?;
        let original_filename =
            String::from_utf8(filename_bytes).context("文件名包含无效的UTF-8字符")?;

        // --- 3. 准备目标路径 ---
        let parent_dir = source_path.parent().context("无法获取父目录")?;
        let target_path = parent_dir.join(original_filename);

        // 防止意外覆盖现有文件
        if target_path.exists() {
            bail!("目标文件 {} 已存在，为防止数据覆盖，操作已中止。", target_path.display());
        }
        log::info!("解密后的文件将保存为: {}", target_path.display());

        // 在开始写入前，将目标路径存入共享状态
        *temp_file_path.lock().unwrap() = Some(target_path.clone());

        // --- 4. 读取密码学元数据 ---
        // 必须严格按照加密时写入的顺序来读取
        let mut salt = [0u8; SALT_LEN];
        reader.read_exact(&mut salt).context("无法读取Salt")?;
        let mut iv = [0u8; IV_LEN];
        reader.read_exact(&mut iv).context("无法读取IV")?;

        // 读取 Argon2 参数
        let mut m_cost_bytes = [0u8; 4];
        let mut t_cost_bytes = [0u8; 4];
        let mut p_cost_bytes = [0u8; 4];
        reader
            .read_exact(&mut m_cost_bytes)
            .context("无法读取 Argon2 m_cost")?;
        reader
            .read_exact(&mut t_cost_bytes)
            .context("无法读取 Argon2 t_cost")?;
        reader
            .read_exact(&mut p_cost_bytes)
            .context("无法读取 Argon2 p_cost")?;

        let m_cost = u32::from_le_bytes(m_cost_bytes);
        let t_cost = u32::from_le_bytes(t_cost_bytes);
        let p_cost = u32::from_le_bytes(p_cost_bytes);

        // 使用从文件头读取的参数重新构建 Argon2 配置
        let argon2_params = Params::new(m_cost, t_cost, p_cost, Some(MASTER_KEY_LEN))
            .map_err(|e| anyhow!("从文件头创建 Argon2 参数失败: {}", e))?;
        
        log::info!("文件使用的 Argon2 参数: m_cost={}, t_cost={}, p_cost={}", m_cost, t_cost, p_cost);

        // --- 5. 密钥派生 ---
        log::info!("正在从密码派生密钥...");
        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2_params,
        );

        // 根据是否有密钥文件，准备密码材料
        let mut password_material = if let Some(kf) = keyfile {
            log::info!("使用密钥文件进行解密。");
            combine_password_and_keyfile(password, kf)?
        } else {
            password.as_bytes().to_vec()
        };

        // 使用与加密时完全相同的参数（密码材料、盐）来派生密钥
        let mut master_key = [0u8; MASTER_KEY_LEN];
        argon2
            .hash_password_into(&password_material, &salt, &mut master_key)
            .map_err(|e| anyhow!("Argon2密钥派生失败: {}", e))?;
        
        password_material.zeroize();
        log::info!("密钥派生完成。");

        // --- 6. 初始化加密器和 MAC ---
        let (aes_key, hmac_key) = master_key.split_at(AES_KEY_LEN);
        let mut cipher = Aes256Ctr::new(aes_key.into(), &iv.into());
        let mut mac =
            HmacSha256::new_from_slice(hmac_key).context("无法创建HMAC实例")?;

        // --- 7. 计算密文大小并准备流式解密 ---
        let header_size = (2 + filename_len + SALT_LEN + IV_LEN + 12) as u64; // +12 for Argon2 params
        let ciphertext_size = file_size - header_size - TAG_LEN as u64;

        log::info!("开始流式解密文件...");
        let target_file = File::create(&target_path).context("无法创建目标文件")?;
        let mut writer = BufWriter::with_capacity(BUFFER_LEN, target_file);

        // 初始化进度条
        let pb = ProgressBar::new(ciphertext_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"));

        // 使用 `take` 方法精确地只读取密文部分，不包括最后的认证标签
        let mut ciphertext_reader = reader.take(ciphertext_size);
        let mut buffer = vec![0u8; BUFFER_LEN];
        loop {
            let bytes_read = ciphertext_reader
                .read(&mut buffer)
                .context("读取密文失败")?;
            if bytes_read == 0 {
                break;
            }
            pb.inc(bytes_read as u64);
            let chunk = &mut buffer[..bytes_read];
            
            // MAC-then-Decrypt 模式的逆过程:
            // 1. 将从文件读取的密文块送入 HMAC 进行认证
            mac.update(chunk);
            // 2. 解密数据块 (AES-CTR 的加解密是同一个操作)
            cipher.apply_keystream(chunk);
            // 3. 将解密后的明文写入目标文件
            writer.write_all(chunk).context("写入目标文件失败")?;
        }

        pb.finish_with_message("解密完成，正在验证...");

        // --- 8. 验证认证标签 ---
        // 读取文件末尾原始的认证标签
        let mut original_tag = [0u8; TAG_LEN];
        ciphertext_reader
            .into_inner() // 获取 `take` 装饰器内部的 reader
            .read_exact(&mut original_tag)
            .context("无法读取文件的认证标签")?;

        // 将我们实时计算出的 HMAC 标签与文件中的原始标签进行比较
        // 这是一个常数时间比较，可以防止时序攻击
        match mac.verify_slice(&original_tag) {
            Ok(_) => {
                // 验证成功，刷新缓冲区，完成写入
                writer.flush().context("刷新文件缓冲区失败")?;
                log::info!("--- ✅ 验证成功，解密完成! ---");
            }
            Err(_) => {
                // 验证失败，立即报错并中止。
                // 这通常意味着密码错误、密钥文件错误或文件已损坏。
                bail!("严重错误: 认证失败! 文件可能已损坏，或密码/密钥文件错误。");
            }
        }

        // 安全擦除主密钥
        master_key.zeroize();
        Ok(())
    })();

    // 无论成功或失败，都清理共享状态
    *temp_file_path.lock().unwrap() = None;

    result
}