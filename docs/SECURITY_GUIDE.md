# Ferox Encryptor 安全指南

## 🔒 安全概述

Ferox Encryptor 采用现代密码学最佳实践，提供军用级别的文件加密保护。本指南详细说明了工具的安全特性、威胁模型和最佳使用实践。

## 🛡️ 密码学设计

### 核心算法

- **密钥派生**: Argon2id (RFC 9106)
  - 抗 GPU/ASIC 攻击
  - 内存困难函数
  - 可配置的计算成本

- **对称加密**: AES-256-CTR
  - NIST 标准算法
  - 256位密钥长度
  - 计数器模式支持流式处理

- **消息认证**: HMAC-SHA256
  - 防止篡改攻击
  - Encrypt-then-MAC 模式
  - 256位认证标签

### 安全级别详解

#### Interactive 级别 (交互式)
```
内存成本: 19 MiB
时间成本: 2 次迭代
并行度: 1
适用场景: 频繁访问的文件，开发测试
安全强度: 中等
```

#### Moderate 级别 (中等) - **推荐**
```
内存成本: 64 MiB  
时间成本: 3 次迭代
并行度: 1
适用场景: 个人文档，敏感数据
安全强度: 高
```

#### Paranoid 级别 (偏执)
```
内存成本: 256 MiB
时间成本: 4 次迭代
并行度: 1
适用场景: 极度敏感数据，长期存储
安全强度: 极高
```

## 🎯 威胁模型

### 防护的威胁

✅ **暴力破解攻击**
- Argon2id 使密码破解在计算上不可行
- 高内存需求阻止大规模并行攻击

✅ **字典攻击**
- 强密钥派生函数大幅增加攻击成本
- 唯一盐值防止彩虹表攻击

✅ **数据篡改**
- HMAC-SHA256 确保数据完整性
- 任何修改都会被检测到

✅ **密文分析**
- AES-256-CTR 提供语义安全
- 随机 IV 确保相同明文产生不同密文

✅ **侧信道攻击**
- 使用常数时间比较函数
- 内存安全擦除敏感数据

### 不防护的威胁

❌ **密码泄露**
- 如果密码被泄露，加密无法提供保护
- 需要用户选择强密码

❌ **恶意软件**
- 无法防护运行时的恶意软件攻击
- 需要安全的操作环境

❌ **物理访问**
- 无法防护直接的物理访问
- 需要额外的物理安全措施

❌ **元数据泄露**
- 文件大小信息可能泄露
- 访问模式可能被观察

## 🔐 密码安全最佳实践

### 强密码要求

**最低要求:**
- 至少 12 个字符
- 包含大小写字母、数字、特殊字符
- 避免常见词汇和个人信息

**推荐做法:**
- 使用密码管理器生成随机密码
- 采用密码短语 (passphrase) 方式
- 定期更换重要文件的密码

**示例强密码:**
```
❌ 弱密码: password123, 生日+姓名
✅ 强密码: Tr0ub4dor&3, correct-horse-battery-staple
✅ 密码短语: MyDog#Loves2Run!InThe$Park
```

### 密码存储

**绝对不要:**
- 将密码写在纸上或文本文件中
- 在浏览器中保存加密文件的密码
- 通过不安全渠道传输密码

**推荐做法:**
- 使用专业密码管理器 (1Password, Bitwarden, KeePass)
- 启用密码管理器的主密码
- 定期备份密码数据库

## 🚀 操作安全指南

### 加密前准备

1. **环境检查**
   ```bash
   # 确保系统安全
   sudo apt update && sudo apt upgrade
   
   # 检查磁盘空间
   df -h
   
   # 验证文件完整性
   sha256sum important_file.txt
   ```

2. **备份策略**
   - 始终保留原文件备份
   - 使用不同存储介质
   - 验证备份完整性

3. **网络隔离**
   - 在离线环境中进行敏感文件加密
   - 断开网络连接防止数据泄露

### 加密操作

```bash
# 使用最高安全级别
ferox-encryptor encrypt sensitive_document.pdf --level paranoid

# 强制覆盖已存在的加密文件
ferox-encryptor encrypt document.txt --force

# 批量加密目录
ferox-encryptor batch-encrypt /path/to/directory --level moderate
```

### 解密操作

```bash
# 标准解密
ferox-encryptor decrypt document.txt.feroxcrypt

# 批量解密
ferox-encryptor batch-decrypt /path/to/encrypted/files
```

