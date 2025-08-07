# 更新日志 (Changelog)

本文档记录了 Ferox Encryptor 项目的所有重要更改。

*This document records all notable changes to the Ferox Encryptor project.*

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
项目遵循 [语义化版本控制](https://semver.org/lang/zh-CN/)。

*The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).*

## [未发布] (Unreleased)

### 新增 (Added)
- 完整的中文文档体系，包含英文技术术语补充说明
- 架构设计文档 (ARCHITECTURE.md)
- 开发指南文档 (DEVELOPMENT.md)
- 更新日志文档 (CHANGELOG.md)
- 详细的代码注释，采用中英文双语形式

### 改进 (Changed)
- 优化项目目录结构，提高代码组织性
- 改进用户指南，增加中英文对照
- 统一代码格式，修复所有 Clippy 警告
- 增强错误信息的中文本地化

### 修复 (Fixed)
- 修复所有编译警告和 Clippy 建议
- 清理无用的调试文件 (debug_test.rs)
- 统一代码格式化风格

### 安全 (Security)
- 代码安全审查，确保无安全漏洞
- 依赖项安全检查和更新

## [0.1.0] - 2024-01-XX

### 新增 (Added)
- 基于 AES-256-CTR + HMAC-SHA256 的文件加密功能
- Argon2id 密钥派生，支持三种安全级别
- 命令行界面，支持单文件和批量处理
- 密钥文件支持，提供双重安全保护
- 完整的测试套件，包括单元测试、集成测试、边界测试
- 性能测试和安全测试
- 进度条显示和优雅的中断处理
- 详细的用户文档和 API 文档

### 技术特性 (Technical Features)
- 流式处理，支持大文件加密 (GB 级别)
- 内存安全，使用 zeroize 擦除敏感数据
- 跨平台兼容性 (Windows, macOS, Linux)
- 向后兼容的文件格式设计
- 完整性验证和篡改检测

### 安全特性 (Security Features)
- Encrypt-then-MAC 安全模式
- 密码学安全的随机数生成
- 抗暴力破解的密钥派生
- 常量时间操作防止侧信道攻击

---

## 版本说明 (Version Notes)

### 安全级别 (Security Levels)

- **Interactive**: 适用于频繁访问的文件，快速处理
- **Moderate**: 推荐的默认级别，平衡安全性和性能
- **Paranoid**: 最高安全级别，适用于敏感数据长期存储

### 文件格式版本 (File Format Versions)

当前文件格式版本: **1.0**

文件格式向后兼容，新版本软件可以解密旧版本加密的文件。

*Current file format version: **1.0***

*File format is backward compatible - newer software versions can decrypt files encrypted by older versions.*

### 依赖项版本 (Dependency Versions)

主要依赖项及其版本:

- `aes`: 0.8.4 - AES 加密算法实现
- `argon2`: 0.5.3 - Argon2 密钥派生函数
- `hmac`: 0.12.1 - HMAC 消息认证码
- `clap`: 4.5.8 - 命令行参数解析
- `anyhow`: 1.0.86 - 错误处理
- `indicatif`: 0.17.8 - 进度条显示

### 性能基准 (Performance Benchmarks)

基于测试环境的性能数据:

- **小文件** (< 1MB): 主要时间消耗在密钥派生
- **大文件** (> 100MB): 加密速度 > 100MB/s (取决于硬件)
- **批量处理**: 支持并发处理多个文件

### 已知限制 (Known Limitations)

- 文件名长度限制: 65535 字节 (UTF-8 编码)
- 单文件大小限制: 理论上无限制，实际受文件系统限制
- 内存使用: 固定 4MB 缓冲区 + Argon2 内存需求

### 兼容性 (Compatibility)

- **操作系统**: Windows 10+, macOS 10.15+, Linux (主流发行版)
- **Rust 版本**: 1.70+ (2021 edition)
- **架构**: x86_64, ARM64

---

## 贡献者 (Contributors)

感谢所有为 Ferox Encryptor 项目做出贡献的开发者。

*Thanks to all developers who have contributed to the Ferox Encryptor project.*

## 许可证 (License)

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

*This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.*

---

*更新日志格式参考: [Keep a Changelog](https://keepachangelog.com/)*

*Changelog format reference: [Keep a Changelog](https://keepachangelog.com/)*
