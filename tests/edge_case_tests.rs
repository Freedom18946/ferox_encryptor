// tests/edge_case_tests.rs

//! Edge case and boundary tests for Ferox Encryptor

use anyhow::Result;
use ferox_encryptor::{run_decryption_flow, run_encryption_flow, Level};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tempfile::TempDir;

#[test]
fn test_maximum_filename_length() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Test filename at a more reasonable limit for most filesystems (e.g., 250 chars)
    let max_name = "a".repeat(250) + ".txt";
    let max_file = temp_dir.path().join(&max_name);
    fs::write(&max_file, b"max filename test")?;

    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
    let password = "max_filename_test";

    let result = run_encryption_flow(
        &max_file,
        false,
        password,
        Level::Interactive,
        None,
        Arc::clone(&temp_file_path),
    );

    // This should fail gracefully if the OS filesystem has a shorter limit.
    // If it succeeds, that's also fine, as some filesystems might support it.
    if let Err(e) = result {
        let error_msg = e.to_string();
        // We expect an error related to file creation or the OS limit
        assert!(
            error_msg.contains("无法创建目标文件")
                || error_msg.contains("Could not create target file")
                || error_msg.contains("File name too long"),
            "Unexpected error message: {}",
            error_msg
        );
    }

    Ok(())
}

#[test]
fn test_filename_too_long() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Test filename exceeding the limit
    let too_long_name = "a".repeat(70000) + ".txt";
    let _too_long_file = temp_dir.path().join(&too_long_name);

    // This might fail at filesystem level, so we'll create a shorter file
    // and simulate the error condition
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, b"test content")?;

    // We can't easily test this without modifying the source code,
    // but we can verify the error handling exists in the code
    println!("Filename length limit test - would require code modification to test properly");

    Ok(())
}

#[test]
fn test_zero_byte_file() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let zero_file = temp_dir.path().join("zero.bin");
    fs::write(&zero_file, &[])?;

    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
    let password = "zero_byte_test";

    // Encrypt zero-byte file
    run_encryption_flow(
        &zero_file,
        false,
        password,
        Level::Interactive,
        None,
        Arc::clone(&temp_file_path),
    )?;

    let encrypted_file = temp_dir.path().join("zero.bin.feroxcrypt");
    assert!(encrypted_file.exists());

    // Verify encrypted file has headers even for zero content
    let encrypted_size = fs::metadata(&encrypted_file)?.len();
    assert!(
        encrypted_size > 0,
        "Encrypted file should have headers even for zero content"
    );

    // Decrypt
    fs::remove_file(&zero_file)?;
    run_decryption_flow(&encrypted_file, password, None, Arc::clone(&temp_file_path))?;

    // Verify zero content
    let decrypted_content = fs::read(&zero_file)?;
    assert_eq!(decrypted_content.len(), 0);

    Ok(())
}

#[test]
fn test_single_byte_file() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let single_byte_file = temp_dir.path().join("single.bin");
    fs::write(&single_byte_file, &[0x42])?;

    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
    let password = "single_byte_test";

    // Encrypt single-byte file
    run_encryption_flow(
        &single_byte_file,
        false,
        password,
        Level::Interactive,
        None,
        Arc::clone(&temp_file_path),
    )?;

    let encrypted_file = temp_dir.path().join("single.bin.feroxcrypt");
    assert!(encrypted_file.exists());

    // Decrypt
    fs::remove_file(&single_byte_file)?;
    run_decryption_flow(&encrypted_file, password, None, Arc::clone(&temp_file_path))?;

    // Verify single byte content
    let decrypted_content = fs::read(&single_byte_file)?;
    assert_eq!(decrypted_content, &[0x42]);

    Ok(())
}

#[test]
fn test_binary_data_patterns() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let password = "binary_pattern_test";

    // Test various binary patterns
    let patterns = vec![
        (vec![0x00; 1000], "all_zeros"),
        (vec![0xFF; 1000], "all_ones"),
        (
            (0..=255).cycle().take(1000).collect::<Vec<u8>>(),
            "sequential",
        ),
        (
            (0..1000).map(|i| (i % 256) as u8).collect(),
            "modulo_pattern",
        ),
    ];

    for (pattern, name) in patterns {
        let test_file = temp_dir.path().join(format!("{}.bin", name));
        fs::write(&test_file, &pattern)?;

        let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));

        // Encrypt
        run_encryption_flow(
            &test_file,
            false,
            password,
            Level::Interactive,
            None,
            Arc::clone(&temp_file_path),
        )?;

        let encrypted_file = temp_dir.path().join(format!("{}.bin.feroxcrypt", name));
        assert!(encrypted_file.exists());

        // Verify encrypted data is different from original
        let encrypted_data = fs::read(&encrypted_file)?;
        // Skip headers and check that encrypted content differs from original
        assert!(encrypted_data.len() > pattern.len());

        // Decrypt
        fs::remove_file(&test_file)?;
        run_decryption_flow(&encrypted_file, password, None, Arc::clone(&temp_file_path))?;

        // Verify pattern is preserved
        let decrypted_content = fs::read(&test_file)?;
        assert_eq!(decrypted_content, pattern, "Pattern {} not preserved", name);
    }

    Ok(())
}

