# Ferox Encryptor 架构设计文档 (Architecture Design Document)

**系统架构、模块设计和技术实现的详细说明**

*Detailed documentation of system architecture, module design, and technical implementation*

## 📋 目录 (Table of Contents)

1. [系统概览 (System Overview)](#系统概览)
2. [核心架构 (Core Architecture)](#核心架构)
3. [模块设计 (Module Design)](#模块设计)
4. [数据流程 (Data Flow)](#数据流程)
5. [安全设计 (Security Design)](#安全设计)
6. [性能考虑 (Performance Considerations)](#性能考虑)

## 🏗️ 系统概览 (System Overview)

Ferox Encryptor 是一个基于 Rust 的高性能文件加密工具，采用模块化设计，确保代码的可维护性和可扩展性。

*Ferox Encryptor is a high-performance file encryption tool built with Rust, featuring a modular design that ensures code maintainability and extensibility.*

### 核心特性 (Core Features)

- **安全性 (Security)**: 使用 AES-256-CTR + HMAC-SHA256 + Argon2id
- **性能 (Performance)**: 流式处理，支持大文件（GB级别）
- **可用性 (Usability)**: 命令行界面，批量处理支持
- **可靠性 (Reliability)**: 完整性验证，优雅的错误处理

## 🏛️ 核心架构 (Core Architecture)

```
┌─────────────────────────────────────────────────────────────┐
│                    Ferox Encryptor                         │
├─────────────────────────────────────────────────────────────┤
│  CLI Layer (命令行层)                                       │
│  ├── main.rs - 程序入口和参数解析                           │
│  ├── 用户交互处理 (User Interaction)                        │
│  └── 信号处理 (Signal Handling)                            │
├─────────────────────────────────────────────────────────────┤
│  Business Logic Layer (业务逻辑层)                          │
│  ├── batch.rs - 批量处理逻辑                               │
│  ├── encrypt.rs - 加密流程                                 │
│  ├── decrypt.rs - 解密流程                                 │
│  └── keyfile.rs - 密钥文件管理                             │
├─────────────────────────────────────────────────────────────┤
│  Core Layer (核心层)                                        │
│  ├── constants.rs - 系统常量                               │
│  ├── lib.rs - 公共接口定义                                 │
│  └── 类型定义和错误处理                                     │
├─────────────────────────────────────────────────────────────┤
│  Cryptographic Layer (密码学层)                             │
│  ├── AES-256-CTR (对称加密)                                │
│  ├── HMAC-SHA256 (消息认证)                                │
│  ├── Argon2id (密钥派生)                                   │
│  └── 安全随机数生成                                         │
└─────────────────────────────────────────────────────────────┘
```

## 🧩 模块设计 (Module Design)

### 1. 主程序模块 (main.rs)
**职责**: 程序入口点，命令行参数解析，用户交互

**核心功能**:
- CLI 参数解析 (使用 clap)
- 密码安全输入 (使用 rpassword)
- 信号处理 (Ctrl+C 优雅退出)
- 日志初始化

### 2. 加密模块 (encrypt.rs)
**职责**: 文件加密的完整流程实现

**核心流程**:
1. 输入验证 (文件存在性、权限检查)
2. 随机数生成 (盐值、IV)
3. 密钥派生 (Argon2id)
4. 文件头构建 (元数据存储)
5. 流式加密 (AES-256-CTR)
6. 完整性保护 (HMAC-SHA256)

### 3. 解密模块 (decrypt.rs)
**职责**: 文件解密的完整流程实现

**核心流程**:
1. 文件格式验证
2. 文件头解析 (提取元数据)
3. 密钥派生 (使用文件中的参数)
4. 流式解密 (AES-256-CTR)
5. 完整性验证 (HMAC-SHA256)
6. 原始文件恢复

### 4. 批量处理模块 (batch.rs)
**职责**: 多文件和目录的批量操作

**核心功能**:
- 目录遍历 (支持递归)
- 文件过滤 (glob 模式匹配)
- 并发处理 (可配置)
- 进度报告和错误统计

### 5. 密钥文件模块 (keyfile.rs)
**职责**: 密钥文件的生成、验证和使用

**核心功能**:
- 安全随机密钥生成
- 密钥文件格式验证
- 密码与密钥文件的安全组合
- 密钥派生增强

### 6. 常量模块 (constants.rs)
**职责**: 系统级常量定义

**包含内容**:
- 密码学参数 (密钥长度、缓冲区大小)
- 文件格式常量 (扩展名、头部结构)
- 安全配置 (Argon2 参数范围)

## 🔄 数据流程 (Data Flow)

### 加密流程 (Encryption Flow)
```
用户输入 → 参数验证 → 密钥派生 → 文件头生成 → 流式加密 → HMAC计算 → 输出文件
    ↓           ↓           ↓           ↓           ↓           ↓
  密码+文件   安全检查    Argon2id    元数据存储   AES-CTR    完整性保护
```

### 解密流程 (Decryption Flow)
```
加密文件 → 格式验证 → 头部解析 → 密钥派生 → 流式解密 → HMAC验证 → 原始文件
    ↓           ↓           ↓           ↓           ↓           ↓
  输入验证    结构检查    参数提取    Argon2id    AES-CTR    完整性验证
```

## 🛡️ 安全设计 (Security Design)

### 密码学原语选择
- **AES-256-CTR**: 业界标准的对称加密算法
- **HMAC-SHA256**: 强密码学哈希的消息认证码
- **Argon2id**: 抗 GPU/ASIC 攻击的密钥派生函数

### 安全实践
1. **Encrypt-then-MAC**: 先加密后认证的安全模式
2. **密钥分离**: 加密密钥和认证密钥独立派生
3. **内存安全**: 使用 zeroize 安全擦除敏感数据
4. **随机性**: 使用操作系统提供的密码学安全随机数

### 威胁模型
- **暴力破解**: Argon2id 高计算成本防护
- **彩虹表攻击**: 随机盐值防护
- **篡改攻击**: HMAC 完整性验证防护
- **侧信道攻击**: 常量时间操作和内存擦除

## ⚡ 性能考虑 (Performance Considerations)

### 内存管理
- **流式处理**: 固定大小缓冲区 (4MB)
- **零拷贝**: 尽可能避免不必要的数据复制
- **内存池**: 重用缓冲区减少分配开销

### I/O 优化
- **缓冲读写**: 使用 BufReader/BufWriter
- **异步处理**: 支持大文件的非阻塞操作
- **进度显示**: 实时反馈处理进度

### 并发设计
- **线程安全**: 使用 Arc<Mutex<T>> 共享状态
- **批量处理**: 支持多文件并行处理
- **资源控制**: 限制并发数量防止资源耗尽

### 性能基准
- **小文件** (< 1MB): 处理时间主要由密钥派生决定
- **大文件** (> 100MB): I/O 和加密计算并行进行
- **批量处理**: 文件发现和处理流水线化

## 🔧 扩展性设计 (Extensibility Design)

### 模块化接口
- 清晰的模块边界和职责分离
- 标准化的错误处理和返回类型
- 可配置的安全参数和处理选项

### 未来扩展点
- 支持更多加密算法 (ChaCha20-Poly1305)
- 网络传输加密支持
- 图形用户界面 (GUI)
- 云存储集成

---

*本文档描述了 Ferox Encryptor 的核心架构设计，为开发者提供了系统理解和扩展的技术基础。*

*This document describes the core architectural design of Ferox Encryptor, providing developers with the technical foundation for understanding and extending the system.*
