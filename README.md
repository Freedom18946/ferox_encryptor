# Ferox Encryptor

**一个基于 Rust 的高性能、抗暴力破解的本地文件加密工具**

*A high-performance, brute-force resistant local file encryption tool built with Rust*

[![CI](https://github.com/YOUR_USERNAME/ferox_encryptor/actions/workflows/rust.yml/badge.svg)](https://github.com/YOUR_USERNAME/ferox_encryptor/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)

这是一个命令行工具，旨在提供一个安全、快速且用户友好的方式来保护您的本地文件。它使用现代、经过严格审查的加密算法，确保即使在源码泄露的情况下，您的数据依然安全。

*This is a command-line tool designed to provide a secure, fast, and user-friendly way to protect your local files. It uses modern, rigorously reviewed encryption algorithms to ensure your data remains secure even if the source code is compromised.*

## ✨ 主要特性 (Key Features)

-   **极致安全 (Ultimate Security)**: 采用 `Argon2id` 进行密钥派生，有效抵御 GPU 和 ASIC 破解。
    *Uses `Argon2id` for key derivation, effectively resisting GPU and ASIC attacks.*
-   **现代加密 (Modern Encryption)**: 使用 `AES-256-CTR` 进行流式加密，并结合 `HMAC-SHA256` (Encrypt-then-MAC 模式) 保证数据的机密性和完整性。
    *Uses `AES-256-CTR` for streaming encryption, combined with `HMAC-SHA256` (Encrypt-then-MAC mode) to ensure data confidentiality and integrity.*
-   **高性能 (High Performance)**: 为处理大型文件（数GB）而优化，采用流式处理，内存占用极低。
    *Optimized for processing large files (multi-GB), using streaming processing with minimal memory footprint.*
-   **向后兼容 (Backward Compatibility)**: 加密参数存储在文件头中，确保未来的版本能解密旧文件。
    *Encryption parameters stored in file headers, ensuring future versions can decrypt old files.*
-   **可定制的安全等级 (Customizable Security Levels)**: 提供多个安全级别选项，允许用户在性能和安全性之间进行权衡。
    *Provides multiple security level options, allowing users to balance performance and security.*
-   **健壮性 (Robustness)**: 能够优雅地处理 `Ctrl+C` 中断，自动清理不完整的输出文件。
    *Gracefully handles `Ctrl+C` interrupts, automatically cleaning up incomplete output files.*

## ⚙️ 安装

确保你已经安装了 [Rust 工具链](https://www.rust-lang.org/tools/install)。

从源码构建并安装：
```bash
# 克隆仓库 (或者直接下载源码)
# git clone [https://github.com/YOUR_USERNAME/ferox_encryptor.git](https://github.com/YOUR_USERNAME/ferox_encryptor.git)
# cd ferox_encryptor

# 构建并安装到你的 cargo 二进制路径下
cargo install --path .
```
之后，你就可以在任何地方使用 `ferox-encryptor` 命令了。

## 🚀 使用方法

### 加密文件
```bash
ferox-encryptor encrypt <文件路径> [选项]
```
**示例:**
```bash
# 使用默认的中等级别加密文件
ferox-encryptor encrypt "my secret document.docx"

# 使用最高的“偏执”安全级别加密
ferox-encryptor encrypt "my secret document.docx" --level paranoid

# 如果目标文件已存在，强制覆盖
ferox-encryptor encrypt "my secret document.docx" --force
```

### 解密文件
```bash
ferox-encryptor decrypt <加密文件路径>
```
**示例:**
```bash
ferox-encryptor decrypt "my secret document.docx.feroxcrypt"
```

## 📁 项目结构 (Project Structure)

```
ferox_encryptor/
├── src/                    # 源代码 (Source code)
│   ├── main.rs            # 主程序入口 (Main program entry)
│   ├── lib.rs             # 库接口 (Library interface)
│   ├── encrypt.rs         # 加密模块 (Encryption module)
│   ├── decrypt.rs         # 解密模块 (Decryption module)
│   ├── batch.rs           # 批量处理 (Batch processing)
│   ├── keyfile.rs         # 密钥文件 (Keyfile management)
│   └── constants.rs       # 常量定义 (Constants)
├── tests/                  # 测试文件 (Test files)
├── docs/                   # 文档目录 (Documentation)
├── examples/               # 示例代码 (Example code)
└── README.md              # 项目说明 (Project description)
```

## 📚 文档 (Documentation)

-   [**用户指南 (User Guide)**](./docs/USER_GUIDE.md): 详细的使用说明和示例。
    *Detailed usage instructions and examples.*
-   [**API 文档 (API Documentation)**](./docs/API.md): 库接口和编程示例。
    *Library interface and programming examples.*
-   [**安全指南 (Security Guide)**](./docs/SECURITY_GUIDE.md): 深入了解工具的安全设计和威胁模型。
    *In-depth understanding of the tool's security design and threat model.*
-   [**最佳实践 (Best Practices)**](./docs/BEST_PRACTICES.md): 如何最安全、最有效地使用本工具。
    *How to use this tool most securely and effectively.*
-   [**架构文档 (Architecture)**](./docs/ARCHITECTURE.md): 系统架构和设计文档。
    *System architecture and design documentation.*
-   [**开发指南 (Development Guide)**](./docs/DEVELOPMENT.md): 开发环境设置和贡献指南。
    *Development environment setup and contribution guide.*

## 🤝 贡献 (Contributing)

我们欢迎各种形式的贡献！请查看 [开发指南](./docs/DEVELOPMENT.md) 了解如何参与项目开发。

*We welcome contributions of all kinds! Please see the [Development Guide](./docs/DEVELOPMENT.md) to learn how to participate in project development.*

## 📄 授权协议 (License)

本项目采用 [MIT](LICENSE) 授权协议。

*This project is licensed under the [MIT](LICENSE) license.*
