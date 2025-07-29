# Ferox Encryptor 用户指南

## 📖 目录

1. [快速开始](#快速开始)
2. [基本用法](#基本用法)
3. [高级功能](#高级功能)
4. [安全最佳实践](#安全最佳实践)
5. [故障排除](#故障排除)
6. [常见问题](#常见问题)

## 🚀 快速开始

### 安装

确保已安装 [Rust 工具链](https://www.rust-lang.org/tools/install)，然后：

```bash
# 从源码构建并安装
cargo install --path .

# 或者直接运行
cargo run -- --help
```

### 第一次使用

```bash
# 加密单个文件
ferox-encryptor encrypt "我的文档.docx"

# 解密文件
ferox-encryptor decrypt "我的文档.docx.feroxcrypt"
```

## 📝 基本用法

### 文件加密

```bash
# 基本加密（使用默认的中等安全级别）
ferox-encryptor encrypt "secret.txt"

# 指定安全级别
ferox-encryptor encrypt "secret.txt" --level paranoid

# 强制覆盖已存在的加密文件
ferox-encryptor encrypt "secret.txt" --force
```

### 文件解密

```bash
# 基本解密
ferox-encryptor decrypt "secret.txt.feroxcrypt"
```

### 安全级别说明

| 级别 | 内存使用 | 时间成本 | 适用场景 |
|------|----------|----------|----------|
| `interactive` | 19 MiB | 快速 | 频繁访问的文件、开发测试 |
| `moderate` | 64 MiB | 中等 | **推荐默认**，个人文档、敏感数据 |
| `paranoid` | 256 MiB | 较慢 | 高度敏感数据、长期存储 |

## 🔧 高级功能

### 批量处理

#### 批量加密目录

```bash
# 加密目录中的所有文件
ferox-encryptor batch-encrypt "/path/to/documents"

# 递归处理子目录
ferox-encryptor batch-encrypt "/path/to/documents" --recursive

# 只处理特定类型的文件
ferox-encryptor batch-encrypt "/path/to/documents" --include "*.txt" --include "*.doc"

# 排除特定文件
ferox-encryptor batch-encrypt "/path/to/documents" --exclude "*.tmp" --exclude "*.bak"

# 使用高安全级别
ferox-encryptor batch-encrypt "/path/to/documents" --level paranoid
```

#### 批量解密目录

```bash
# 解密目录中的所有 .feroxcrypt 文件
ferox-encryptor batch-decrypt "/path/to/encrypted"

# 递归处理子目录
ferox-encryptor batch-decrypt "/path/to/encrypted" --recursive
```

### 密钥文件支持

密钥文件提供额外的安全层，即使密码泄露，没有密钥文件也无法解密。

#### 生成密钥文件

```bash
# 生成新的密钥文件
ferox-encryptor generate-key "my-secret.key"
```

#### 使用密钥文件加密

```bash
# 使用密钥文件加密
ferox-encryptor encrypt "secret.txt" --keyfile "my-secret.key"

# 批量加密时使用密钥文件
ferox-encryptor batch-encrypt "/documents" --keyfile "my-secret.key"
```

#### 使用密钥文件解密

```bash
# 使用密钥文件解密
ferox-encryptor decrypt "secret.txt.feroxcrypt" --keyfile "my-secret.key"

# 批量解密时使用密钥文件
ferox-encryptor batch-decrypt "/encrypted" --keyfile "my-secret.key"
```

## 🛡️ 安全最佳实践

### 密码安全

1. **使用强密码**
   - 至少 12 个字符
   - 包含大小写字母、数字和特殊字符
   - 避免使用常见词汇或个人信息

2. **密码管理**
   - 使用密码管理器生成和存储密码
   - 不要在多个地方重复使用相同密码
   - 定期更换重要文件的密码

### 密钥文件安全

1. **存储位置**
   - 将密钥文件存储在与加密文件不同的位置
   - 考虑使用 USB 驱动器或云存储
   - 制作多个备份副本

2. **访问控制**
   - 设置适当的文件权限（仅所有者可读）
   - 避免通过不安全的渠道传输密钥文件

### 备份策略

1. **多重备份**
   - 保持加密文件的多个副本
   - 分别存储在不同的物理位置
   - 定期验证备份的完整性

2. **恢复测试**
   - 定期测试解密过程
   - 确保密码和密钥文件的可用性

### 安全级别选择

- **日常文档**: `moderate` 级别已足够
- **财务记录**: 使用 `paranoid` 级别
- **临时文件**: `interactive` 级别可以提高效率
- **长期存档**: 推荐 `paranoid` 级别

## 🔍 故障排除

### 常见错误及解决方案

#### "Authentication failed" 错误

**原因**: 密码错误或文件已损坏

**解决方案**:
1. 确认密码输入正确
2. 检查是否需要密钥文件
3. 验证文件完整性
4. 尝试从备份恢复

#### "File already exists" 错误

**原因**: 目标文件已存在

**解决方案**:
```bash
# 使用 --force 标志强制覆盖
ferox-encryptor encrypt "file.txt" --force
```

#### "Permission denied" 错误

**原因**: 文件权限不足

**解决方案**:
```bash
# 检查文件权限
ls -la "file.txt"

# 修改权限（如果需要）
chmod 644 "file.txt"
```

#### 内存不足错误

**原因**: 系统内存不足以支持选择的安全级别

**解决方案**:
```bash
# 使用较低的安全级别
ferox-encryptor encrypt "file.txt" --level interactive
```

### 性能优化

#### 大文件处理

- 确保有足够的磁盘空间（至少是原文件大小的 2 倍）
- 使用 SSD 存储可以显著提高性能
- 考虑使用 `interactive` 级别以提高速度

#### 批量处理优化

```bash
# 使用文件模式过滤减少处理时间
ferox-encryptor batch-encrypt "/large-dir" --include "*.important"

# 避免递归处理不必要的子目录
ferox-encryptor batch-encrypt "/specific-dir" # 不使用 --recursive
```

## ❓ 常见问题

### Q: 忘记密码怎么办？

**A**: 很遗憾，如果忘记密码且没有备份，文件将无法恢复。这是设计的安全特性。建议：
- 使用密码管理器
- 创建密码提示（但不要太明显）
- 考虑使用密钥文件作为额外保护

### Q: 可以更改已加密文件的密码吗？

**A**: 需要先解密文件，然后用新密码重新加密：
```bash
ferox-encryptor decrypt "file.txt.feroxcrypt"
ferox-encryptor encrypt "file.txt" # 输入新密码
```

### Q: 加密文件可以在不同操作系统间使用吗？

**A**: 是的，Ferox Encryptor 的加密格式是跨平台的，可以在 Windows、macOS 和 Linux 之间自由使用。

### Q: 如何验证文件完整性？

**A**: Ferox Encryptor 内置了完整性验证。如果文件被篡改，解密时会自动检测并报错。

### Q: 可以加密整个目录结构吗？

**A**: 使用批量处理功能：
```bash
ferox-encryptor batch-encrypt "/directory" --recursive
```

### Q: 密钥文件丢失怎么办？

**A**: 如果加密时使用了密钥文件，丢失密钥文件将导致无法解密。建议：
- 制作多个密钥文件备份
- 存储在不同位置
- 考虑使用云存储备份

### Q: 如何选择合适的安全级别？

**A**: 根据数据敏感性和性能需求：
- 个人文档：`moderate`
- 商业机密：`paranoid`
- 临时文件：`interactive`
- 不确定时：使用默认的 `moderate`

### Q: 批量处理时部分文件失败怎么办？

**A**: 工具会显示详细的失败信息。常见原因：
- 文件权限问题
- 磁盘空间不足
- 文件正在被其他程序使用

检查日志输出，解决具体问题后重新运行。

## 📞 获取帮助

如果遇到问题：

1. 查看命令行帮助：`ferox-encryptor --help`
2. 检查日志输出中的错误信息
3. 参考本指南的故障排除部分
4. 在 GitHub 仓库提交 Issue

记住：安全第一，定期备份！