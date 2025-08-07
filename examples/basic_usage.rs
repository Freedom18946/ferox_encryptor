//! # Ferox Encryptor 基本使用示例 (Basic Usage Example)
//!
//! 本示例展示了如何在 Rust 代码中使用 Ferox Encryptor 库进行文件加密和解密。
//!
//! *This example demonstrates how to use the Ferox Encryptor library for file encryption and decryption in Rust code.*

use anyhow::Result;
use ferox_encryptor::{run_decryption_flow, run_encryption_flow, Level};
use std::fs;

use std::sync::{Arc, Mutex};
use tempfile::TempDir;

/// 基本的加密解密示例 (Basic encryption/decryption example)
fn basic_encrypt_decrypt_example() -> Result<()> {
    println!("🔐 基本加密解密示例 (Basic Encryption/Decryption Example)");
    
    // 创建临时目录用于演示 (Create temporary directory for demonstration)
    let temp_dir = TempDir::new()?;
    let temp_file_path = Arc::new(Mutex::new(None));
    
    // 创建测试文件 (Create test file)
    let test_file = temp_dir.path().join("secret_document.txt");
    let test_content = "这是一个需要加密的机密文档。\nThis is a confidential document that needs encryption.";
    fs::write(&test_file, test_content)?;
    
    println!("📄 创建测试文件: {}", test_file.display());
    println!("📝 文件内容: {}", test_content);
    
    // 设置加密参数 (Set encryption parameters)
    let password = "my_secure_password_123";
    let security_level = Level::Moderate;
    
    println!("\n🔒 开始加密...");
    println!("🔑 使用密码: {} (在实际应用中不要打印密码!)", password);
    println!("🛡️ 安全级别: {:?}", security_level);
    
    // 执行加密 (Perform encryption)
    run_encryption_flow(
        &test_file,
        false, // 不强制覆盖 (don't force overwrite)
        password,
        security_level,
        None, // 不使用密钥文件 (no keyfile)
        Arc::clone(&temp_file_path),
    )?;
    
    let encrypted_file = temp_dir.path().join("secret_document.txt.feroxcrypt");
    println!("✅ 加密完成! 加密文件: {}", encrypted_file.display());
    
    // 显示文件大小对比 (Show file size comparison)
    let original_size = fs::metadata(&test_file)?.len();
    let encrypted_size = fs::metadata(&encrypted_file)?.len();
    println!("📊 原始文件大小: {} 字节", original_size);
    println!("📊 加密文件大小: {} 字节", encrypted_size);
    println!("📊 大小增加: {} 字节 (包含加密头和认证标签)", encrypted_size - original_size);
    
    // 删除原始文件以模拟真实场景 (Remove original file to simulate real scenario)
    fs::remove_file(&test_file)?;
    println!("\n🗑️ 删除原始文件 (模拟真实使用场景)");
    
    println!("\n🔓 开始解密...");
    
    // 执行解密 (Perform decryption)
    run_decryption_flow(
        &encrypted_file,
        password,
        None, // 不使用密钥文件 (no keyfile)
        temp_file_path,
    )?;
    
    println!("✅ 解密完成! 恢复文件: {}", test_file.display());
    
    // 验证解密结果 (Verify decryption result)
    let decrypted_content = fs::read_to_string(&test_file)?;
    if decrypted_content == test_content {
        println!("✅ 验证成功: 解密内容与原始内容完全一致!");
    } else {
        println!("❌ 验证失败: 解密内容与原始内容不一致!");
    }
    
    println!("📝 解密后的内容: {}", decrypted_content);
    
    Ok(())
}

