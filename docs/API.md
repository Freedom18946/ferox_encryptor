# Ferox Encryptor API 文档 (API Documentation)

**Ferox Encryptor 库的完整 API 参考文档**

*Comprehensive API reference documentation for the Ferox Encryptor library*

## 概览 (Overview)

Ferox Encryptor 提供了一个简单而强大的 API，用于加密和解密文件，并提供强大的安全保证。

*Ferox Encryptor provides a simple yet powerful API for encrypting and decrypting files with strong security guarantees.*

## 核心函数 (Core Functions)

### `run_encryption_flow` - 文件加密函数

使用指定的安全级别和密码加密文件。

*Encrypts a file with the specified security level and password.*

```rust
pub fn run_encryption_flow(
    source_path: &Path,           // 源文件路径 (Source file path)
    force_overwrite: bool,        // 是否强制覆盖 (Force overwrite flag)
    password: &str,               // 加密密码 (Encryption password)
    level: Level,                 // 安全级别 (Security level)
    keyfile: Option<&KeyFile>,    // 可选密钥文件 (Optional keyfile)
    temp_file_path: Arc<Mutex<Option<PathBuf>>>, // 临时文件路径 (Temp file path)
) -> Result<()>
```

**参数说明 (Parameters):**
- `source_path`: 要加密的文件路径 (Path to the file to encrypt)
- `force_overwrite`: 是否覆盖已存在的加密文件 (Whether to overwrite existing encrypted files)
- `password`: 用于加密的密码 (Password for encryption)
- `level`: 安全级别 (Interactive, Moderate, 或 Paranoid) (Security level: Interactive, Moderate, or Paranoid)
- `keyfile`: 可选的密钥文件，用于增强安全性 (Optional keyfile for enhanced security)
- `temp_file_path`: 用于中断时清理的共享状态 (Shared state for cleanup on interruption)

**返回值 (Returns):** 成功时返回 `Ok(())`，失败时返回描述错误的信息。
*Returns `Ok(())` on success, or an error describing what went wrong.*

**输出 (Output):** 创建一个带有 `.feroxcrypt` 扩展名的新文件，包含加密数据。
*Creates a new file with `.feroxcrypt` extension containing the encrypted data.*

### `run_decryption_flow`

Decrypts a `.feroxcrypt` file back to its original form.

```rust
pub fn run_decryption_flow(
    source_path: &Path,
    password: &str,
    temp_file_path: Arc<Mutex<Option<PathBuf>>>,
) -> Result<()>
```

**Parameters:**
- `source_path`: Path to the encrypted `.feroxcrypt` file
- `password`: Password used for decryption
- `temp_file_path`: Shared state for cleanup on interruption

**Returns:** `Ok(())` on success, or an error describing what went wrong.

**Output:** Creates the original file in the same directory as the encrypted file.

## Security Levels

### `Level` Enum

Defines different security levels with corresponding Argon2 computation costs:

```rust
pub enum Level {
    Interactive,  // m=19MiB, t=2 - Fastest
    Moderate,     // m=64MiB, t=3 - Recommended default
    Paranoid,     // m=256MiB, t=4 - Maximum security
}
```

Each level provides:
- **Interactive**: Best for frequently accessed files, development/testing
- **Moderate**: Best for most use cases, personal documents, sensitive data
- **Paranoid**: Best for highly sensitive data, long-term storage, maximum security

### `Level::argon2_params()`

Returns the Argon2 parameters for a security level:

```rust
pub fn argon2_params(&self) -> (u32, u32, u32)
```

**Returns:** A tuple of `(memory_cost_kb, time_cost, parallelism)`

## File Format

Encrypted files use the `.feroxcrypt` extension with the following structure:

```
[filename_length(2 bytes)] + 
[original_filename] + 
[salt(16 bytes)] + 
[iv(16 bytes)] + 
[argon2_params(12 bytes)] + 
[encrypted_data] + 
[hmac_tag(32 bytes)]
```

## Error Handling

All functions return `anyhow::Result<()>` for comprehensive error handling. Common error scenarios:

- **File not found**: Source file doesn't exist
- **Permission denied**: Insufficient permissions to read/write files
- **Already encrypted**: Attempting to encrypt a `.feroxcrypt` file
- **Invalid format**: Attempting to decrypt a non-encrypted file
- **Authentication failure**: Wrong password or corrupted file
- **Disk space**: Insufficient space for output file

## Example Usage

### Basic Encryption/Decryption

```rust
use ferox_encryptor::{run_encryption_flow, run_decryption_flow, Level};
use std::path::Path;
use std::sync::{Arc, Mutex};

fn main() -> anyhow::Result<()> {
    let temp_file_path = Arc::new(Mutex::new(None));
    
    // Encrypt a file
    run_encryption_flow(
        Path::new("document.txt"),
        false, // don't force overwrite
        "my_secure_password",
        Level::Moderate,
        Arc::clone(&temp_file_path)
    )?;
    
    // Decrypt the file
    run_decryption_flow(
        Path::new("document.txt.feroxcrypt"),
        "my_secure_password",
        temp_file_path
    )?;
    
    Ok(())
}
```

### Handling Different Security Levels

```rust
use ferox_encryptor::{run_encryption_flow, Level};

// For maximum security (slower)
run_encryption_flow(
    Path::new("sensitive.txt"),
    false,
    "strong_password",
    Level::Paranoid,
    temp_file_path
)?;

// For development/testing (faster)
run_encryption_flow(
    Path::new("test.txt"),
    false,
    "test_password",
    Level::Interactive,
    temp_file_path
)?;
```

## Thread Safety

The API is designed to be thread-safe:
- All functions accept `Arc<Mutex<Option<PathBuf>>>` for shared state management
- Internal cryptographic operations use thread-safe primitives
- Multiple encryption/decryption operations can run concurrently

## Performance Considerations

- **Memory Usage**: ~4MB buffer for streaming operations
- **CPU Usage**: Varies by security level (Interactive < Moderate < Paranoid)
- **I/O Patterns**: Optimized for large files with sequential access
- **Progress Tracking**: Built-in progress bars for long operations

## Security Guarantees

- **Key Derivation**: Argon2id with configurable parameters
- **Encryption**: AES-256-CTR mode for confidentiality
- **Authentication**: HMAC-SHA256 for integrity (Encrypt-then-MAC)
- **Salt/IV**: Cryptographically secure random generation
- **Memory Safety**: Automatic zeroization of sensitive data