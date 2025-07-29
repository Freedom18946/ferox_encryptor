// tests/security_tests.rs

//! Security-focused tests for Ferox Encryptor

use anyhow::Result;
use ferox_encryptor::{run_decryption_flow, run_encryption_flow, Level};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tempfile::TempDir;

#[test]
fn test_wrong_password_fails() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_content = b"Secret content that should not be readable with wrong password";
    let test_file = temp_dir.path().join("secret.txt");
    fs::write(&test_file, test_content)?;

    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
    let correct_password = "correct_password_123";
    let wrong_password = "wrong_password_456";

    // Encrypt with correct password
    run_encryption_flow(
        &test_file,
        false,
        correct_password,
        Level::Interactive,
        Arc::clone(&temp_file_path),
    )?;

    let encrypted_file = temp_dir.path().join("secret.txt.feroxcrypt");
    assert!(encrypted_file.exists());

    // Remove original file
    fs::remove_file(&test_file)?;

    // Try to decrypt with wrong password - should fail
    let decrypt_result = run_decryption_flow(
        &encrypted_file,
        wrong_password,
        Arc::clone(&temp_file_path),
    );

    assert!(decrypt_result.is_err(), "Decryption with wrong password should fail");
    
    // Verify the error is authentication-related
    let error_msg = format!("{}", decrypt_result.unwrap_err());
    assert!(error_msg.contains("Authentication failed") || error_msg.contains("CRITICAL ERROR"));

    // Verify original file was not created
    assert!(!test_file.exists(), "Original file should not be created with wrong password");

    Ok(())
}

#[test]
fn test_corrupted_file_detection() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_content = b"Content to be corrupted";
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, test_content)?;

    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
    let password = "test_password";

    // Encrypt file
    run_encryption_flow(
        &test_file,
        false,
        password,
        Level::Interactive,
        Arc::clone(&temp_file_path),
    )?;

    let encrypted_file = temp_dir.path().join("test.txt.feroxcrypt");
    
    // Corrupt the encrypted file by modifying some bytes
    let mut encrypted_data = fs::read(&encrypted_file)?;
    let corruption_pos = encrypted_data.len() / 2;
    encrypted_data[corruption_pos] ^= 0xFF; // Flip all bits in one byte
    fs::write(&encrypted_file, &encrypted_data)?;

    // Remove original file
    fs::remove_file(&test_file)?;

    // Try to decrypt corrupted file - should fail
    let decrypt_result = run_decryption_flow(
        &encrypted_file,
        password,
        Arc::clone(&temp_file_path),
    );

    assert!(decrypt_result.is_err(), "Decryption of corrupted file should fail");
    
    // Verify the error is authentication-related
    let error_msg = format!("{}", decrypt_result.unwrap_err());
    assert!(error_msg.contains("Authentication failed") || error_msg.contains("CRITICAL ERROR"));

    Ok(())
}

#[test]
fn test_different_security_levels_compatibility() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_content = b"Security level compatibility test";
    let password = "test_password";

    for level in [Level::Interactive, Level::Moderate, Level::Paranoid] {
        let test_file = temp_dir.path().join(format!("test_{:?}.txt", level));
        fs::write(&test_file, test_content)?;

        let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));

        // Encrypt with specific level
        run_encryption_flow(
            &test_file,
            false,
            password,
            level,
            Arc::clone(&temp_file_path),
        )?;

        let encrypted_file = temp_dir.path().join(format!("test_{:?}.txt.feroxcrypt", level));
        assert!(encrypted_file.exists());

        // Remove original and decrypt
        fs::remove_file(&test_file)?;
        run_decryption_flow(&encrypted_file, password, temp_file_path)?;

        // Verify content is correctly decrypted
        let decrypted_content = fs::read(&test_file)?;
        assert_eq!(decrypted_content, test_content);
    }

    Ok(())
}

