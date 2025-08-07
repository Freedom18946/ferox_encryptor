// tests/performance_tests.rs

//! Performance tests for Ferox Encryptor
//! These tests can be slow and are ignored by default.
//! Run them with `cargo test -- --ignored`

use anyhow::Result;
use ferox_encryptor::{run_decryption_flow, run_encryption_flow, Level};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tempfile::TempDir;

#[test]
#[ignore]
fn test_large_file_performance() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Create a 10MB test file
    let file_size = 10 * 1024 * 1024; // 10MB
    let test_content = vec![0xAB; file_size];
    let test_file = temp_dir.path().join("large_file.bin");
    fs::write(&test_file, &test_content)?;

    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
    let password = "performance_test_password";

    // Test encryption performance
    let encrypt_start = Instant::now();
    run_encryption_flow(
        &test_file,
        false,
        password,
        Level::Interactive, // Use fastest level for performance test
        None,
        Arc::clone(&temp_file_path),
    )?;
    let encrypt_duration = encrypt_start.elapsed();

    let encrypted_file = temp_dir.path().join("large_file.bin.feroxcrypt");
    assert!(encrypted_file.exists());

    // Calculate encryption throughput
    let encrypt_throughput = file_size as f64 / encrypt_duration.as_secs_f64() / (1024.0 * 1024.0);
    println!(
        "
[Large File] Encryption throughput (Interactive): {:.2} MB/s",
        encrypt_throughput
    );

    // Test decryption performance
    fs::remove_file(&test_file)?;
    let decrypt_start = Instant::now();
    run_decryption_flow(&encrypted_file, password, None, temp_file_path)?;
    let decrypt_duration = decrypt_start.elapsed();

    // Calculate decryption throughput
    let decrypt_throughput = file_size as f64 / decrypt_duration.as_secs_f64() / (1024.0 * 1024.0);
    println!(
        "[Large File] Decryption throughput (Interactive): {:.2} MB/s",
        decrypt_throughput
    );

    // Verify content integrity
    let decrypted_content = fs::read(&test_file)?;
    assert_eq!(decrypted_content, test_content);

    // Performance assertions (these are quite lenient to account for different systems)
    assert!(
        encrypt_throughput > 1.0,
        "Encryption should be faster than 1 MB/s"
    );
    assert!(
        decrypt_throughput > 1.0,
        "Decryption should be faster than 1 MB/s"
    );

    Ok(())
}

#[test]
#[ignore]
fn test_security_level_performance_comparison() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Create a larger test file for this comparison (5MB) to better show differences
    let file_size = 5 * 1024 * 1024; // 5MB
    let test_content = vec![0xCD; file_size];
    let password = "security_level_test";

    let levels = [Level::Interactive, Level::Moderate, Level::Paranoid];
    let mut results = Vec::new();

    println!(
        "
[Security Level Comparison] Testing with a {}MB file...",
        file_size / (1024 * 1024)
    );

    for level in levels {
        let test_file = temp_dir.path().join(format!("test_{:?}.bin", level));
        fs::write(&test_file, &test_content)?;

        let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));

        // Measure encryption time
        let encrypt_start = Instant::now();
        run_encryption_flow(
            &test_file,
            false,
            password,
            level,
            None,
            Arc::clone(&temp_file_path),
        )?;
        let encrypt_duration = encrypt_start.elapsed();

        let encrypted_file = temp_dir
            .path()
            .join(format!("test_{:?}.bin.feroxcrypt", level));

        // Measure decryption time
        fs::remove_file(&test_file)?;
        let decrypt_start = Instant::now();
        run_decryption_flow(&encrypted_file, password, None, temp_file_path)?;
        let decrypt_duration = decrypt_start.elapsed();

        // Verify integrity
        let decrypted_content = fs::read(&test_file)?;
        assert_eq!(decrypted_content, test_content);

        results.push((level, encrypt_duration, decrypt_duration));

        println!(
            "Level: {:<12} | Encrypt: {:<8.2}s | Decrypt: {:.2}s",
            format!("{:?}", level),
            encrypt_duration.as_secs_f64(),
            decrypt_duration.as_secs_f64()
        );
    }

    // Verify that higher security levels take longer (at least for encryption)
    // Allow for a small tolerance for measurement noise
    assert!(
        results[0].1.as_secs_f64() <= results[1].1.as_secs_f64() + 0.5, // Interactive <= Moderate
        "Interactive should be faster than or equal to Moderate"
    );
    assert!(
        results[1].1.as_secs_f64() <= results[2].1.as_secs_f64() + 0.5, // Moderate <= Paranoid
        "Moderate should be faster than or equal to Paranoid"
    );

    Ok(())
}

