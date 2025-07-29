// tests/batch_tests.rs

//! Tests for batch processing functionality

use anyhow::Result;
use ferox_encryptor::{batch_decrypt_directory, batch_encrypt_directory, BatchConfig, Level};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_batch_encrypt_decrypt_basic() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let password = "batch_test_password";

    // Create test files
    let files = [
        ("file1.txt", "Content of file 1"),
        ("file2.txt", "Content of file 2"),
        ("file3.txt", "Content of file 3"),
    ];

    for (filename, content) in &files {
        let file_path = temp_dir.path().join(filename);
        fs::write(&file_path, content.as_bytes())?;
    }

    let config = BatchConfig::default();

    // Test batch encryption
    let encrypt_result = batch_encrypt_directory(temp_dir.path(), password, &config)?;
    assert_eq!(encrypt_result.success_count, 3);
    assert_eq!(encrypt_result.failure_count, 0);

    // Verify encrypted files exist
    for (filename, _) in &files {
        let encrypted_path = temp_dir.path().join(format!("{}.feroxcrypt", filename));
        assert!(encrypted_path.exists(), "Encrypted file should exist: {}", filename);
    }

    // Remove original files
    for (filename, _) in &files {
        let file_path = temp_dir.path().join(filename);
        fs::remove_file(&file_path)?;
    }

    // Test batch decryption
    let decrypt_result = batch_decrypt_directory(temp_dir.path(), password, &config)?;
    assert_eq!(decrypt_result.success_count, 3);
    assert_eq!(decrypt_result.failure_count, 0);

    // Verify original files are restored with correct content
    for (filename, expected_content) in &files {
        let file_path = temp_dir.path().join(filename);
        assert!(file_path.exists(), "Decrypted file should exist: {}", filename);
        let actual_content = fs::read_to_string(&file_path)?;
        assert_eq!(&actual_content, expected_content);
    }

    Ok(())
}

#[test]
fn test_batch_with_subdirectories() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let password = "recursive_test_password";

    // Create directory structure
    let subdir1 = temp_dir.path().join("subdir1");
    let subdir2 = temp_dir.path().join("subdir2");
    fs::create_dir(&subdir1)?;
    fs::create_dir(&subdir2)?;

    // Create files in different directories
    let files = [
        ("root.txt", "Root file content"),
        ("subdir1/sub1.txt", "Subdir1 file content"),
        ("subdir2/sub2.txt", "Subdir2 file content"),
    ];

    for (filepath, content) in &files {
        let full_path = temp_dir.path().join(filepath);
        fs::write(&full_path, content.as_bytes())?;
    }

    // Test recursive encryption
    let recursive_config = BatchConfig {
        recursive: true,
        ..Default::default()
    };

    let encrypt_result = batch_encrypt_directory(temp_dir.path(), password, &recursive_config)?;
    assert_eq!(encrypt_result.success_count, 3);
    assert_eq!(encrypt_result.failure_count, 0);

    // Remove original files
    for (filepath, _) in &files {
        let full_path = temp_dir.path().join(filepath);
        fs::remove_file(&full_path)?;
    }

    // Test recursive decryption
    let decrypt_result = batch_decrypt_directory(temp_dir.path(), password, &recursive_config)?;
    assert_eq!(decrypt_result.success_count, 3);
    assert_eq!(decrypt_result.failure_count, 0);

    // Verify all files are restored
    for (filepath, expected_content) in &files {
        let full_path = temp_dir.path().join(filepath);
        assert!(full_path.exists(), "File should exist: {}", filepath);
        let actual_content = fs::read_to_string(&full_path)?;
        assert_eq!(&actual_content, expected_content);
    }

    Ok(())
}

#[test]
fn test_batch_with_file_patterns() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let password = "pattern_test_password";

    // Create files with different extensions
    let files = [
        ("document1.txt", "Text document 1"),
        ("document2.txt", "Text document 2"),
        ("image1.jpg", "Image data 1"),
        ("image2.png", "Image data 2"),
        ("data.csv", "CSV data"),
    ];

    for (filename, content) in &files {
        let file_path = temp_dir.path().join(filename);
        fs::write(&file_path, content.as_bytes())?;
    }

    // Test with include pattern for only .txt files
    let txt_config = BatchConfig {
        include_patterns: vec!["*.txt".to_string()],
        ..Default::default()
    };

    let encrypt_result = batch_encrypt_directory(temp_dir.path(), password, &txt_config)?;
    assert_eq!(encrypt_result.success_count, 2); // Only 2 .txt files

    // Verify only .txt files were encrypted
    assert!(temp_dir.path().join("document1.txt.feroxcrypt").exists());
    assert!(temp_dir.path().join("document2.txt.feroxcrypt").exists());
    assert!(!temp_dir.path().join("image1.jpg.feroxcrypt").exists());
    assert!(!temp_dir.path().join("image2.png.feroxcrypt").exists());
    assert!(!temp_dir.path().join("data.csv.feroxcrypt").exists());

    Ok(())
}

