//! # 密钥文件使用示例 (Keyfile Usage Example)
//!
//! 本示例展示了如何使用密钥文件来增强加密安全性。
//! 密钥文件提供了双重保护：即使密码泄露，没有密钥文件也无法解密数据。
//!
//! *This example demonstrates how to use keyfiles to enhance encryption security.*
//! *Keyfiles provide dual protection: even if the password is compromised, data cannot be decrypted without the keyfile.*

use anyhow::Result;
use ferox_encryptor::{
    keyfile::{validate_keyfile, KeyFile},
    run_decryption_flow, run_encryption_flow, Level,
};
use std::fs;

use std::sync::{Arc, Mutex};
use tempfile::TempDir;

/// 基本密钥文件使用示例 (Basic keyfile usage example)
fn basic_keyfile_example() -> Result<()> {
    println!("🔐 基本密钥文件使用示例 (Basic Keyfile Usage Example)");
    
    let temp_dir = TempDir::new()?;
    let temp_file_path = Arc::new(Mutex::new(None));
    
    // 1. 生成密钥文件 (Generate keyfile)
    println!("\n🔑 步骤1: 生成密钥文件");
    let keyfile = KeyFile::generate();
    let keyfile_path = temp_dir.path().join("secret.key");
    keyfile.save_to_file(&keyfile_path)?;
    
    println!("✅ 密钥文件已生成: {}", keyfile_path.display());
    println!("📊 密钥文件大小: {} 字节", fs::metadata(&keyfile_path)?.len());
    
    // 2. 验证密钥文件 (Validate keyfile)
    println!("\n🔍 步骤2: 验证密钥文件");
    validate_keyfile(&keyfile_path)?;
    println!("✅ 密钥文件验证通过");
    
    // 3. 创建测试文件 (Create test file)
    println!("\n📄 步骤3: 创建测试文件");
    let test_file = temp_dir.path().join("confidential.txt");
    let test_content = "这是使用密钥文件保护的机密信息。\nThis is confidential information protected by a keyfile.";
    fs::write(&test_file, test_content)?;
    println!("📝 测试文件内容: {}", test_content);
    
    // 4. 使用密钥文件加密 (Encrypt with keyfile)
    println!("\n🔒 步骤4: 使用密钥文件加密");
    let password = "my_password";
    
    // 重新加载密钥文件以模拟真实使用场景 (Reload keyfile to simulate real usage)
    let loaded_keyfile = KeyFile::load_from_file(&keyfile_path)?;
    
    run_encryption_flow(
        &test_file,
        false,
        password,
        Level::Moderate,
        Some(&loaded_keyfile), // 使用密钥文件 (Use keyfile)
        Arc::clone(&temp_file_path),
    )?;
    
    let encrypted_file = temp_dir.path().join("confidential.txt.feroxcrypt");
    println!("✅ 加密完成: {}", encrypted_file.display());
    
    // 5. 删除原始文件 (Remove original file)
    fs::remove_file(&test_file)?;
    println!("🗑️ 原始文件已删除");
    
    // 6. 使用密钥文件解密 (Decrypt with keyfile)
    println!("\n🔓 步骤5: 使用密钥文件解密");
    let loaded_keyfile_for_decrypt = KeyFile::load_from_file(&keyfile_path)?;
    
    run_decryption_flow(
        &encrypted_file,
        password,
        Some(&loaded_keyfile_for_decrypt), // 使用密钥文件 (Use keyfile)
        temp_file_path,
    )?;
    
    println!("✅ 解密完成: {}", test_file.display());
    
    // 7. 验证解密结果 (Verify decryption result)
    let decrypted_content = fs::read_to_string(&test_file)?;
    if decrypted_content == test_content {
        println!("✅ 验证成功: 解密内容与原始内容完全一致!");
    } else {
        println!("❌ 验证失败: 解密内容与原始内容不一致!");
    }
    
    Ok(())
}

