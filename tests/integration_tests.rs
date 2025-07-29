// tests/integration_tests.rs

use anyhow::Result;
use ferox_encryptor::{run_decryption_flow, run_encryption_flow, Level};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tempfile::TempDir;

/// Helper function to create a test file with content
fn create_test_file(dir: &TempDir, filename: &str, content: &[u8]) -> Result<PathBuf> {
    let file_path = dir.path().join(filename);
    fs::write(&file_path, content)?;
    Ok(file_path)
}

#[test]
fn test_encryption_decryption_roundtrip() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_content = b"Hello, World! This is a test file for encryption.";
    let original_file = create_test_file(&temp_dir, "test.txt", test_content)?;

    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
    let password = "test_password_123";

    // Test encryption
    run_encryption_flow(
        &original_file,
        false,
        password,
        Level::Interactive,
        None, Arc::clone(&temp_file_path),
    )?;

    let encrypted_file = temp_dir.path().join("test.txt.feroxcrypt");
    assert!(encrypted_file.exists(), "Encrypted file should exist");

    // Remove original file to test decryption
    fs::remove_file(&original_file)?;

    // Test decryption
    run_decryption_flow(&encrypted_file, password, None, Arc::clone(&temp_file_path))?;

    // Verify decrypted content
    let decrypted_content = fs::read(&original_file)?;
    assert_eq!(
        decrypted_content, test_content,
        "Decrypted content should match original"
    );

    Ok(())
}

#[test]
fn test_different_security_levels() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_content = b"Security level test content";

    for level in [Level::Interactive, Level::Moderate, Level::Paranoid] {
        let filename = format!("test_{:?}.txt", level);
        let original_file = create_test_file(&temp_dir, &filename, test_content)?;

        let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
        let password = "security_test_password";

        // Encrypt with specific security level
        run_encryption_flow(
            &original_file,
            false,
            password,
            level,
            None, Arc::clone(&temp_file_path),
        )?;

        let encrypted_file = temp_dir.path().join(format!("{}.feroxcrypt", filename));
        assert!(
            encrypted_file.exists(),
            "Encrypted file should exist for {:?}",
            level
        );

        // Remove original and decrypt
        fs::remove_file(&original_file)?;
        run_decryption_flow(&encrypted_file, password, None, Arc::clone(&temp_file_path))?;

        // Verify content
        let decrypted_content = fs::read(&original_file)?;
        assert_eq!(
            decrypted_content, test_content,
            "Content mismatch for {:?}",
            level
        );
    }

    Ok(())
}

#[test]
fn test_large_file_encryption() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Create a larger test file (1MB)
    let large_content = vec![0xAB; 1024 * 1024];
    let original_file = create_test_file(&temp_dir, "large_test.bin", &large_content)?;

    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
    let password = "large_file_password";

    // Encrypt
    run_encryption_flow(
        &original_file,
        false,
        password,
        Level::Interactive,
        None, Arc::clone(&temp_file_path),
    )?;

    let encrypted_file = temp_dir.path().join("large_test.bin.feroxcrypt");
    assert!(encrypted_file.exists());

    // Verify encrypted file is larger due to headers and MAC
    let encrypted_size = fs::metadata(&encrypted_file)?.len();
    assert!(encrypted_size > large_content.len() as u64);

    // Decrypt
    fs::remove_file(&original_file)?;
    run_decryption_flow(&encrypted_file, password, None, Arc::clone(&temp_file_path))?;

    // Verify content
    let decrypted_content = fs::read(&original_file)?;
    assert_eq!(decrypted_content, large_content);

    Ok(())
}

#[test]
fn test_unicode_filename() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_content = b"Unicode filename test";
    let unicode_filename = "测试文件_🔒.txt";
    let original_file = create_test_file(&temp_dir, unicode_filename, test_content)?;

    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
    let password = "unicode_test";

    // Encrypt
    run_encryption_flow(
        &original_file,
        false,
        password,
        Level::Interactive,
        None, Arc::clone(&temp_file_path),
    )?;

    let encrypted_file = temp_dir
        .path()
        .join(format!("{}.feroxcrypt", unicode_filename));
    assert!(encrypted_file.exists());

    // Decrypt
    fs::remove_file(&original_file)?;
    run_decryption_flow(&encrypted_file, password, None, Arc::clone(&temp_file_path))?;

    // Verify
    let decrypted_content = fs::read(&original_file)?;
    assert_eq!(decrypted_content, test_content);

    Ok(())
}
