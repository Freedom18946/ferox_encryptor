# Ferox Encryptor API Documentation

## Overview

Ferox Encryptor provides a simple yet powerful API for encrypting and decrypting files with strong security guarantees.

## Core Functions

### `run_encryption_flow`

Encrypts a file with the specified security level and password.

```rust
pub fn run_encryption_flow(
    source_path: &Path,
    force_overwrite: bool,
    password: &str,
    level: Level,
    temp_file_path: Arc<Mutex<Option<PathBuf>>>,
) -> Result<()>
```

**Parameters:**
- `source_path`: Path to the file to encrypt
- `force_overwrite`: Whether to overwrite existing encrypted files
- `password`: Password for encryption
- `level`: Security level (Interactive, Moderate, or Paranoid)
- `temp_file_path`: Shared state for cleanup on interruption

**Returns:** `Ok(())` on success, or an error describing what went wrong.

**Output:** Creates a new file with `.feroxcrypt` extension containing the encrypted data.

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