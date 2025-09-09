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
- 🔗 Automatic model dependency resolution
- 🌐 HTTP API interface
- 📝 Detailed model metadata extraction
- 🔄 Recursive dependency processing
- ⚡ Fast and efficient Rust implementation
- 🛡️ Type-safe model handling

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