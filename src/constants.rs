// src/constants.rs

//! # 常量模块 (Constants Module)
//!
//! 该模块定义了整个加密工具中使用的所有核心常量。
//! 将这些值集中管理有助于保持一致性、可读性和可维护性。
//!
//! *This module defines all core constants used throughout the encryption tool.*
//! *Centralizing these values helps maintain consistency, readability, and maintainability.*

// --- 文件处理常量 (File Processing Constants) ---

/// 加密文件所使用的自定义文件扩展名 (Custom file extension for encrypted files)
///
/// 所有加密后的文件都将以 `.feroxcrypt` 结尾。
/// 这个扩展名用于标识文件已被 Ferox Encryptor 加密，
/// 并且在批量处理时用于自动识别加密文件。
///
/// *All encrypted files will end with `.feroxcrypt`.*
/// *This extension identifies files encrypted by Ferox Encryptor*
/// *and is used for automatic recognition during batch processing.*
pub const CUSTOM_FILE_EXTENSION: &str = "feroxcrypt";

/// 用于流式读写操作的缓冲区大小 (Buffer size for streaming read/write operations)
///
/// 设置为 4MB 是为了在处理大文件时获得较好的 I/O 性能，同时避免过高的内存消耗。
/// 这个大小经过测试，在大多数系统上能够提供良好的性能平衡。
///
/// *Set to 4MB to achieve good I/O performance when processing large files*
/// *while avoiding excessive memory consumption.*
/// *This size has been tested to provide good performance balance on most systems.*
pub const BUFFER_LEN: usize = 4 * 1024 * 1024;

// --- 密码学常量 (Cryptographic Constants) ---

/// 使用 Argon2 从用户密码派生出的主密钥的长度（单位：字节）(Master key length derived from user password using Argon2)
///
/// 这个主密钥随后会被分割成加密密钥和认证密钥。
/// 长度为 64 字节（32 字节用于 AES-256，32 字节用于 HMAC-SHA256）。
///
/// *This master key is subsequently split into encryption key and authentication key.*
/// *Length is 64 bytes (32 bytes for AES-256, 32 bytes for HMAC-SHA256).*
pub const MASTER_KEY_LEN: usize = 64;

/// AES-256 对称加密算法所需的密钥长度（单位：字节）(Key length required for AES-256 symmetric encryption algorithm)
///
/// AES-256 使用 256 位（32 字节）的密钥长度，提供高强度的加密保护。
///
/// *AES-256 uses a 256-bit (32-byte) key length, providing high-strength encryption protection.*
pub const AES_KEY_LEN: usize = 32; // 256 bits

/// 用于 Argon2 密钥派生函数的盐（Salt）的长度（单位：字节）(Salt length for Argon2 key derivation function)
///
/// 盐是一个随机值，用于确保即使相同的密码也会生成不同的密钥，有效抵抗彩虹表攻击。
/// Argon2 官方推荐长度为 16 字节。
///
/// *Salt is a random value used to ensure that even identical passwords generate different keys,*
/// *effectively resisting rainbow table attacks. Argon2 officially recommends 16 bytes.*
pub const SALT_LEN: usize = 16;

/// AES-CTR 加密模式所需的初始化向量（IV）或随机数（Nonce）的长度（单位：字节）(IV/Nonce length for AES-CTR encryption mode)
///
/// IV 必须是唯一的，以确保相同的明文和密钥组合产生不同的密文。
/// 对于 AES-CTR 模式，IV 长度为 16 字节。
///
/// *IV must be unique to ensure that identical plaintext and key combinations produce different ciphertext.*
/// *For AES-CTR mode, IV length is 16 bytes.*
pub const IV_LEN: usize = 16;

/// HMAC-SHA256 消息认证码（MAC）的标签（Tag）长度（单位：字节）(HMAC-SHA256 message authentication code tag length)
///
/// 这个标签用于验证文件的完整性和真实性，防止数据被篡改。
/// SHA-256 的输出固定为 32 字节。
///
/// *This tag is used to verify file integrity and authenticity, preventing data tampering.*
/// *SHA-256 output is fixed at 32 bytes.*
pub const TAG_LEN: usize = 32;

// --- 密钥文件常量 (Keyfile Constants) ---

/// 密钥文件的最小允许大小（单位：字节）(Minimum allowed size for keyfiles)
///
/// 用于验证一个文件是否可能是有效的密钥文件。
/// 太小的文件无法提供足够的熵来保证安全性。
///
/// *Used to verify if a file could be a valid keyfile.*
/// *Files that are too small cannot provide sufficient entropy to ensure security.*
pub const MIN_KEYFILE_SIZE: usize = 64;

/// 密钥文件的最大允许大小（单位：字节）(Maximum allowed size for keyfiles)
///
/// 限制最大大小以防止意外读取非常大的文件。
/// 4KB 的大小足以提供高强度的安全性，同时保持合理的文件大小。
///
/// *Limits maximum size to prevent accidentally reading very large files.*
/// *4KB size is sufficient to provide high-strength security while maintaining reasonable file size.*
pub const MAX_KEYFILE_SIZE: usize = 4 * 1024; // 4 KB

/// 用于密钥文件内容派生的盐（单位：字节）(Salt for keyfile content derivation)
///
/// 这个盐是固定的，以确保从相同的密钥文件内容总是能派生出相同的密钥材料。
/// 使用固定盐确保了密钥文件的确定性行为。
///
/// *This salt is fixed to ensure that identical keyfile content always derives the same key material.*
/// *Using a fixed salt ensures deterministic behavior of keyfiles.*
pub const KEYFILE_DERIVATION_SALT: &[u8] = b"ferox-encryptor-keyfile-salt";

/// 从密钥文件内容派生出的密钥材料的长度（单位：字节）(Length of key material derived from keyfile content)
///
/// 32 字节提供了 256 位的安全强度，与 AES-256 的密钥长度相匹配。
///
/// *32 bytes provides 256-bit security strength, matching AES-256 key length.*
pub const KEYFILE_DERIVED_LEN: usize = 32;
