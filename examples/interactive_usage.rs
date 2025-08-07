// examples/interactive_usage.rs

//! # Ferox Encryptor 交互式模式使用示例
//!
//! 本示例展示了如何在代码中集成和使用 Ferox Encryptor 的交互式功能。
//! 虽然交互式模式主要设计为命令行工具，但这里展示了相关的核心功能。
//!
//! *This example demonstrates how to integrate and use Ferox Encryptor's 
//! interactive features in code. While interactive mode is primarily designed 
//! as a command-line tool, this shows the core functionality.*

use anyhow::Result;
use ferox_encryptor::{
    batch::{batch_encrypt_files, BatchConfig},
    keyfile::KeyFile,
    Level,
};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// 演示交互式模式的核心功能
/// 
/// *Demonstrates core functionality of interactive mode*
fn main() -> Result<()> {
    println!("🔐 Ferox Encryptor 交互式功能演示");
    println!("🔐 Ferox Encryptor Interactive Features Demo");
    println!("{}", "=".repeat(60));

    // 演示不同安全级别的选择
    demonstrate_security_levels()?;
    
    // 演示密钥文件生成和使用
    demonstrate_keyfile_usage()?;
    
    // 演示批量配置选项
    demonstrate_batch_configuration()?;
    
    println!("\n🎉 演示完成！");
    println!("🎉 Demo completed!");
    println!("\n💡 要体验完整的交互式界面，请运行:");
    println!("💡 To experience the full interactive interface, run:");
    println!("   ferox_encryptor interactive");

    Ok(())
}

/// 演示安全级别选择
fn demonstrate_security_levels() -> Result<()> {
    println!("\n📊 安全级别选择演示 (Security Level Selection Demo)");
    println!("{}", "-".repeat(50));
    
    let levels = vec![
        (Level::Interactive, "Interactive - 快速 (19 MiB 内存)", "适合频繁访问的文件"),
        (Level::Moderate, "Moderate - 推荐 (64 MiB 内存)", "个人文档、敏感数据的最佳选择"),
        (Level::Paranoid, "Paranoid - 最安全 (256 MiB 内存)", "高度敏感数据、长期存储"),
    ];
    
    for (level, description, use_case) in levels {
        println!("🔒 {:?}", level);
        println!("   📝 描述: {}", description);
        println!("   🎯 适用: {}", use_case);
        println!();
    }
    
    Ok(())
}

/// 演示密钥文件生成和使用
fn demonstrate_keyfile_usage() -> Result<()> {
    println!("🔑 密钥文件功能演示 (Key File Features Demo)");
    println!("{}", "-".repeat(50));
    
    // 生成示例密钥文件
    println!("📝 生成密钥文件...");
    let keyfile = KeyFile::generate();
    
    // 在实际应用中，您会保存到文件
    // keyfile.save_to_file(Path::new("demo.key"))?;
    println!("✅ 密钥文件生成成功");
    
    println!("🛡️ 密钥文件提供的安全优势:");
    println!("   • 双重保护: 需要密码 + 密钥文件");
    println!("   • 抗暴力破解: 即使密码泄露也无法解密");
    println!("   • 便携性: 可以存储在安全的外部设备");
    
    Ok(())
}

/// 演示批量配置选项
fn demonstrate_batch_configuration() -> Result<()> {
    println!("\n📁 批量处理配置演示 (Batch Processing Configuration Demo)");
    println!("{}", "-".repeat(50));
    
    // 创建不同的批量配置示例
    let configs = vec![
        (
            "基本配置",
            BatchConfig {
                level: Level::Moderate,
                force_overwrite: false,
                recursive: false,
                include_patterns: vec![],
                exclude_patterns: vec![],
            }
        ),
        (
            "递归处理配置",
            BatchConfig {
                level: Level::Moderate,
                force_overwrite: false,
                recursive: true,
                include_patterns: vec![],
                exclude_patterns: vec![],
            }
        ),
        (
            "高安全级别配置",
            BatchConfig {
                level: Level::Paranoid,
                force_overwrite: true,
                recursive: true,
                include_patterns: vec![],
                exclude_patterns: vec![],
            }
        ),
    ];
    
    for (name, config) in configs {
        println!("⚙️ {}", name);
        println!("   🔒 安全级别: {:?}", config.level);
        println!("   🔄 递归处理: {}", if config.recursive { "是" } else { "否" });
        println!("   ⚡ 强制覆盖: {}", if config.force_overwrite { "是" } else { "否" });
        println!();
    }
    
    println!("💡 在交互式模式中，这些配置通过友好的菜单进行设置");
    println!("💡 In interactive mode, these configurations are set through friendly menus");
    
    Ok(())
}

/// 演示操作预览功能（模拟）
#[allow(dead_code)]
fn demonstrate_operation_preview() -> Result<()> {
    println!("\n📋 操作预览演示 (Operation Preview Demo)");
    println!("{}", "-".repeat(50));
    
    println!("📋 操作预览 (Operation Preview):");
    println!("   🔧 操作类型: 加密 (Encryption)");
    println!("   📁 文件数量: 3 个");
    println!("   🔒 安全级别: Moderate");
    println!("   🔑 密钥文件: 是 (Yes)");
    println!("   ⚡ 强制覆盖: 否 (No)");
    println!("   📄 文件列表:");
    println!("      • document1.txt");
    println!("      • document2.pdf");
    println!("      • image.jpg");
    println!();
    
    println!("✅ 预览确认后，用户可以选择继续或取消操作");
    println!("✅ After preview confirmation, users can choose to continue or cancel");
    
    Ok(())
}

/// 演示结果显示功能（模拟）
#[allow(dead_code)]
fn demonstrate_result_display() -> Result<()> {
    println!("\n📊 结果显示演示 (Result Display Demo)");
    println!("{}", "-".repeat(50));
    
    println!("✅ 加密完成! (Completed!)");
    println!("📊 处理统计: 3 个文件全部成功处理");
    println!("💾 处理数据量: 15.67 MB");
    println!();
    
    println!("🎯 交互式模式提供详细的操作反馈，包括:");
    println!("   • 成功/失败统计");
    println!("   • 处理的数据量");
    println!("   • 失败文件的详细错误信息");
    println!("   • 针对性的解决建议");
    
    Ok(())
}

/// 演示错误处理和恢复建议
#[allow(dead_code)]
fn demonstrate_error_handling() -> Result<()> {
    println!("\n🚨 错误处理演示 (Error Handling Demo)");
    println!("{}", "-".repeat(50));
    
    println!("❌ 操作失败 (Operation Failed):");
    println!("   📁 /path/to/file.txt");
    println!("   🔍 错误: Permission denied");
    println!("   💡 建议: 尝试使用管理员权限运行，或检查文件权限设置");
    println!();
    
    println!("🔧 交互式模式的智能错误处理:");
    println!("   • 详细的错误描述");
    println!("   • 针对性的解决建议");
    println!("   • 重试选项");
    println!("   • 跳过失败项目继续处理");
    
    Ok(())
}
