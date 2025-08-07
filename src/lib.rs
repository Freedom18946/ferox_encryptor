// src/lib.rs

//! # Ferox Encryptor Crate
//!
//! 本项目是一个基于 Rust 的高性能、抗暴力破解的本地文件加密工具库。
//!
//! *This project is a high-performance, brute-force resistant local file encryption tool library built with Rust.*
//!
//! ## 主要特性 (Key Features)
//!
//! - **极致安全 (Ultimate Security)**: 采用 `Argon2id` 进行密钥派生，有效抵御 GPU 和 ASIC 破解。
//!   *Uses `Argon2id` for key derivation, effectively resisting GPU and ASIC attacks.*
//! - **现代加密 (Modern Encryption)**: 使用 `AES-256-CTR` 进行流式加密，并结合 `HMAC-SHA256` (Encrypt-then-MAC 模式) 保证数据的机密性和完整性。
//!   *Uses `AES-256-CTR` for streaming encryption, combined with `HMAC-SHA256` (Encrypt-then-MAC mode) to ensure data confidentiality and integrity.*
//! - **高性能 (High Performance)**: 为处理大型文件（数GB）而优化，采用流式处理，内存占用极低。
//!   *Optimized for processing large files (multi-GB), using streaming processing with minimal memory footprint.*
//! - **向后兼容 (Backward Compatibility)**: 加密参数存储在文件头中，确保未来的版本能解密旧文件。
//!   *Encryption parameters stored in file headers, ensuring future versions can decrypt old files.*
//! - **可定制的安全等级 (Customizable Security Levels)**: 提供多个安全级别选项，允许用户在性能和安全性之间进行权衡。
//!   *Provides multiple security level options, allowing users to balance performance and security.*
//! - **健壮性 (Robustness)**: 能够优雅地处理 `Ctrl+C` 中断，自动清理不完整的输出文件。
//!   *Gracefully handles `Ctrl+C` interrupts, automatically cleaning up incomplete output files.*
//!
//! ## 加密文件格式
//!
//! 加密文件使用 `.feroxcrypt` 扩展名，其内部结构如下:
//!
//! ```text
//! +-------------------------+-----------------------+------------------+----------------+-----------------------+--------------------+-------------------+
//! | 文件名长度 (2字节)      | 原始文件名 (可变)     | Salt (16字节)    | IV (16字节)    | Argon2 参数 (12字节) | 加密数据 (可变)    | HMAC 标签 (32字节) |
//! +-------------------------+-----------------------+------------------+----------------+-----------------------+--------------------+-------------------+
//! ```
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use ferox_encryptor::{encrypt::run_encryption_flow, decrypt::run_decryption_flow, Level, keyfile::KeyFile};
//! use std::path::Path;
//! use std::sync::{Arc, Mutex};
//!
//! # fn main() -> anyhow::Result<()> {
//! let temp_file_path = Arc::new(Mutex::new(None));
//! let password = "my_secure_password";
//!
//! // 加密文件
//! run_encryption_flow(
//!     Path::new("document.txt"),
//!     false, // 不强制覆盖
//!     password,
//!     Level::Moderate,
//!     None, // 不使用密钥文件
//!     Arc::clone(&temp_file_path)
//! )?;
//!
//! // 解密文件
//! run_decryption_flow(
//!     Path::new("document.txt.feroxcrypt"),
//!     password,
//!     None, // 不使用密钥文件
//!     temp_file_path
//! )?;
//! # Ok(())
//! # }
//! ```

// 声明本 crate 的模块
pub mod batch;
pub mod constants;
pub mod decrypt;
pub mod encrypt;
pub mod interactive;
pub mod keyfile;

// 从子模块中重新导出公共类型，方便外部调用者使用。
// 例如，外部可以直接使用 `ferox_encryptor::Level` 而不是 `ferox_encryptor::lib::Level`。
pub use batch::{
    batch_decrypt_directory, batch_decrypt_files, batch_encrypt_directory, batch_encrypt_files,
    BatchConfig, BatchResult,
};
pub use decrypt::run_decryption_flow;
pub use encrypt::run_encryption_flow;
pub use keyfile::{validate_keyfile, KeyFile};

