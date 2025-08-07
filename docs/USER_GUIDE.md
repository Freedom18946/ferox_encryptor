# Ferox Encryptor 用户指南 (User Guide)

**一个基于 Rust 的高性能、抗暴力破解的本地文件加密工具的详细使用指南**

*A comprehensive user guide for the high-performance, brute-force resistant local file encryption tool built with Rust*

## 📖 目录 (Table of Contents)

1. [快速开始 (Quick Start)](#快速开始)
2. [基本用法 (Basic Usage)](#基本用法)
3. [高级功能 (Advanced Features)](#高级功能)
4. [安全最佳实践 (Security Best Practices)](#安全最佳实践)
5. [故障排除 (Troubleshooting)](#故障排除)
6. [常见问题 (FAQ)](#常见问题)

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

## 🎯 交互式模式 (推荐新用户)

对于新用户，我们强烈推荐使用交互式模式，它提供了直观的菜单驱动界面：

```bash
# 启动交互式模式
ferox-encryptor interactive
```

### 交互式模式特性

- 🎯 **用户友好的菜单系统** - 清晰的选项导航，无需记忆复杂命令
- 🔍 **操作预览** - 执行前显示详细的操作信息和影响范围
- ✅ **智能验证** - 自动验证文件路径、权限和配置有效性
- 🌍 **双语支持** - 完整的中英文界面，适合不同用户
- 💡 **智能建议** - 根据文件类型和使用场景提供最佳实践建议
- 🛡️ **安全确认** - 重要操作前的多重确认提示
- 📊 **实时反馈** - 详细的进度显示和结果统计

### 交互式模式操作流程

1. **启动程序**: `ferox-encryptor interactive`
2. **选择操作**: 从主菜单选择加密、解密或批量操作
3. **配置参数**: 通过交互式提示设置文件路径、安全级别等
4. **预览确认**: 查看操作预览，确认无误后执行
5. **查看结果**: 获得详细的操作结果和统计信息

### 适用场景

- 🆕 **新用户入门** - 无需学习命令行参数
- 🔧 **复杂配置** - 需要设置多个参数的场景
- 📁 **批量操作** - 处理大量文件时的可视化管理
- 🎓 **学习工具** - 了解各种加密选项和最佳实践

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

## 🎮 交互式模式使用示例

### 示例 1: 加密单个文件

```bash
$ ferox-encryptor interactive

🔐 Ferox Encryptor - 交互式模式 (Interactive Mode)
═══════════════════════════════════════════════════════════
高性能文件加密工具 - 交互式用户界面
High-performance file encryption tool - Interactive UI
═══════════════════════════════════════════════════════════

📋 请选择操作 (Please select an operation):
> 🔒 加密文件 (Encrypt Files)
  🔓 解密文件 (Decrypt Files)
  📁 批量加密目录 (Batch Encrypt Directory)
  📂 批量解密目录 (Batch Decrypt Directory)
  🔑 生成密钥文件 (Generate Key File)
  ❓ 帮助信息 (Help)
  🚪 退出程序 (Exit)

# 选择加密文件后，系统会引导您：
# 1. 输入文件路径
# 2. 选择安全级别
# 3. 配置密钥文件（可选）
# 4. 预览操作
# 5. 确认执行
```

### 示例 2: 批量加密目录

交互式模式特别适合批量操作，因为它提供了：
- 文件过滤模式的可视化配置
- 操作范围的清晰预览
- 详细的结果统计

### 示例 3: 密钥文件管理

交互式模式简化了密钥文件的生成和使用：
- 智能路径建议
- 安全性提醒
- 验证确认

### 交互式模式最佳实践

1. **首次使用**: 建议先使用帮助功能了解各选项
2. **批量操作**: 使用文件模式过滤避免处理不必要的文件
3. **安全配置**: 对重要数据使用密钥文件双重保护
4. **操作确认**: 仔细查看操作预览，确认无误后执行
5. **结果检查**: 关注操作结果统计，及时处理失败项目

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

1. **使用交互式模式**: `ferox-encryptor interactive` - 提供内置帮助和引导
2. 查看命令行帮助：`ferox-encryptor --help`
3. 检查日志输出中的错误信息
4. 参考本指南的故障排除部分
5. 在 GitHub 仓库提交 Issue

### 交互式帮助

在交互式模式中，选择 "❓ 帮助信息 (Help)" 可以获得：
- 功能特性详细说明
- 安全级别选择建议
- 密钥文件使用指导
- 操作导航提示

记住：安全第一，定期备份！