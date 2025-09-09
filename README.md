# HuggingFace AIBOM Generator

一个用于生成机器学习模型AI物料清单(AIBOM)的Rust工具，支持CLI和HTTP服务器两种使用方式。

## 项目结构

- `cli/` - 命令行工具
- `server/` - HTTP服务器 (基于actix_web)
- `lib/` - 核心库，包含AIBOM生成逻辑

## 快速开始

### CLI使用

```bash
# 构建CLI工具
cargo build -p cli

# 生成AIBOM
cargo run -p cli -- microsoft/DialoGPT-medium --output aibom.json --verbose
```

### Server使用

```bash
# 启动服务器
cargo run -p server

# 在另一个终端测试API
curl -X POST http://localhost:8080/generate \
  -H "Content-Type: application/json" \
  -d '{"model_id": "microsoft/DialoGPT-medium", "verbose": true}'
```



## 功能特性

- 🤖 支持HuggingFace模型的AIBOM生成
- 📋 符合CycloneDX 1.6规范
- 🔗 自动解析模型依赖关系
- 🌐 提供HTTP API接口
- 📝 详细的模型元数据提取
- 🔄 递归处理依赖模型

## API文档

详细的API使用说明请参考 [server/README.md](server/README.md)。