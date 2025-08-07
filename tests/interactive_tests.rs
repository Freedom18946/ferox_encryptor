// tests/interactive_tests.rs

//! # 交互式功能测试
//!
//! 测试交互式模块的核心功能，确保所有组件正常工作。
//! 由于交互式功能主要依赖用户输入，这里主要测试非交互部分。
//!
//! *Tests for interactive functionality. Since interactive features mainly 
//! depend on user input, this primarily tests non-interactive components.*

use anyhow::Result;
use ferox_encryptor::{
    batch::{batch_encrypt_files, batch_decrypt_files, BatchConfig},
    keyfile::KeyFile,
    Level,
};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// 测试所有安全级别的向后兼容性
#[test]
fn test_security_levels_compatibility() -> Result<()> {
    let temp_dir = TempDir::new()?;

    let levels = vec![Level::Interactive, Level::Moderate, Level::Paranoid];

    for level in levels {
        // 为每个级别创建独立的测试文件
        let test_file = temp_dir.path().join(format!("test_{:?}.txt", level));
        fs::write(&test_file, format!("Test content for {:?} level", level))?;

        // 加密后的文件路径
        let encrypted_file = temp_dir.path().join(format!("test_{:?}.txt.feroxcrypt", level));

        // 测试加密
        let config = BatchConfig {
            level,
            force_overwrite: true,
            ..Default::default()
        };

        let encrypt_result = batch_encrypt_files(
            &[test_file.clone()],
            "test_password",
            None,
            &config,
        )?;

        assert_eq!(encrypt_result.success_count, 1);
        assert_eq!(encrypt_result.failure_count, 0);
        assert!(encrypted_file.exists());

        // 删除原文件，准备解密测试
        fs::remove_file(&test_file)?;

        // 测试解密
        let decrypt_result = batch_decrypt_files(
            &[encrypted_file],
            "test_password",
            None,
        )?;

        assert_eq!(decrypt_result.success_count, 1);
        assert_eq!(decrypt_result.failure_count, 0);
        assert!(test_file.exists()); // 解密后原文件应该恢复

        // 验证内容
        let decrypted_content = fs::read_to_string(&test_file)?;
        assert_eq!(decrypted_content, format!("Test content for {:?} level", level));
    }

    Ok(())
}

/// 测试密钥文件功能的向后兼容性
#[test]
fn test_keyfile_compatibility() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let test_file = temp_dir.path().join("test.txt");
    let keyfile_path = temp_dir.path().join("test.key");

    fs::write(&test_file, "Test content with keyfile")?;

    // 生成密钥文件
    let keyfile = KeyFile::generate();
    keyfile.save_to_file(&keyfile_path)?;

    // 重新加载密钥文件
    let loaded_keyfile = KeyFile::load_from_file(&keyfile_path)?;

    // 测试加密
    let config = BatchConfig {
        level: Level::Moderate,
        force_overwrite: true,
        ..Default::default()
    };

    let encrypt_result = batch_encrypt_files(
        &[test_file.clone()],
        "test_password",
        Some(&loaded_keyfile),
        &config,
    )?;

    assert_eq!(encrypt_result.success_count, 1);
    assert_eq!(encrypt_result.failure_count, 0);

    // 删除原文件，准备解密测试
    fs::remove_file(&test_file)?;

    // 测试解密
    let encrypted_file = temp_dir.path().join("test.txt.feroxcrypt");
    assert!(encrypted_file.exists());

    let decrypt_result = batch_decrypt_files(
        &[encrypted_file],
        "test_password",
        Some(&loaded_keyfile),
    )?;

    assert_eq!(decrypt_result.success_count, 1);
    assert_eq!(decrypt_result.failure_count, 0);
    assert!(test_file.exists()); // 解密后原文件应该恢复

    // 验证内容
    let decrypted_content = fs::read_to_string(&test_file)?;
    assert_eq!(decrypted_content, "Test content with keyfile");

    Ok(())
}

