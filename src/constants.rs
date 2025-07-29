// src/constants.rs

//! # 常量模块
//!
//! 该模块定义了整个加密工具中使用的所有核心常量。
//! 将这些值集中管理有助于保持一致性、可读性和可维护性。

// --- 文件处理常量 ---

/// 加密文件所使用的自定义文件扩展名。
/// 所有加密后的文件都将以 `.feroxcrypt` 结尾。
pub const CUSTOM_FILE_EXTENSION: &str = "feroxcrypt";

/// 用于流式读写操作的缓冲区大小。
/// 设置为 4MB 是为了在处理大文件时获得较好的 I/O 性能，同时避免过高的内存消耗。
pub const BUFFER_LEN: usize = 4 * 1024 * 1024;

// --- 密码学常量 ---

/// 使用 Argon2 从用户密码派生出的主密钥的长度（单位：字节）。
/// 这个主密钥随后会被分割成加密密钥和认证密钥。
/// 长度为 64 字节（32 字节用于 AES-256，32 字节用于 HMAC-SHA256）。
pub const MASTER_KEY_LEN: usize = 64;

/// AES-256 对称加密算法所需的密钥长度（单位：字节）。
pub const AES_KEY_LEN: usize = 32; // 256 bits

/// 用于 Argon2 密钥派生函数的盐（Salt）的长度（单位：字节）。
/// 盐是一个随机值，用于确保即使相同的密码也会生成不同的密钥，有效抵抗彩虹表攻击。
/// Argon2 官方推荐长度为 16 字节。
pub const SALT_LEN: usize = 16;

/// AES-CTR 加密模式所需的初始化向量（IV）或随机数（Nonce）的长度（单位：字节）。
/// IV 必须是唯一的，以确保相同的明文和密钥组合产生不同的密文。
pub const IV_LEN: usize = 16;

/// HMAC-SHA256 消息认证码（MAC）的标签（Tag）长度（单位：字节）。
/// 这个标签用于验证文件的完整性和真实性，防止数据被篡改。
/// SHA-256 的输出固定为 32 字节。
pub const TAG_LEN: usize = 32;

// --- 密钥文件常量 ---

/// 密钥文件的最小允许大小（单位：字节）。
/// 用于验证一个文件是否可能是有效的密钥文件。
pub const MIN_KEYFILE_SIZE: usize = 64;

/// 密钥文件的最大允许大小（单位：字节）。
/// 限制最大大小以防止意外读取非常大的文件。
pub const MAX_KEYFILE_SIZE: usize = 4 * 1024; // 4 KB

/// 用于密钥文件内容派生的盐的长度（单位：字节）。
/// 这个盐是固定的，以确保从相同的密钥文件内容总是能派生出相同的密钥材料。
pub const KEYFILE_DERIVATION_SALT: &[u8] = b"ferox-encryptor-keyfile-salt";

/// 从密钥文件内容派生出的密钥材料的长度（单位：字节）。
pub const KEYFILE_DERIVED_LEN: usize = 32;