/// # 安全级别 (Security Levels)
///
/// 定义了不同的安全级别，对应不同的 Argon2 计算成本。
/// 每个级别代表了在安全性和性能之间的不同权衡：
/// - 更高的级别能更好地防御暴力破解攻击。
/// - 更低的级别加密/解密速度更快，但安全性相应降低。
///
/// *Defines different security levels corresponding to different Argon2 computational costs.*
/// *Each level represents a different trade-off between security and performance:*
/// *- Higher levels provide better protection against brute-force attacks.*
/// *- Lower levels encrypt/decrypt faster but with correspondingly reduced security.*
#[derive(clap::ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Level {
    /// **交互式 (Interactive)**: 速度最快，适用于普通文件。
    /// - Argon2 参数 (Parameters): m_cost=19MiB, t_cost=2, p_cost=1
    /// - **最佳用途 (Best Use Cases)**: 需要频繁访问的文件、开发和测试环境。
    ///   *Files that need frequent access, development and testing environments.*
    Interactive,
    /// **中等 (Moderate)**: 推荐的默认级别，在安全性和性能之间有很好的平衡。
    /// - Argon2 参数 (Parameters): m_cost=64MiB, t_cost=3, p_cost=1
    /// - **最佳用途 (Best Use Cases)**: 大多数使用场景，如个人文档、敏感数据。
    ///   *Most use cases, such as personal documents and sensitive data.*
    Moderate,
    /// **偏执 (Paranoid)**: 极高的安全性，但速度显著减慢。
    /// - Argon2 参数 (Parameters): m_cost=256MiB, t_cost=4, p_cost=1
    /// - **最佳用途 (Best Use Cases)**: 高度敏感的数据、长期存储、需要最高安全保障的场景。
    ///   *Highly sensitive data, long-term storage, scenarios requiring maximum security.*
    Paranoid,
}

impl Level {
    /// 返回此安全级别对应的 Argon2 参数 (Returns Argon2 parameters for this security level)
    ///
    /// # 返回 (Returns)
    ///
    /// 一个元组，包含 (内存成本(KB), 时间成本(迭代次数), 并行度)。
    ///
    /// *A tuple containing (memory cost in KB, time cost in iterations, parallelism).*
    ///
    /// # 参数说明 (Parameter Explanation)
    ///
    /// - **内存成本 (Memory Cost)**: Argon2 算法使用的内存量，越大越安全但消耗更多内存
    ///   *Amount of memory used by Argon2 algorithm, larger values are more secure but consume more memory*
    /// - **时间成本 (Time Cost)**: 算法的迭代次数，越多越安全但计算时间更长
    ///   *Number of iterations of the algorithm, more iterations are more secure but take longer to compute*
    /// - **并行度 (Parallelism)**: 并行线程数，当前设置为1以保持一致性
    ///   *Number of parallel threads, currently set to 1 for consistency*
    pub fn argon2_params(&self) -> (u32, u32, u32) {
        match self {
            Level::Interactive => (19 * 1024, 2, 1), // 19 MiB, 2 次迭代 (19 MiB, 2 iterations)
            Level::Moderate => (64 * 1024, 3, 1),    // 64 MiB, 3 次迭代 (64 MiB, 3 iterations)
            Level::Paranoid => (256 * 1024, 4, 1),   // 256 MiB, 4 次迭代 (256 MiB, 4 iterations)
        }
    }
}

// --- 集成测试 ---
#[cfg(test)]
mod tests {
    use super::{decrypt, encrypt, keyfile::KeyFile, Level};
    use anyhow::Result;
    use std::fs;
    use std::sync::{Arc, Mutex};
    use tempfile::TempDir;

    /// 端到端测试：加密一个文件，然后解密，并验证内容是否一致。
    #[test]
    fn test_e2e_encryption_decryption() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let test_content = b"Hello, World! This is a test.";
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, test_content)?;

        let temp_file_path = Arc::new(Mutex::new(None));
        let password = "test_password";

        // 加密
        encrypt::run_encryption_flow(
            &test_file,
            false,
            password,
            Level::Interactive,
            None,
            Arc::clone(&temp_file_path),
        )?;

        let encrypted_file = temp_dir.path().join("test.txt.feroxcrypt");
        assert!(encrypted_file.exists());

        // 删除原文件并解密
        fs::remove_file(&test_file)?;
        decrypt::run_decryption_flow(&encrypted_file, password, None, temp_file_path)?;

        // 验证解密后的内容与原始内容相同
        let decrypted_content = fs::read(&test_file)?;
        assert_eq!(decrypted_content, test_content);

        Ok(())
    }

    /// 使用密钥文件进行端到端测试。
    #[test]
    fn test_e2e_with_keyfile() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let test_content = b"This is a test with a keyfile.";
        let test_file = temp_dir.path().join("test_kf.txt");
        fs::write(&test_file, test_content)?;

        // 生成并保存密钥文件
        let keyfile_path = temp_dir.path().join("test.key");
        let keyfile = KeyFile::generate();
        keyfile.save_to_file(&keyfile_path)?;

        let temp_file_path = Arc::new(Mutex::new(None));
        let password = "keyfile_password";

        // 使用密钥文件加密
        encrypt::run_encryption_flow(
            &test_file,
            false,
            password,
            Level::Interactive,
            Some(&keyfile),
            Arc::clone(&temp_file_path),
        )?;

        let encrypted_file = temp_dir.path().join("test_kf.txt.feroxcrypt");
        assert!(encrypted_file.exists());

        // 删除原文件并使用密钥文件解密
        fs::remove_file(&test_file)?;
        decrypt::run_decryption_flow(&encrypted_file, password, Some(&keyfile), temp_file_path)?;

        // 验证内容
        let decrypted_content = fs::read(&test_file)?;
        assert_eq!(decrypted_content, test_content);

        Ok(())
    }
}
