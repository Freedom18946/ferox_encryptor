# Ferox Encryptor 开发指南 (Development Guide)

**开发环境设置、代码贡献和项目维护的完整指南**

*Complete guide for development environment setup, code contribution, and project maintenance*

## 📋 目录 (Table of Contents)

1. [开发环境设置 (Development Setup)](#开发环境设置)
2. [项目结构 (Project Structure)](#项目结构)
3. [代码规范 (Code Standards)](#代码规范)
4. [测试指南 (Testing Guide)](#测试指南)
5. [贡献流程 (Contribution Process)](#贡献流程)
6. [发布流程 (Release Process)](#发布流程)

## 🛠️ 开发环境设置 (Development Setup)

### 前置要求 (Prerequisites)

```bash
# 安装 Rust 工具链 (Install Rust toolchain)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 安装必要的组件 (Install required components)
rustup component add clippy rustfmt
rustup component add llvm-tools-preview

# 安装开发工具 (Install development tools)
cargo install cargo-audit
cargo install cargo-tarpaulin  # 代码覆盖率工具
cargo install cargo-watch     # 自动重新编译
```

### 项目克隆和构建 (Clone and Build)

```bash
# 克隆项目 (Clone project)
git clone <repository-url>
cd ferox_encryptor

# 检查代码质量 (Check code quality)
cargo check
cargo clippy -- -D warnings
cargo fmt --check

# 运行测试 (Run tests)
cargo test
cargo test --release  # 发布模式测试

# 构建项目 (Build project)
cargo build
cargo build --release
```

### 开发工具配置 (Development Tools Configuration)

#### VS Code 配置
```json
// .vscode/settings.json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.cargo.features": "all",
    "editor.formatOnSave": true,
    "files.trimTrailingWhitespace": true
}
```

#### Git 钩子 (Git Hooks)
```bash
# 设置 pre-commit 钩子
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/sh
cargo fmt --check
cargo clippy -- -D warnings
cargo test
EOF
chmod +x .git/hooks/pre-commit
```

## 📁 项目结构 (Project Structure)

```
ferox_encryptor/
├── src/                    # 源代码目录
│   ├── main.rs            # 主程序入口
│   ├── lib.rs             # 库接口定义
│   ├── constants.rs       # 系统常量
│   ├── encrypt.rs         # 加密模块
│   ├── decrypt.rs         # 解密模块
│   ├── batch.rs           # 批量处理模块
│   └── keyfile.rs         # 密钥文件模块
├── tests/                  # 集成测试
│   ├── integration_tests.rs
│   ├── batch_tests.rs
│   ├── edge_case_tests.rs
│   ├── performance_tests.rs
│   └── security_tests.rs
├── docs/                   # 文档目录
│   ├── USER_GUIDE.md      # 用户指南
│   ├── API.md             # API 文档
│   ├── SECURITY_GUIDE.md  # 安全指南
│   ├── BEST_PRACTICES.md  # 最佳实践
│   ├── ARCHITECTURE.md    # 架构文档
│   └── DEVELOPMENT.md     # 开发指南
├── examples/               # 示例代码 (计划中)
├── benches/               # 性能基准测试 (计划中)
├── Cargo.toml             # 项目配置
├── Cargo.lock             # 依赖锁定
├── README.md              # 项目说明
├── LICENSE                # 许可证
└── SECURITY_AUDIT.md      # 安全审计报告
```

## 📝 代码规范 (Code Standards)

### Rust 代码风格

1. **格式化**: 使用 `cargo fmt` 自动格式化
2. **Linting**: 使用 `cargo clippy` 进行代码检查
3. **命名规范**: 遵循 Rust 官方命名约定

### 注释规范

```rust
//! # 模块级文档注释
//! 
//! 模块的详细描述，包括用途和使用方法。
//! 
//! *Module-level documentation comment with detailed description.*

/// # 函数文档注释 (Function Documentation)
/// 
/// 函数的详细说明，包括参数、返回值和示例。
/// 
/// *Detailed function description including parameters, return values, and examples.*
/// 
/// # 参数 (Parameters)
/// 
/// * `param1` - 参数1的描述 (Description of parameter 1)
/// * `param2` - 参数2的描述 (Description of parameter 2)
/// 
/// # 返回值 (Returns)
/// 
/// 返回值的描述 (Description of return value)
/// 
/// # 示例 (Example)
/// 
/// ```rust
/// let result = function_name(param1, param2)?;
/// ```
pub fn function_name(param1: Type1, param2: Type2) -> Result<ReturnType> {
    // 行内注释使用中文，关键技术术语保留英文
    // Inline comments in Chinese, keeping key technical terms in English
    todo!()
}
```

### 错误处理规范

```rust
use anyhow::{Context, Result, bail};

// 使用 anyhow 进行错误处理
pub fn example_function() -> Result<()> {
    // 使用 context 提供中文错误信息
    std::fs::read("file.txt")
        .context("无法读取文件 file.txt")?;
    
    // 使用 bail! 宏直接返回错误
    if condition {
        bail!("条件不满足: {}", reason);
    }
    
    Ok(())
}
```

## 🧪 测试指南 (Testing Guide)

### 测试分类

1. **单元测试**: 在各模块内部的 `#[cfg(test)]` 模块
2. **集成测试**: 在 `tests/` 目录下的独立文件
3. **性能测试**: 标记为 `#[ignore]` 的长时间运行测试
4. **安全测试**: 专门测试安全相关功能

### 测试命令

```bash
# 运行所有测试 (Run all tests)
cargo test

# 运行特定测试 (Run specific tests)
cargo test test_encryption
cargo test batch_tests

# 运行性能测试 (Run performance tests)
cargo test --release -- --ignored

# 生成测试覆盖率报告 (Generate coverage report)
cargo tarpaulin --out Html
```

### 测试编写规范

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_function_name() -> Result<()> {
        // 准备测试数据 (Setup test data)
        let temp_dir = TempDir::new()?;
        let test_file = temp_dir.path().join("test.txt");
        
        // 执行测试 (Execute test)
        let result = function_under_test(&test_file)?;
        
        // 验证结果 (Verify results)
        assert_eq!(result.len(), expected_length);
        assert!(result.contains("expected_content"));
        
        Ok(())
    }
}
```

## 🤝 贡献流程 (Contribution Process)

### 1. 问题报告 (Issue Reporting)

- 使用 GitHub Issues 报告 bug 或提出功能请求
- 提供详细的重现步骤和环境信息
- 使用适当的标签分类问题

### 2. 代码贡献 (Code Contribution)

```bash
# 1. Fork 项目并创建分支
git checkout -b feature/new-feature

# 2. 进行开发
# 编写代码、测试、文档

# 3. 提交前检查
cargo fmt
cargo clippy -- -D warnings
cargo test

# 4. 提交更改
git add .
git commit -m "feat: 添加新功能的简短描述"

# 5. 推送并创建 Pull Request
git push origin feature/new-feature
```

### 3. 提交信息规范 (Commit Message Convention)

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

类型 (Types):
- `feat`: 新功能
- `fix`: 错误修复
- `docs`: 文档更新
- `style`: 代码格式化
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建过程或辅助工具的变动

### 4. Pull Request 检查清单

- [ ] 代码通过所有测试
- [ ] 代码符合格式规范
- [ ] 添加了适当的测试
- [ ] 更新了相关文档
- [ ] 提交信息符合规范

## 🚀 发布流程 (Release Process)

### 版本号规范 (Semantic Versioning)

遵循语义化版本控制 (SemVer): `MAJOR.MINOR.PATCH`

- **MAJOR**: 不兼容的 API 更改
- **MINOR**: 向后兼容的功能添加
- **PATCH**: 向后兼容的错误修复

### 发布步骤

```bash
# 1. 更新版本号
# 编辑 Cargo.toml 中的 version 字段

# 2. 更新 CHANGELOG.md
# 记录本版本的所有更改

# 3. 运行完整测试
cargo test --release
cargo test --release -- --ignored

# 4. 构建发布版本
cargo build --release

# 5. 创建 Git 标签
git tag -a v0.2.0 -m "Release version 0.2.0"
git push origin v0.2.0

# 6. 发布到 crates.io (如果适用)
cargo publish
```

### 发布检查清单

- [ ] 所有测试通过
- [ ] 文档已更新
- [ ] CHANGELOG.md 已更新
- [ ] 版本号已更新
- [ ] 安全审计已完成
- [ ] 性能基准测试已运行

---

*本指南为 Ferox Encryptor 项目的开发提供了完整的技术规范和流程指导。*

*This guide provides comprehensive technical specifications and process guidance for Ferox Encryptor project development.*
