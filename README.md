# HuggingFace AIBOM Generator

A Rust tool for generating AI Bill of Materials (AIBOM) for machine learning models, supporting both CLI and HTTP server usage modes.

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

## Features

- 🤖 HuggingFace model AIBOM generation
- 📋 CycloneDX 1.6 specification compliance
- 🔗 Intelligent dependency extraction from model metadata
- 🌐 HTTP API interface
- 📝 Detailed model metadata extraction
- 🔄 Recursive dependency processing
- ⚡ Fast and efficient Rust implementation

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

This project is licensed under the MIT License - see the LICENSE file for details.