/// 密钥文件安全性演示 (Keyfile security demonstration)
fn keyfile_security_demonstration() -> Result<()> {
    println!("\n\n🛡️ 密钥文件安全性演示 (Keyfile Security Demonstration)");
    
    let temp_dir = TempDir::new()?;
    let temp_file_path = Arc::new(Mutex::new(None));
    
    // 创建测试文件和密钥文件 (Create test file and keyfile)
    let test_file = temp_dir.path().join("secure_data.txt");
    let test_content = "高度机密的数据内容";
    fs::write(&test_file, test_content)?;
    
    let keyfile = KeyFile::generate();
    let keyfile_path = temp_dir.path().join("security.key");
    keyfile.save_to_file(&keyfile_path)?;
    
    let password = "shared_password";
    
    // 使用密钥文件加密 (Encrypt with keyfile)
    println!("\n🔒 使用密码 + 密钥文件加密");
    let loaded_keyfile = KeyFile::load_from_file(&keyfile_path)?;
    
    run_encryption_flow(
        &test_file,
        false,
        password,
        Level::Moderate,
        Some(&loaded_keyfile),
        Arc::clone(&temp_file_path),
    )?;
    
    let encrypted_file = temp_dir.path().join("secure_data.txt.feroxcrypt");
    fs::remove_file(&test_file)?;
    
    // 场景1: 只有密码，没有密钥文件 (Scenario 1: Password only, no keyfile)
    println!("\n❌ 场景1: 尝试仅用密码解密 (应该失败)");
    match run_decryption_flow(
        &encrypted_file,
        password,
        None, // 没有密钥文件 (No keyfile)
        Arc::clone(&temp_file_path),
    ) {
        Ok(_) => println!("❌ 意外成功 - 这表明安全性有问题!"),
        Err(e) => println!("✅ 预期失败: {}", e),
    }
    
    // 场景2: 错误的密钥文件 (Scenario 2: Wrong keyfile)
    println!("\n❌ 场景2: 使用错误的密钥文件 (应该失败)");
    let wrong_keyfile = KeyFile::generate(); // 生成不同的密钥文件
    let wrong_keyfile_path = temp_dir.path().join("wrong.key");
    wrong_keyfile.save_to_file(&wrong_keyfile_path)?;
    let wrong_loaded_keyfile = KeyFile::load_from_file(&wrong_keyfile_path)?;
    
    match run_decryption_flow(
        &encrypted_file,
        password,
        Some(&wrong_loaded_keyfile), // 错误的密钥文件 (Wrong keyfile)
        Arc::clone(&temp_file_path),
    ) {
        Ok(_) => println!("❌ 意外成功 - 这表明安全性有问题!"),
        Err(e) => println!("✅ 预期失败: {}", e),
    }
    
    // 场景3: 正确的密码和密钥文件 (Scenario 3: Correct password and keyfile)
    println!("\n✅ 场景3: 使用正确的密码和密钥文件");
    let correct_keyfile = KeyFile::load_from_file(&keyfile_path)?;
    
    run_decryption_flow(
        &encrypted_file,
        password,
        Some(&correct_keyfile), // 正确的密钥文件 (Correct keyfile)
        temp_file_path,
    )?;
    
    println!("✅ 解密成功!");
    
    // 验证内容 (Verify content)
    let decrypted_content = fs::read_to_string(&test_file)?;
    println!("📝 解密内容: {}", decrypted_content);
    
    Ok(())
}