#[test]
fn test_empty_file_encryption() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_file = temp_dir.path().join("empty.txt");
    fs::write(&test_file, b"")?; // Empty file

    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
    let password = "test_password";

    // Encrypt empty file
    run_encryption_flow(
        &test_file,
        false,
        password,
        Level::Interactive,
        Arc::clone(&temp_file_path),
    )?;

    let encrypted_file = temp_dir.path().join("empty.txt.feroxcrypt");
    assert!(encrypted_file.exists());

    // Verify encrypted file is larger than 0 (due to headers and MAC)
    let encrypted_size = fs::metadata(&encrypted_file)?.len();
    assert!(encrypted_size > 0, "Encrypted empty file should have headers and MAC");

    // Decrypt and verify
    fs::remove_file(&test_file)?;
    run_decryption_flow(&encrypted_file, password, temp_file_path)?;

    let decrypted_content = fs::read(&test_file)?;
    assert_eq!(decrypted_content, b"", "Decrypted empty file should be empty");

    Ok(())
}

#[test]
fn test_very_long_filename() -> Result<()> {
    let temp_dir = TempDir::new()?;
    
    // Create a file with a very long name (but within limits)
    let long_name = "a".repeat(200) + ".txt";
    let test_file = temp_dir.path().join(&long_name);
    fs::write(&test_file, b"Content with long filename")?;

    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
    let password = "test_password";

    // Encrypt file with long name
    run_encryption_flow(
        &test_file,
        false,
        password,
        Level::Interactive,
        Arc::clone(&temp_file_path),
    )?;

    let encrypted_file = temp_dir.path().join(format!("{}.feroxcrypt", long_name));
    assert!(encrypted_file.exists());

    // Decrypt and verify filename is preserved
    fs::remove_file(&test_file)?;
    run_decryption_flow(&encrypted_file, password, temp_file_path)?;

    assert!(test_file.exists());
    let decrypted_content = fs::read(&test_file)?;
    assert_eq!(decrypted_content, b"Content with long filename");

    Ok(())
}

#[test]
fn test_special_characters_in_filename() -> Result<()> {
    let temp_dir = TempDir::new()?;
    
    // Test various special characters that are valid in filenames
    let special_names = vec![
        "file with spaces.txt",
        "file-with-dashes.txt",
        "file_with_underscores.txt",
        "file.with.dots.txt",
        "file(with)parentheses.txt",
        "file[with]brackets.txt",
    ];

    let password = "test_password";

    for name in special_names {
        let test_file = temp_dir.path().join(name);
        fs::write(&test_file, format!("Content for {}", name).as_bytes())?;

        let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));

        // Encrypt
        run_encryption_flow(
            &test_file,
            false,
            password,
            Level::Interactive,
            Arc::clone(&temp_file_path),
        )?;

        let encrypted_file = temp_dir.path().join(format!("{}.feroxcrypt", name));
        assert!(encrypted_file.exists(), "Encrypted file should exist for {}", name);

        // Decrypt
        fs::remove_file(&test_file)?;
        run_decryption_flow(&encrypted_file, password, temp_file_path)?;

        assert!(test_file.exists(), "Decrypted file should exist for {}", name);
        let decrypted_content = fs::read_to_string(&test_file)?;
        assert_eq!(decrypted_content, format!("Content for {}", name));
    }

    Ok(())
}

#[test]
fn test_concurrent_operations() -> Result<()> {
    use std::thread;

    let temp_dir = TempDir::new()?;
    let password = "concurrent_test_password";
    
    // Create multiple test files
    let mut handles = vec![];
    
    for i in 0..5 {
        let temp_dir_path = temp_dir.path().to_path_buf();
        let password = password.to_string();
        
        let handle = thread::spawn(move || -> Result<()> {
            let test_file = temp_dir_path.join(format!("concurrent_test_{}.txt", i));
            fs::write(&test_file, format!("Concurrent test content {}", i).as_bytes())?;

            let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));

            // Encrypt
            run_encryption_flow(
                &test_file,
                false,
                &password,
                Level::Interactive,
                Arc::clone(&temp_file_path),
            )?;

            let encrypted_file = temp_dir_path.join(format!("concurrent_test_{}.txt.feroxcrypt", i));
            assert!(encrypted_file.exists());

            // Decrypt
            fs::remove_file(&test_file)?;
            run_decryption_flow(&encrypted_file, &password, temp_file_path)?;

            let decrypted_content = fs::read_to_string(&test_file)?;
            assert_eq!(decrypted_content, format!("Concurrent test content {}", i));

            Ok(())
        });
        
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap()?;
    }

    Ok(())
}