/// 不同安全级别的性能对比示例 (Performance comparison example for different security levels)
fn security_levels_comparison_example() -> Result<()> {
    println!("\n\n🏆 安全级别性能对比示例 (Security Levels Performance Comparison)");
    
    let temp_dir = TempDir::new()?;
    let test_content = "性能测试文档内容".repeat(1000); // 创建较大的测试内容
    
    let levels = [
        (Level::Interactive, "交互式 (Interactive)"),
        (Level::Moderate, "中等 (Moderate)"),
        (Level::Paranoid, "偏执 (Paranoid)"),
    ];
    
    for (level, level_name) in levels {
        println!("\n🔐 测试安全级别: {}", level_name);
        
        let test_file = temp_dir.path().join(format!("test_{:?}.txt", level));
        fs::write(&test_file, &test_content)?;
        
        let temp_file_path = Arc::new(Mutex::new(None));
        let password = "performance_test_password";
        
        // 测量加密时间 (Measure encryption time)
        let start_time = std::time::Instant::now();
        run_encryption_flow(
            &test_file,
            false,
            password,
            level,
            None,
            Arc::clone(&temp_file_path),
        )?;
        let encrypt_duration = start_time.elapsed();
        
        let encrypted_file = temp_dir.path().join(format!("test_{:?}.txt.feroxcrypt", level));
        
        // 删除原始文件 (Remove original file)
        fs::remove_file(&test_file)?;
        
        // 测量解密时间 (Measure decryption time)
        let start_time = std::time::Instant::now();
        run_decryption_flow(&encrypted_file, password, None, temp_file_path)?;
        let decrypt_duration = start_time.elapsed();
        
        println!("⏱️ 加密时间: {:.2?}", encrypt_duration);
        println!("⏱️ 解密时间: {:.2?}", decrypt_duration);
        println!("⏱️ 总时间: {:.2?}", encrypt_duration + decrypt_duration);
        
        // 清理文件 (Clean up files)
        let _ = fs::remove_file(&test_file);
        let _ = fs::remove_file(&encrypted_file);
    }
    
    Ok(())
}

/// 错误处理示例 (Error handling example)
fn error_handling_example() -> Result<()> {
    println!("\n\n❌ 错误处理示例 (Error Handling Example)");
    
    let temp_dir = TempDir::new()?;
    let temp_file_path = Arc::new(Mutex::new(None));
    
    // 示例1: 尝试加密不存在的文件 (Example 1: Try to encrypt non-existent file)
    println!("\n📁 示例1: 尝试加密不存在的文件");
    let non_existent_file = temp_dir.path().join("does_not_exist.txt");
    
    match run_encryption_flow(
        &non_existent_file,
        false,
        "password",
        Level::Interactive,
        None,
        Arc::clone(&temp_file_path),
    ) {
        Ok(_) => println!("❌ 意外成功 - 这不应该发生!"),
        Err(e) => println!("✅ 预期错误: {}", e),
    }
    
    // 示例2: 尝试用错误密码解密 (Example 2: Try to decrypt with wrong password)
    println!("\n🔑 示例2: 尝试用错误密码解密");
    
    // 首先创建一个加密文件 (First create an encrypted file)
    let test_file = temp_dir.path().join("password_test.txt");
    fs::write(&test_file, "密码测试内容")?;
    
    let correct_password = "correct_password";
    let wrong_password = "wrong_password";
    
    run_encryption_flow(
        &test_file,
        false,
        correct_password,
        Level::Interactive,
        None,
        Arc::clone(&temp_file_path),
    )?;
    
    let encrypted_file = temp_dir.path().join("password_test.txt.feroxcrypt");
    fs::remove_file(&test_file)?; // 删除原始文件
    
    // 尝试用错误密码解密 (Try to decrypt with wrong password)
    match run_decryption_flow(&encrypted_file, wrong_password, None, temp_file_path) {
        Ok(_) => println!("❌ 意外成功 - 这不应该发生!"),
        Err(e) => println!("✅ 预期错误 (密码错误): {}", e),
    }
    
    Ok(())
}

/// 主函数 - 运行所有示例 (Main function - run all examples)
fn main() -> Result<()> {
    println!("🚀 Ferox Encryptor 使用示例集合");
    println!("🚀 Ferox Encryptor Usage Examples Collection");
    println!("{}", "=".repeat(60));
    
    // 运行基本示例 (Run basic example)
    basic_encrypt_decrypt_example()?;
    
    // 运行性能对比示例 (Run performance comparison example)
    security_levels_comparison_example()?;
    
    // 运行错误处理示例 (Run error handling example)
    error_handling_example()?;
    
    println!("\n\n🎉 所有示例运行完成!");
    println!("🎉 All examples completed successfully!");
    println!("\n💡 提示: 在实际应用中，请确保:");
    println!("💡 Tips: In real applications, please ensure:");
    println!("   - 使用强密码 (Use strong passwords)");
    println!("   - 安全存储密码 (Store passwords securely)");
    println!("   - 定期备份重要文件 (Regularly backup important files)");
    println!("   - 选择适当的安全级别 (Choose appropriate security levels)");
    
    Ok(())
}
