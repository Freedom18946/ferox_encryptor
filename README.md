# Ferox Encryptor

**一个基于 Rust 的高性能、抗暴力破解的本地文件加密工具。**

[![CI](https://github.com/YOUR_USERNAME/ferox_encryptor/actions/workflows/rust.yml/badge.svg)](https://github.com/YOUR_USERNAME/ferox_encryptor/actions/workflows/rust.yml)

这是一个命令行工具，旨在提供一个安全、快速且用户友好的方式来保护您的本地文件。它使用现代、经过严格审查的加密算法，确保即使在源码泄露的情况下，您的数据依然安全。

## ✨ 主要特性

-   **极致安全**: 采用 `Argon2id` 进行密钥派生，有效抵御 GPU 和 ASIC 破解。
-   **现代加密**: 使用 `AES-256-CTR` 进行流式加密，并结合 `HMAC-SHA256` (Encrypt-then-MAC 模式) 保证数据的机密性和完整性。
-   **高性能**: 为处理大型文件（数GB）而优化，采用流式处理，内存占用极低。
-   **向后兼容**: 加密参数存储在文件头中，确保未来的版本能解密旧文件。
-   **可定制的安全等级**: 提供多个安全级别选项，允许用户在性能和安全性之间进行权'衡。
-   **健壮性**: 能够优雅地处理 `Ctrl+C` 中断，自动清理不完整的输出文件。

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

## 📚 文档

-   [**用户指南**](./docs/USER_GUIDE.md): 详细的使用说明和示例。
-   [**安全指南**](./docs/SECURITY_GUIDE.md): 深入了解工具的安全设计和威胁模型。
-   [**最佳实践**](./docs/BEST_PRACTICES.md): 如何最安全、最有效地使用本工具。

## 授权协议

本项目采用 [MIT](LICENSE) 授权协议。