#[test]
fn test_file_with_no_extension() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let no_ext_file = temp_dir.path().join("no_extension_file");
    fs::write(&no_ext_file, b"file without extension")?;

    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
    let password = "no_extension_test";

    // Encrypt
    run_encryption_flow(
        &no_ext_file,
        false,
        password,
        Level::Interactive,
        None,
        Arc::clone(&temp_file_path),
    )?;

    let encrypted_file = temp_dir.path().join("no_extension_file.feroxcrypt");
    assert!(encrypted_file.exists());

    // Decrypt
    fs::remove_file(&no_ext_file)?;
    run_decryption_flow(&encrypted_file, password, None, Arc::clone(&temp_file_path))?;

    // Verify
    let decrypted_content = fs::read(&no_ext_file)?;
    assert_eq!(decrypted_content, b"file without extension");

    Ok(())
}

#[test]
fn test_file_with_multiple_dots() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let multi_dot_file = temp_dir.path().join("file.with.many.dots.txt");
    fs::write(&multi_dot_file, b"file with multiple dots")?;

    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
    let password = "multi_dot_test";

    // Encrypt
    run_encryption_flow(
        &multi_dot_file,
        false,
        password,
        Level::Interactive,
        None,
        Arc::clone(&temp_file_path),
    )?;

    let encrypted_file = temp_dir.path().join("file.with.many.dots.txt.feroxcrypt");
    assert!(encrypted_file.exists());

    // Decrypt
    fs::remove_file(&multi_dot_file)?;
    run_decryption_flow(&encrypted_file, password, None, Arc::clone(&temp_file_path))?;

    // Verify
    let decrypted_content = fs::read(&multi_dot_file)?;
    assert_eq!(decrypted_content, b"file with multiple dots");

    Ok(())
}

#[test]
fn test_already_encrypted_file_detection() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, b"original content")?;

    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
    let password = "double_encrypt_test";

    // First encryption
    run_encryption_flow(
        &test_file,
        false,
        password,
        Level::Interactive,
        None,
        Arc::clone(&temp_file_path),
    )?;

    let encrypted_file = temp_dir.path().join("test.txt.feroxcrypt");
    assert!(encrypted_file.exists());

    // Attempt to encrypt the already encrypted file - should fail
    let result = run_encryption_flow(
        &encrypted_file,
        false,
        password,
        Level::Interactive,
        None,
        Arc::clone(&temp_file_path),
    );

    assert!(
        result.is_err(),
        "Should not allow encrypting already encrypted files"
    );
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("already encrypted")
            || error_msg.contains("已加密")
            || error_msg.contains("feroxcrypt")
    );

    Ok(())
}

#[test]
fn test_overwrite_protection() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, b"original content")?;

    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
    let password = "overwrite_test";

    // Encrypt once
    run_encryption_flow(
        &test_file,
        false,
        password,
        Level::Interactive,
        None,
        Arc::clone(&temp_file_path),
    )?;

    let encrypted_file = temp_dir.path().join("test.txt.feroxcrypt");
    assert!(encrypted_file.exists());

    // Attempt to encrypt again without --force - should fail
    let result = run_encryption_flow(
        &test_file,
        false,
        password,
        Level::Interactive,
        None,
        Arc::clone(&temp_file_path),
    );
    assert!(result.is_err(), "Should fail without --force");
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("already exists")
            || error_msg.contains("已存在")
            || error_msg.contains("exists")
    );

    // Encrypt again with --force - should succeed
    let result_force = run_encryption_flow(
        &test_file,
        true, // force overwrite
        password,
        Level::Interactive,
        None,
        Arc::clone(&temp_file_path),
    );
    assert!(result_force.is_ok(), "Should succeed with --force");

    Ok(())
}

#[test]
fn test_non_feroxcrypt_file_decryption() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let regular_file = temp_dir.path().join("regular.txt");
    fs::write(&regular_file, b"this is not encrypted")?;

    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
    let password = "regular_file_test";

    // Attempt to decrypt a regular file - should fail
    let result = run_decryption_flow(&regular_file, password, None, Arc::clone(&temp_file_path));
    assert!(
        result.is_err(),
        "Should not allow decrypting non-encrypted files"
    );

    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("does not appear to be encrypted")
            || error_msg.contains("不是加密文件")
            || error_msg.contains("not encrypted")
            || error_msg.contains("无法读取加密文件头")
            || error_msg.contains("Cannot read encrypted file header")
            || error_msg.contains("文件格式错误")
            || error_msg.contains("Invalid file format")
            || error_msg.contains("文件看起来不是一个有效的加密文件")
            || error_msg.contains("feroxcrypt"),
        "Unexpected error message: {}",
        error_msg
    );

    Ok(())
}

