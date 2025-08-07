// src/keyfile.rs

//! # 密钥文件支持模块
//!
//! 该模块提供了用于增强安全性的密钥文件功能。
//! 密钥文件是包含随机数据的文件，可以与用户密码结合使用，
//! 提供双重保护。即使密码泄露，没有对应的密钥文件，数据也无法被解密。

use crate::constants::{
    KEYFILE_DERIVATION_SALT, KEYFILE_DERIVED_LEN, MAX_KEYFILE_SIZE, MIN_KEYFILE_SIZE,
};
use anyhow::{bail, Context, Result};
use argon2::{self, Argon2};
use rand::{rngs::OsRng, RngCore};
use std::fs;
use std::path::Path;
use zeroize::Zeroize;

/// 定义 `KeyFile` 结构体，用于处理密钥文件的生成、加载和保存。
pub struct KeyFile {
    /// 存储密钥文件内容的字节向量。
    data: Vec<u8>,
}

impl KeyFile {
    /// 生成一个新的随机密钥文件。
    ///
    /// # 返回
    ///
    /// 一个包含密码学安全随机数据的 `KeyFile` 新实例。
    pub fn generate() -> Self {
        // 创建一个用零填充的字节向量
        let mut data = vec![0u8; MAX_KEYFILE_SIZE];
        // 使用操作系统提供的密码学安全随机数生成器填充向量
        OsRng.fill_bytes(&mut data);
        Self { data }
    }

    /// 从磁盘加载密钥文件。
    ///
    /// # 参数
    ///
    /// * `path` - 指向密钥文件的路径。
    ///
    /// # 返回
    ///
    /// 一个从指定路径加载的 `KeyFile` 实例。
    ///
    /// # 错误
    ///
    /// 如果文件无法读取或大小不符合要求，则返回错误。
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        // 读取文件内容
        let data =
            fs::read(path).with_context(|| format!("无法读取密钥文件: {}", path.display()))?;

        // 验证文件大小是否在允许范围内
        if data.len() < MIN_KEYFILE_SIZE || data.len() > MAX_KEYFILE_SIZE {
            bail!(
                "密钥文件大小无效: {} 字节 (必须在 {} 和 {} 字节之间)",
                data.len(),
                MIN_KEYFILE_SIZE,
                MAX_KEYFILE_SIZE
            );
        }

        Ok(Self { data })
    }

    /// 将密钥文件保存到磁盘。
    ///
    /// # 参数
    ///
    /// * `path` - 保存密钥文件的路径。
    ///
    /// # 错误
    ///
    /// 如果文件无法写入，则返回错误。
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        fs::write(path, &self.data)
            .with_context(|| format!("无法写入密钥文件: {}", path.display()))?;

        log::info!("密钥文件已保存: {}", path.display());
        Ok(())
    }

    /// 获取密钥文件内容的哈希值，用于验证。
    ///
    /// # 返回
    ///
    /// 密钥文件数据的 SHA-256 哈希值。
    pub fn hash(&self) -> [u8; 32] {
        let mut output = [0u8; KEYFILE_DERIVED_LEN];
        argon2_config()
            .hash_password_into(&self.data, KEYFILE_DERIVATION_SALT, &mut output)
            .unwrap();
        output
    }
}

/// 实现 `Drop` trait，以在 `KeyFile` 实例离开作用域时安全地擦除其内存中的数据。
/// 这是为了防止敏感数据（密钥材料）残留在内存中。
impl Drop for KeyFile {
    fn drop(&mut self) {
        // `zeroize` 会用零覆盖 `data` 向量的内容
        self.data.zeroize();
    }
}

/// 将用户密码和密钥文件结合起来，生成用于最终密钥派生的材料。
/// 这种方法增强了安全性，因为攻击者需要同时获得密码和密钥文件才能破解加密。
///
/// # 参数
///
/// * `password` - 用户的密码。
/// * `keyfile` - `KeyFile` 实例。
///
/// # 返回
///
/// 结合了密码和密钥文件信息的字节向量，将用作 Argon2 的输入。
pub fn combine_password_and_keyfile(password: &str, keyfile: &KeyFile) -> Result<Vec<u8>> {
    // 使用 Argon2 从密钥文件内容派生出一个哈希值
    let keyfile_hash = keyfile.hash();

    // 使用 Argon2 将密码和密钥文件的哈希值结合起来
    let mut combined_hash = vec![0u8; KEYFILE_DERIVED_LEN];
    argon2_config()
        .hash_password_into(
            password.as_bytes(),
            &keyfile_hash, // 使用密钥文件的哈希作为盐
            &mut combined_hash,
        )
        .map_err(|e| anyhow::anyhow!("Argon2 error: {}", e))?;

    Ok(combined_hash)
}

/// 验证一个文件是否可以用作密钥文件。
///
/// # 参数
///
/// * `path` - 指向潜在密钥文件的路径。
///
/// # 返回
///
/// 如果文件有效则返回 `Ok(())`，否则返回错误。
pub fn validate_keyfile<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();

    // 检查文件是否存在
    if !path.exists() {
        bail!("密钥文件不存在: {}", path.display());
    }

    // 检查路径是否指向一个文件而不是目录
    if !path.is_file() {
        bail!("密钥文件路径不是一个文件: {}", path.display());
    }

    // 获取文件元数据以检查大小
    let metadata = fs::metadata(path)
        .with_context(|| format!("无法读取密钥文件元数据: {}", path.display()))?;

    let file_size = metadata.len();
    if file_size < MIN_KEYFILE_SIZE as u64 || file_size > MAX_KEYFILE_SIZE as u64 {
        bail!(
            "密钥文件大小无效: {} 字节 (必须在 {} 和 {} 字节之间)",
            file_size,
            MIN_KEYFILE_SIZE,
            MAX_KEYFILE_SIZE
        );
    }

    Ok(())
}

/// 返回一个静态的 Argon2 配置实例。
/// 这确保了在整个程序中都使用一致的、预设的 Argon2 参数来处理密钥文件。
fn argon2_config() -> Argon2<'static> {
    // 使用 Argon2id 算法, v19 版本
    // 参数: m_cost=19MiB, t_cost=2, p_cost=1, output_len=32
    Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(19 * 1024, 2, 1, Some(KEYFILE_DERIVED_LEN)).unwrap(),
    )
}