#[test]
fn test_batch_with_exclude_patterns() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let password = "exclude_test_password";

    // Create files
    let files = [
        ("keep1.txt", "Keep this file"),
        ("keep2.doc", "Keep this document"),
        ("temp1.tmp", "Temporary file 1"),
        ("temp2.tmp", "Temporary file 2"),
        ("backup.bak", "Backup file"),
    ];

    for (filename, content) in &files {
        let file_path = temp_dir.path().join(filename);
        fs::write(&file_path, content.as_bytes())?;
    }

    // Test with exclude patterns for temporary and backup files
    let exclude_config = BatchConfig {
        exclude_patterns: vec!["*.tmp".to_string(), "*.bak".to_string()],
        ..Default::default()
    };

    let encrypt_result = batch_encrypt_directory(temp_dir.path(), password, &exclude_config)?;
    assert_eq!(encrypt_result.success_count, 2); // Only keep1.txt and keep2.doc

    // Verify correct files were encrypted
    assert!(temp_dir.path().join("keep1.txt.feroxcrypt").exists());
    assert!(temp_dir.path().join("keep2.doc.feroxcrypt").exists());
    assert!(!temp_dir.path().join("temp1.tmp.feroxcrypt").exists());
    assert!(!temp_dir.path().join("temp2.tmp.feroxcrypt").exists());
    assert!(!temp_dir.path().join("backup.bak.feroxcrypt").exists());

    Ok(())
}

#[test]
fn test_batch_force_overwrite() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let password = "overwrite_test_password";

    // Create a test file
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, b"Original content")?;

    let config = BatchConfig::default();

    // First encryption
    let encrypt_result1 = batch_encrypt_directory(temp_dir.path(), password, &config)?;
    assert_eq!(encrypt_result1.success_count, 1);

    // Modify original file
    fs::write(&test_file, b"Modified content")?;

    // Try to encrypt again without force - should fail
    let encrypt_result2 = batch_encrypt_directory(temp_dir.path(), password, &config)?;
    assert_eq!(encrypt_result2.failure_count, 1);

    // Try with force overwrite - should succeed
    let force_config = BatchConfig {
        force_overwrite: true,
        ..Default::default()
    };

    let encrypt_result3 = batch_encrypt_directory(temp_dir.path(), password, &force_config)?;
    assert_eq!(encrypt_result3.success_count, 1);

    Ok(())
}

#[test]
fn test_batch_different_security_levels() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let password = "security_level_batch_test";

    for level in [Level::Interactive, Level::Moderate, Level::Paranoid] {
        let level_dir = temp_dir.path().join(format!("{:?}", level));
        fs::create_dir(&level_dir)?;

        // Create test file for this level
        let test_file = level_dir.join("test.txt");
        fs::write(&test_file, format!("Content for {:?} level", level).as_bytes())?;

        let config = BatchConfig {
            level,
            ..Default::default()
        };

        // Encrypt with specific level
        let encrypt_result = batch_encrypt_directory(&level_dir, password, &config)?;
        assert_eq!(encrypt_result.success_count, 1);

        // Verify encrypted file exists
        let encrypted_file = level_dir.join("test.txt.feroxcrypt");
        assert!(encrypted_file.exists());

        // Decrypt and verify
        fs::remove_file(&test_file)?;
        let decrypt_result = batch_decrypt_directory(&level_dir, password, &config)?;
        assert_eq!(decrypt_result.success_count, 1);

        let decrypted_content = fs::read_to_string(&test_file)?;
        assert_eq!(decrypted_content, format!("Content for {:?} level", level));
    }

    Ok(())
}

#[test]
fn test_batch_error_handling() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let password = "error_handling_test";

    // Create some valid files and some problematic scenarios
    fs::write(temp_dir.path().join("valid1.txt"), b"Valid content 1")?;
    fs::write(temp_dir.path().join("valid2.txt"), b"Valid content 2")?;

    // Create a file that's already encrypted (should be skipped)
    fs::write(temp_dir.path().join("already.txt.feroxcrypt"), b"Fake encrypted content")?;

    let config = BatchConfig::default();

    // Batch encrypt - should process valid files and skip encrypted one
    let encrypt_result = batch_encrypt_directory(temp_dir.path(), password, &config)?;
    assert_eq!(encrypt_result.success_count, 2); // Only valid1.txt and valid2.txt
    assert_eq!(encrypt_result.failure_count, 0); // .feroxcrypt files are filtered out, not failed

    Ok(())
}

#[test]
fn test_batch_empty_directory() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let password = "empty_dir_test";

    let config = BatchConfig::default();

    // Test batch operations on empty directory
    let encrypt_result = batch_encrypt_directory(temp_dir.path(), password, &config)?;
    assert_eq!(encrypt_result.success_count, 0);
    assert_eq!(encrypt_result.failure_count, 0);

    let decrypt_result = batch_decrypt_directory(temp_dir.path(), password, &config)?;
    assert_eq!(decrypt_result.success_count, 0);
    assert_eq!(decrypt_result.failure_count, 0);

    Ok(())
}