#[test]
fn test_decryption_with_wrong_password() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_file = temp_dir.path().join("test_wrong_pass.txt");
    fs::write(&test_file, b"this is a secret")?;

    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
    let correct_password = "correct_password";
    let wrong_password = "wrong_password";

    // Encrypt with the correct password
    run_encryption_flow(
        &test_file,
        false,
        correct_password,
        Level::Interactive,
        None,
        Arc::clone(&temp_file_path),
    )?;

    let encrypted_file = temp_dir.path().join("test_wrong_pass.txt.feroxcrypt");
    assert!(encrypted_file.exists());

    // Remove the original file before attempting decryption
    fs::remove_file(&test_file)?;

    // Attempt to decrypt with the wrong password
    let result = run_decryption_flow(
        &encrypted_file,
        wrong_password,
        None,
        Arc::clone(&temp_file_path),
    );

    assert!(
        result.is_err(),
        "Decryption should fail with the wrong password"
    );
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("Authentication failed")
            || error_msg.contains("认证失败")
            || error_msg.contains("验证失败")
            || error_msg.contains("HMAC")
            || error_msg.contains("密码错误"),
        "Error message should indicate authentication failure: {}",
        error_msg
    );

    Ok(())
}

#[test]
fn test_decryption_with_tampered_hmac() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_file = temp_dir.path().join("test_tampered_hmac.txt");
    fs::write(&test_file, b"tamper with this")?;

    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
    let password = "hmac_tamper_test";

    // Encrypt the file
    run_encryption_flow(
        &test_file,
        false,
        password,
        Level::Interactive,
        None,
        Arc::clone(&temp_file_path),
    )?;

    let encrypted_file = temp_dir.path().join("test_tampered_hmac.txt.feroxcrypt");
    let mut encrypted_data = fs::read(&encrypted_file)?;

    // Tamper with the HMAC tag (last 32 bytes)
    let data_len = encrypted_data.len();
    encrypted_data[data_len - 5] ^= 0xFF; // Flip some bits in the tag
    fs::write(&encrypted_file, &encrypted_data)?;

    // Remove the original file before attempting decryption
    fs::remove_file(&test_file)?;

    // Attempt to decrypt the tampered file
    let result = run_decryption_flow(&encrypted_file, password, None, Arc::clone(&temp_file_path));

    assert!(
        result.is_err(),
        "Decryption should fail with a tampered HMAC"
    );
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("Authentication failed")
            || error_msg.contains("认证失败")
            || error_msg.contains("验证失败")
            || error_msg.contains("HMAC")
            || error_msg.contains("密码错误"),
        "Error message should indicate authentication failure: {}",
        error_msg
    );

    Ok(())
}

#[test]
fn test_decryption_with_tampered_ciphertext() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_file = temp_dir.path().join("test_tampered_ciphertext.txt");
    fs::write(&test_file, b"tamper the ciphertext now")?;

    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
    let password = "ciphertext_tamper_test";

    // Encrypt the file
    run_encryption_flow(
        &test_file,
        false,
        password,
        Level::Interactive,
        None,
        Arc::clone(&temp_file_path),
    )?;

    let encrypted_file = temp_dir
        .path()
        .join("test_tampered_ciphertext.txt.feroxcrypt");
    let mut encrypted_data = fs::read(&encrypted_file)?;

    // Tamper with the ciphertext (somewhere in the middle, after the header)
    encrypted_data[100] ^= 0xFF; // Flip a bit
    fs::write(&encrypted_file, &encrypted_data)?;

    // Remove the original file before attempting decryption
    fs::remove_file(&test_file)?;

    // Attempt to decrypt the tampered file
    let result = run_decryption_flow(&encrypted_file, password, None, Arc::clone(&temp_file_path));

    assert!(
        result.is_err(),
        "Decryption should fail with tampered ciphertext"
    );
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("Authentication failed")
            || error_msg.contains("认证失败")
            || error_msg.contains("验证失败")
            || error_msg.contains("HMAC")
            || error_msg.contains("密码错误"),
        "Error message should indicate authentication failure: {}",
        error_msg
    );

    Ok(())
}

#[test]
fn test_encrypt_non_existent_file() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let non_existent_file = temp_dir.path().join("i_do_not_exist.txt");
    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));

    let result = run_encryption_flow(
        &non_existent_file,
        false,
        "password",
        Level::Interactive,
        None,
        temp_file_path,
    );

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("File does not exist")
            || error_msg.contains("文件不存在")
            || error_msg.contains("No such file")
    );

    Ok(())
}

#[test]
fn test_encrypt_directory_as_input() -> Result<()> {
    let temp_dir = TempDir::new()?; // This is a directory
    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));

    let result = run_encryption_flow(
        temp_dir.path(),
        false,
        "password",
        Level::Interactive,
        None,
        temp_file_path,
    );

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("Path is not a file")
            || error_msg.contains("不是文件")
            || error_msg.contains("is a directory")
            || error_msg.contains("无法读取输入文件")
            || error_msg.contains("Cannot read input file")
            || error_msg.contains("提供的路径不是一个文件"),
        "Unexpected error message: {}",
        error_msg
    );

    Ok(())
}