### 安全清理

```bash
# 安全删除原文件 (Linux)
shred -vfz -n 3 original_file.txt

# 安全删除原文件 (macOS)
rm -P original_file.txt

# 清理系统缓存
sync && echo 3 > /proc/sys/vm/drop_caches
```

## 🔍 安全验证

### 加密文件验证

```bash
# 检查文件格式
file encrypted_file.feroxcrypt

# 验证文件头
hexdump -C encrypted_file.feroxcrypt | head -5

# 检查文件大小合理性
ls -la original_file.txt encrypted_file.feroxcrypt
```

### 完整性检查

```bash
# 加密前计算哈希
sha256sum document.txt > document.txt.sha256

# 解密后验证
sha256sum -c document.txt.sha256
```

## ⚠️ 安全警告

### 关键注意事项

🚨 **密码丢失 = 数据永久丢失**
- 没有密码恢复机制
- 没有后门或主密钥
- 请务必安全保存密码

🚨 **中断处理**
- Ctrl+C 会自动清理临时文件
- 确保操作完成后验证结果
- 避免在关键时刻中断操作

🚨 **存储安全**
- 加密文件和原文件分开存储
- 使用不同的存储介质
- 定期测试恢复流程

### 常见安全错误

❌ **错误做法:**
```bash
# 在同一目录保存原文件和加密文件
ferox-encryptor encrypt document.txt
# document.txt 和 document.txt.feroxcrypt 都在同一位置

# 使用弱密码
echo "123456" | ferox-encryptor encrypt secret.txt

# 不验证加密结果
ferox-encryptor encrypt important.pdf && rm important.pdf
```

✅ **正确做法:**
```bash
# 加密后移动到安全位置
ferox-encryptor encrypt document.txt --level moderate
mv document.txt.feroxcrypt /secure/storage/
shred -vfz -n 3 document.txt

# 使用强密码并验证
ferox-encryptor encrypt important.pdf --level paranoid
# 手动输入强密码
ferox-encryptor decrypt important.pdf.feroxcrypt
# 验证解密结果正确后再删除原文件
```

## 🛠️ 高级安全配置

### 系统加固

```bash
# 禁用交换文件 (防止密码写入磁盘)
sudo swapoff -a

# 设置安全的 umask
umask 077

# 清理命令历史
history -c && history -w
```

### 环境变量安全

```bash
# 避免在命令行传递密码
export FEROX_PASSWORD="your_secure_password"
ferox-encryptor encrypt document.txt
unset FEROX_PASSWORD
```

### 自动化脚本安全

```bash
#!/bin/bash
set -euo pipefail

# 安全的批量加密脚本
SECURE_DIR="/tmp/secure_$$"
mkdir -p "$SECURE_DIR"
chmod 700 "$SECURE_DIR"

trap 'rm -rf "$SECURE_DIR"' EXIT

# 加密操作
ferox-encryptor batch-encrypt /source/directory --level moderate

echo "加密完成，临时文件已清理"
```

## 📋 安全检查清单

### 加密前检查
- [ ] 选择适当的安全级别
- [ ] 准备强密码
- [ ] 备份原文件
- [ ] 检查磁盘空间
- [ ] 确保网络安全

### 加密后检查
- [ ] 验证加密文件存在
- [ ] 检查文件大小合理
- [ ] 测试解密功能
- [ ] 安全删除原文件
- [ ] 备份加密文件

### 定期维护
- [ ] 更新软件版本
- [ ] 测试恢复流程
- [ ] 检查密码强度
- [ ] 审查访问日志
- [ ] 更新备份策略

## 🆘 应急响应

### 密码泄露处理

1. **立即行动**
   - 更换所有相关密码
   - 重新加密受影响文件
   - 通知相关人员

2. **损害评估**
   - 确定泄露范围
   - 评估数据敏感性
   - 记录事件详情

3. **预防措施**
   - 加强密码策略
   - 增加监控措施
   - 更新安全流程

### 文件损坏处理

1. **验证损坏**
   ```bash
   ferox-encryptor decrypt corrupted.feroxcrypt
   # 如果报告认证失败，文件可能已损坏
   ```

2. **恢复尝试**
   - 检查备份文件
   - 尝试不同密码
   - 联系技术支持

3. **预防措施**
   - 实施多重备份
   - 定期完整性检查
   - 使用错误检测码

---

**记住：安全是一个过程，不是一个产品。定期审查和更新您的安全实践。**