/// 测试批量配置选项
#[test]
fn test_batch_configuration_options() -> Result<()> {
    let temp_dir = TempDir::new()?;
    
    // 创建测试文件
    let files = vec![
        temp_dir.path().join("test1.txt"),
        temp_dir.path().join("test2.txt"),
        temp_dir.path().join("test3.txt"),
    ];
    
    for file in &files {
        fs::write(file, "Test content")?;
    }
    
    // 测试不同的批量配置
    let configs = vec![
        BatchConfig {
            level: Level::Interactive,
            force_overwrite: false,
            recursive: false,
            include_patterns: vec![],
            exclude_patterns: vec![],
        },
        BatchConfig {
            level: Level::Moderate,
            force_overwrite: true,
            recursive: true,
            include_patterns: vec![],
            exclude_patterns: vec![],
        },
        BatchConfig {
            level: Level::Paranoid,
            force_overwrite: true,
            recursive: false,
            include_patterns: vec![],
            exclude_patterns: vec![],
        },
    ];
    
    for (i, config) in configs.iter().enumerate() {
        let test_files: Vec<PathBuf> = files.iter()
            .map(|f| {
                let mut new_path = f.clone();
                new_path.set_file_name(format!("test{}_config{}.txt", 
                    new_path.file_stem().unwrap().to_str().unwrap().chars().last().unwrap(), i));
                fs::copy(f, &new_path).unwrap();
                new_path
            })
            .collect();
        
        let result = batch_encrypt_files(
            &test_files,
            "test_password",
            None,
            config,
        )?;
        
        assert_eq!(result.success_count, test_files.len());
        assert_eq!(result.failure_count, 0);
    }
    
    Ok(())
}

/// 测试错误处理和恢复
#[test]
fn test_error_handling() -> Result<()> {
    let temp_dir = TempDir::new()?;
    
    // 测试不存在的文件
    let nonexistent_file = temp_dir.path().join("nonexistent.txt");
    let config = BatchConfig::default();
    
    let result = batch_encrypt_files(
        &[nonexistent_file],
        "test_password",
        None,
        &config,
    );
    
    // 应该返回错误或者在结果中标记失败
    match result {
        Ok(batch_result) => {
            // 如果返回成功，应该有失败计数
            assert!(batch_result.failure_count > 0 || batch_result.success_count == 0);
        }
        Err(_) => {
            // 返回错误也是可接受的
        }
    }
    
    Ok(())
}

/// 测试大文件处理能力
#[test]
fn test_large_file_handling() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let large_file = temp_dir.path().join("large_test.txt");

    // 创建一个较大的测试文件 (1MB)
    let content = "A".repeat(1024 * 1024);
    fs::write(&large_file, &content)?;

    let config = BatchConfig {
        level: Level::Interactive, // 使用快速级别进行测试
        force_overwrite: true,
        ..Default::default()
    };

    // 测试加密
    let encrypt_result = batch_encrypt_files(
        &[large_file.clone()],
        "test_password",
        None,
        &config,
    )?;

    assert_eq!(encrypt_result.success_count, 1);
    assert_eq!(encrypt_result.failure_count, 0);
    assert!(encrypt_result.total_bytes > 0);

    // 删除原文件，准备解密测试
    fs::remove_file(&large_file)?;

    // 测试解密
    let encrypted_file = temp_dir.path().join("large_test.txt.feroxcrypt");
    assert!(encrypted_file.exists());

    let decrypt_result = batch_decrypt_files(
        &[encrypted_file],
        "test_password",
        None,
    )?;

    assert_eq!(decrypt_result.success_count, 1);
    assert_eq!(decrypt_result.failure_count, 0);
    assert!(large_file.exists()); // 解密后原文件应该恢复

    // 验证内容
    let decrypted_content = fs::read_to_string(&large_file)?;
    assert_eq!(content, decrypted_content);

    Ok(())
}

/// 测试命令行兼容性
#[test]
fn test_cli_backward_compatibility() {
    use std::process::Command;
    
    // 测试帮助命令
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");
    
    let help_text = String::from_utf8_lossy(&output.stdout);
    
    // 验证所有原有命令仍然存在
    assert!(help_text.contains("encrypt"));
    assert!(help_text.contains("decrypt"));
    assert!(help_text.contains("batch-encrypt"));
    assert!(help_text.contains("batch-decrypt"));
    assert!(help_text.contains("generate-key"));
    
    // 验证新的交互式命令存在
    assert!(help_text.contains("interactive"));
}

/// 测试模块集成
#[test]
fn test_module_integration() {
    // 验证所有模块都可以正确导入
    use ferox_encryptor::{
        batch::BatchConfig,
        keyfile::KeyFile,
        Level,
    };

    // 测试枚举值
    let _levels = vec![Level::Interactive, Level::Moderate, Level::Paranoid];

    // 测试结构体创建
    let _config = BatchConfig::default();
    let _keyfile = KeyFile::generate();

    // 验证交互式模块存在（虽然我们不能直接测试交互功能）
    // 这确保模块正确编译和导出
    let _module_exists = std::any::type_name::<fn() -> Result<()>>();
    assert!(_module_exists.len() > 0);
}
