# HuggingFace AIBOM Generator

A Rust tool for generating AI Bill of Materials (AIBOM) for machine learning models, supporting both CLI and HTTP server usage modes.

## Table of Contents

- [Overview](#overview)
- [Key Features](#key-features)
- [Project Structure](#project-structure)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Feature Highlights](#feature-highlights)
- [CLI Options](#cli-options)
- [API Documentation](#api-documentation)
- [Examples](#examples)
- [Dependency Extraction](#dependency-extraction)
- [Model Relationships](#model-relationships)
- [Development](#development)
- [License](#license)

## Overview

This project generates comprehensive AI Bill of Materials (AIBOM) documents for HuggingFace models, following the CycloneDX 1.6 specification. It extracts model metadata, dependencies, and creates structured documentation for AI model supply chain transparency.

## Project Structure

- `cli/` - Command-line interface tool
- `server/` - HTTP server (based on actix_web)
- `lib/` - Core library containing AIBOM generation logic

## Installation

```bash
git clone <repository-url>
cd aibom-generator
cargo build --release
```

## Quick Start

### CLI Usage

```bash
# Build the CLI tool
cargo build -p cli

# Generate AIBOM for a model
cargo run -p cli -- microsoft/DialoGPT-medium --output aibom.json --verbose

# Show help
cargo run -p cli -- --help
```

### Server Usage

```bash
# Start the server
cargo run -p server

# Test the API in another terminal
curl -X POST http://localhost:8080/generate \
  -H "Content-Type: application/json" \
  -d '{"model_id": "microsoft/DialoGPT-medium", "verbose": true}'

# Health check
curl http://localhost:8080/health
```

## Key Features

- 🤖 HuggingFace model AIBOM generation
- 📋 CycloneDX 1.6 specification compliance
- 🔗 Intelligent dependency extraction from model metadata
- 🌐 HTTP API interface
- 📝 Detailed model metadata extraction
- 🔄 Recursive dependency processing
- ⚡ Fast and efficient Rust implementation
- 📄 Custom license detection and normalization
- 🗂️ Training dataset dependency tracking
- 🔗 Model relationship mapping (adapter, fine-tuned, etc.)

## Feature Highlights

### 1. Custom License Detection

The tool can detect and normalize both SPDX and custom licenses from model metadata:

```bash
cargo run -p cli -- tencent/HunyuanWorld-Voyager --verbose
```

**Output highlights:**
```json
{
  "licenses": [
    {
      "license": {
        "name": "**Custom Tencent License**",
        "url": "https://huggingface.co/tencent/HunyuanWorld-Voyager/resolve/main/LICENSE"
      }
    }
  ]
}
```

### 2. Model Dependency Relationships

Automatically discovers and maps complex model relationships including base models, adapters, and training datasets:

```bash
cargo run -p cli -- aeevnn/NaveenBhav --verbose
```

**Complete Output:**
```json
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.6",
  "serialNumber": "urn:uuid:12345678-1234-5678-9abc-123456789abc",
  "version": 1,
  "metadata": {
    "timestamp": "2024-01-01T00:00:00Z",
    "tools": {
      "components": [
        {
          "bom-ref": "pkg:generic/rust-aibom-generator@1.0.0",
          "type": "application",
          "name": "rust-aibom-generator",
          "version": "1.0"
        }
      ]
    },
    "component": {
      "type": "application",
      "bom-ref": "pkg:generic/aeevnn%2FNaveenBhav@1.0",
      "name": "NaveenBhav"
    }
  },
  "components": [
    {
      "type": "data",
      "bom-ref": "**pkg:huggingface-dataset/nvidia/Llama-Nemotron-VLM-Dataset-v1@1.0**",
      "name": "Llama-Nemotron-VLM-Dataset-v1",
      "group": "nvidia",
      "description": "Training dataset"
    },
    {
      "type": "machine-learning-model",
      "bom-ref": "pkg:huggingface/deepseek-ai/DeepSeek-V3.1-Base@1.0",
      "name": "DeepSeek-V3.1-Base",
      "modelCard": {
        "modelParameters": {
          "properties": [
            {
              "name": "**ai.model.relation**",
              "value": "**adapter**"
            }
          ]
        }
      }
    },
    {
      "type": "machine-learning-model", 
      "bom-ref": "pkg:huggingface/meta-llama/Llama-3.1-8B-Instruct@1.0",
      "name": "Llama-3.1-8B-Instruct",
      "modelCard": {
        "modelParameters": {
          "properties": [
            {
              "name": "**ai.model.relation**",
              "value": "**adapter**"
            }
          ]
        }
      }
    }
  ],
  "**dependencies**": [
    {
      "ref": "pkg:huggingface/aeevnn/NaveenBhav@1.0",
      "**dependsOn**": [
        "**pkg:huggingface-dataset/nvidia/Llama-Nemotron-VLM-Dataset-v1@1.0**",
        "**pkg:huggingface/deepseek-ai/DeepSeek-V3.1-Base@1.0**",
        "**pkg:huggingface/meta-llama/Llama-3.1-8B-Instruct@1.0**"
      ]
    },
    {
      "ref": "pkg:huggingface/meta-llama/Llama-3.1-8B-Instruct@1.0", 
      "dependsOn": [
        "pkg:huggingface/meta-llama/Meta-Llama-3.1-8B@1.0"
      ]
    }
  ]
}
```

**Key Features Demonstrated:**
- **Training Dataset Detection**: Automatically identifies `nvidia/Llama-Nemotron-VLM-Dataset-v1` as training data
- **Model Relationships**: Maps adapter relationships in `modelCard.modelParameters.properties`
- **CycloneDX 1.6 Compliance**: Simple string arrays in `dependsOn` instead of complex objects
- **Recursive Dependencies**: Follows the full dependency chain (NaveenBhav → Llama-3.1-8B-Instruct → Meta-Llama-3.1-8B)

### Model Relationship Representation

Model relationships are stored in the `modelCard.modelParameters.properties` section using the standardized property:

```json
{
  "name": "ai.model.relation",
  "value": "adapter"
}
```

**Supported Relationship Types:**
- `adapter` - LoRA adapters, parameter-efficient fine-tuning
- `finetuned` - Instruction-tuned, domain-specific models  
- `quantized` - GGUF, GPTQ, AWQ quantized models
- `merged` - Model merging, ensemble models
- `distilled` - Knowledge distillation
- `converted` - ONNX, TensorRT conversions
- `pruned` - Structured/unstructured pruning

This approach follows CycloneDX 1.6 best practices by separating relationship metadata from dependency structure, ensuring both standards compliance and rich AI governance metadata.

## CLI Options

```bash
USAGE:
    aibom-generator [OPTIONS] <MODEL_ID>

ARGS:
    <MODEL_ID>    HuggingFace model identifier (e.g., microsoft/DialoGPT-medium)

OPTIONS:
    -o, --output <FILE>    Output file path (default: stdout)
    -v, --verbose          Enable verbose output
    -h, --help             Print help information
```

## API Documentation

For detailed API usage instructions, see [server/README.md](server/README.md).

## Examples

### Generate AIBOM for Models

```bash
# Generate AIBOM for a model
cargo run -p cli -- microsoft/DialoGPT-medium --output dialog-aibom.json --verbose

# Using the HTTP API
curl -X POST http://localhost:8080/generate \
  -H "Content-Type: application/json" \
  -d '{"model_id": "microsoft/DialoGPT-medium", "verbose": true}' | jq '.'
```

## Dependency Extraction

The tool automatically extracts model dependencies from HuggingFace model metadata:

1. **Model Card Data** - Extracts `base_model`, `parent_model` fields from model card
2. **Tag Analysis** - Infers relationships from model tags and metadata
3. **Name Pattern Matching** - Detects common patterns like quantization, fine-tuning, etc.

### Example

```bash
# Automatically discovers dependencies
cargo run -p cli -- microsoft/DialoGPT-medium
```

For detailed information, see [DEPENDENCY_EXTRACTION.md](DEPENDENCY_EXTRACTION.md).

## Model Relationships

Model relationships (adapter, fine-tuned, quantized, etc.) are stored in the `modelCard.modelParameters.properties` section using the standardized property `ai.model.relation`. This approach follows CycloneDX 1.6 best practices by separating relationship metadata from dependency structure.

For comprehensive documentation on model relationships, see:
- [MODEL_RELATIONSHIPS.md](MODEL_RELATIONSHIPS.md) - Complete guide to model relationship representation
- [FEATURE_SHOWCASE.md](FEATURE_SHOWCASE.md) - Real examples with detailed explanations

## Development

```bash
# Run tests
cargo test

# Format code
cargo fmt

# Run linting
cargo clippy
```

## License

This project is licensed under the Apache License 2.0 - see the LICENSE file for details.