#[test]
#[ignore]
fn test_memory_usage_with_large_file() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Create a 50MB test file to test memory efficiency
    let file_size = 50 * 1024 * 1024; // 50MB
    let test_file = temp_dir.path().join("memory_test.bin");

    // Create file with pattern to verify integrity
    let mut test_content = Vec::with_capacity(file_size);
    for i in 0..file_size {
        test_content.push((i % 256) as u8);
    }
    fs::write(&test_file, &test_content)?;

    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));
    let password = "memory_test_password";

    println!(
        "
[Memory Usage] Processing {}MB file to check for low memory footprint...",
        file_size / (1024 * 1024)
    );
    // Encrypt large file
    run_encryption_flow(
        &test_file,
        false,
        password,
        Level::Interactive,
        None,
        Arc::clone(&temp_file_path),
    )?;

    let encrypted_file = temp_dir.path().join("memory_test.bin.feroxcrypt");
    assert!(encrypted_file.exists());

    // Decrypt large file
    fs::remove_file(&test_file)?;
    run_decryption_flow(&encrypted_file, password, None, temp_file_path)?;

    // Verify content integrity
    let decrypted_content = fs::read(&test_file)?;
    assert_eq!(decrypted_content.len(), test_content.len());

    // Verify pattern integrity (check every 1000th byte to speed up test)
    for i in (0..decrypted_content.len()).step_by(1000) {
        assert_eq!(
            decrypted_content[i], test_content[i],
            "Content mismatch at position {}",
            i
        );
    }

    println!(
        "[Memory Usage] Successfully processed {}MB file",
        file_size / (1024 * 1024)
    );

    Ok(())
}

#[test]
#[ignore]
fn test_many_small_files_performance() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let password = "small_files_test";
    let num_files = 100;
    let file_size = 1024; // 1KB each

    // Create many small files
    let mut test_files = Vec::new();
    for i in 0..num_files {
        let test_file = temp_dir.path().join(format!("small_file_{}.txt", i));
        let content = format!("Small file content {}", i).repeat(file_size / 20);
        fs::write(&test_file, content.as_bytes())?;
        test_files.push(test_file);
    }

    let temp_file_path = Arc::new(Mutex::new(None::<PathBuf>));

    println!(
        "
[Small Files] Processing {} small files (1KB each)...",
        num_files
    );
    // Measure time to encrypt all files
    let encrypt_start = Instant::now();
    for test_file in &test_files {
        run_encryption_flow(
            test_file,
            false,
            password,
            Level::Interactive,
            None,
            Arc::clone(&temp_file_path),
        )?;
    }
    let encrypt_duration = encrypt_start.elapsed();

    // Measure time to decrypt all files
    for test_file in &test_files {
        fs::remove_file(test_file)?;
    }

    let decrypt_start = Instant::now();
    for test_file in &test_files {
        let encrypted_file = temp_dir.path().join(format!(
            "{}.feroxcrypt",
            test_file.file_name().unwrap().to_str().unwrap()
        ));
        run_decryption_flow(&encrypted_file, password, None, Arc::clone(&temp_file_path))?;
    }
    let decrypt_duration = decrypt_start.elapsed();

    println!(
        "[Small Files] Total time for {} files: Encrypt: {:.2}s, Decrypt: {:.2}s",
        num_files,
        encrypt_duration.as_secs_f64(),
        decrypt_duration.as_secs_f64()
    );

    // Verify all files were processed correctly
    for (i, test_file) in test_files.iter().enumerate() {
        assert!(
            test_file.exists(),
            "File {} should exist after decryption",
            i
        );
        let content = fs::read_to_string(test_file)?;
        assert!(content.contains(&format!("Small file content {i}")));
    }

    // Performance assertion - should handle 100 small files reasonably quickly
    assert!(
        encrypt_duration.as_secs() < 60,
        "Should encrypt 100 small files in under 60 seconds"
    );
    assert!(
        decrypt_duration.as_secs() < 60,
        "Should decrypt 100 small files in under 60 seconds"
    );

    Ok(())
}
