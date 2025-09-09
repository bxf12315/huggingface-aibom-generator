# HuggingFace AIBOM Generator

A Rust tool for generating AI Bill of Materials (AIBOM) for machine learning models, supporting both CLI and HTTP server usage modes.

## Overview

This project generates comprehensive AI Bill of Materials (AIBOM) documents for HuggingFace models, following the CycloneDX 1.6 specification. It extracts model metadata, dependencies, and creates structured documentation for AI model supply chain transparency.

## Project Structure

- `cli/` - Command-line interface tool
- `server/` - HTTP server (based on actix_web)
- `lib/` - Core library containing AIBOM generation logic

## Prerequisites

- Rust 1.70 or later
- Cargo package manager

## Installation

Clone the repository and build the project:

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

## Features

- 🤖 HuggingFace model AIBOM generation
- 📋 CycloneDX 1.6 specification compliance
- 🔗 **Intelligent Dependency Extraction** - Dynamically extract model dependencies from HuggingFace API
- 🌐 HTTP API interface
- 📝 Detailed model metadata extraction
- 🔄 Recursive dependency processing
- ⚡ Fast and efficient Rust implementation
- 🛡️ Type-safe model handling
- ⚙️ Configurable model hierarchies via JSON config
- 🔧 Dynamic dependency relationship management
- 🧠 **Multi-layer Dependency Inference** - Infer dependencies from model cards, tags, configurations, and other sources

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

### Generate AIBOM for Popular Models

```bash
# GPT-2 model
cargo run -p cli -- gpt2 --output gpt2-aibom.json

# BERT model
cargo run -p cli -- bert-base-uncased --output bert-aibom.json

# Custom model with verbose output
cargo run -p cli -- microsoft/DialoGPT-medium --output dialog-aibom.json --verbose
```

### Using the HTTP API

```bash
# Generate AIBOM via API
curl -X POST http://localhost:8080/generate \
  -H "Content-Type: application/json" \
  -d '{
    "model_id": "gpt2",
    "verbose": false
  }' | jq '.'
```

## Intelligent Dependency Extraction

### Dynamic Dependency Discovery

This tool now supports **automatic dependency extraction** from HuggingFace model information, eliminating the need to manually maintain all model dependencies!

#### Extraction Strategies (by priority)

1. **Model Card Data** - Extract `base_model`, `parent_model`, `dependencies` fields from `card_data`
2. **Configuration Files** - Infer from `_name_or_path` and other fields in `config.json`
3. **Tag Inference** - Infer dependencies based on model tags (`conversational`, `gpt2`, `bert`, etc.)
4. **Pipeline Tags** - Infer common dependency patterns from `pipeline_tag`
5. **Pattern Matching** - Based on model name patterns (`large`→`medium`→`base`, `-instruct`→base model)
6. **Configuration Files** - Compatible with existing `model_hierarchies.json` configuration

#### Example

```bash
# Automatically discover that DialoGPT-medium depends on DialoGPT-base
cargo run -p cli -- microsoft/DialoGPT-medium

# Output will include correct dependencies:
# "dependencies": [
#   {
#     "ref": "pkg:huggingface/microsoft/DialoGPT-medium@1.0",
#     "dependsOn": ["pkg:huggingface/microsoft/DialoGPT-base@1.0"]
#   }
# ]
```

### Legacy Configuration Support

Still supports configuring dependencies through `model_hierarchies.json` file:

```json
{
  "microsoft/DialoGPT-large": [
    "microsoft/DialoGPT-medium",
    "microsoft/DialoGPT-base"
  ],
  "custom/my-model-large": [
    "custom/my-model-base"
  ]
}
```

### Programming Interface

```rust
use lib::AIBOMGenerator;

let mut generator = AIBOMGenerator::new()?;

// Automatically extract dependencies from HuggingFace API
let aibom = generator.generate_aibom("microsoft/DialoGPT-medium")?;

// Manually add custom dependency relationships
generator.add_model_hierarchy(
    "custom/my-model-large".to_string(),
    vec!["custom/my-model-base".to_string()]
);
```

For detailed information, please refer to [DEPENDENCY_EXTRACTION.md](DEPENDENCY_EXTRACTION.md).

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for specific package
cargo test -p lib
cargo test -p cli
cargo test -p server
```

### Code Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check
```

### Linting

```bash
# Run clippy
cargo clippy

# Run clippy with all features
cargo clippy --all-features
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- HuggingFace for providing the model hub and APIs
- CycloneDX community for the SBOM specification
- Rust community for excellent tooling and libraries