/// 密钥文件最佳实践演示 (Keyfile best practices demonstration)
fn keyfile_best_practices() -> Result<()> {
    println!("\n\n💡 密钥文件最佳实践演示 (Keyfile Best Practices Demonstration)");
    
    let temp_dir = TempDir::new()?;
    
    // 最佳实践1: 密钥文件验证 (Best practice 1: Keyfile validation)
    println!("\n1️⃣ 最佳实践: 使用前验证密钥文件");
    
    let keyfile = KeyFile::generate();
    let keyfile_path = temp_dir.path().join("validated.key");
    keyfile.save_to_file(&keyfile_path)?;
    
    // 验证密钥文件 (Validate keyfile)
    match validate_keyfile(&keyfile_path) {
        Ok(_) => println!("✅ 密钥文件验证通过"),
        Err(e) => println!("❌ 密钥文件验证失败: {}", e),
    }
    
    // 最佳实践2: 检查无效的密钥文件 (Best practice 2: Check invalid keyfiles)
    println!("\n2️⃣ 最佳实践: 检测无效的密钥文件");
    
    // 创建一个太小的文件 (Create a file that's too small)
    let invalid_keyfile_path = temp_dir.path().join("invalid.key");
    fs::write(&invalid_keyfile_path, b"too_small")?;
    
    match validate_keyfile(&invalid_keyfile_path) {
        Ok(_) => println!("❌ 意外通过验证"),
        Err(e) => println!("✅ 正确检测到无效密钥文件: {}", e),
    }
    
    // 最佳实践3: 密钥文件备份建议 (Best practice 3: Keyfile backup recommendations)
    println!("\n3️⃣ 最佳实践: 密钥文件备份策略");
    println!("📋 建议的备份策略:");
    println!("   • 创建多个密钥文件副本");
    println!("   • 存储在不同的物理位置");
    println!("   • 使用云存储作为备份选项");
    println!("   • 定期验证备份的完整性");
    
    // 演示创建备份 (Demonstrate creating backups)
    let original_keyfile = KeyFile::generate();
    let original_path = temp_dir.path().join("original.key");
    original_keyfile.save_to_file(&original_path)?;
    
    // 创建备份副本 (Create backup copies)
    let backup_paths = [
        temp_dir.path().join("backup1.key"),
        temp_dir.path().join("backup2.key"),
        temp_dir.path().join("backup3.key"),
    ];
    
    for backup_path in &backup_paths {
        fs::copy(&original_path, backup_path)?;
        println!("📁 创建备份: {}", backup_path.display());
    }
    
    // 验证所有备份 (Verify all backups)
    println!("\n🔍 验证所有备份:");
    for backup_path in &backup_paths {
        match validate_keyfile(backup_path) {
            Ok(_) => println!("✅ 备份验证通过: {}", backup_path.display()),
            Err(e) => println!("❌ 备份验证失败: {} - {}", backup_path.display(), e),
        }
    }
    
    Ok(())
}

/// 主函数 - 运行所有密钥文件示例 (Main function - run all keyfile examples)
fn main() -> Result<()> {
    println!("🔑 Ferox Encryptor 密钥文件使用示例");
    println!("🔑 Ferox Encryptor Keyfile Usage Examples");
    println!("{}", "=".repeat(60));
    
    // 运行基本密钥文件示例 (Run basic keyfile example)
    basic_keyfile_example()?;
    
    // 运行安全性演示 (Run security demonstration)
    keyfile_security_demonstration()?;
    
    // 运行最佳实践演示 (Run best practices demonstration)
    keyfile_best_practices()?;
    
    println!("\n\n🎉 所有密钥文件示例运行完成!");
    println!("🎉 All keyfile examples completed successfully!");
    
    println!("\n🔒 密钥文件安全提醒:");
    println!("🔒 Keyfile Security Reminders:");
    println!("   • 密钥文件与密码同等重要 (Keyfiles are as important as passwords)");
    println!("   • 妥善保管密钥文件 (Store keyfiles securely)");
    println!("   • 制作多个备份副本 (Create multiple backup copies)");
    println!("   • 不要通过不安全渠道传输 (Don't transmit via insecure channels)");
    println!("   • 定期验证密钥文件完整性 (Regularly verify keyfile integrity)");
    
    Ok(())
}
