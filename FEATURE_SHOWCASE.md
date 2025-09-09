# Feature Showcase

This document demonstrates the key features of the HuggingFace AIBOM Generator with real examples.

## 1. Custom License Detection

### Command:
```bash
cargo run -p cli -- tencent/HunyuanWorld-Voyager --verbose
```

### Key Output Highlights:

The tool automatically detects and handles custom licenses that are not in the SPDX registry:

```json
{
  "components": [
    {
      "type": "machine-learning-model",
      "bom-ref": "pkg:huggingface/tencent/HunyuanWorld-Voyager@1.0",
      "name": "HunyuanWorld-Voyager",
      "group": "tencent",
      "licenses": [
        {
          "license": {
            "name": "Custom Tencent License",
            "url": "https://huggingface.co/tencent/HunyuanWorld-Voyager/resolve/main/LICENSE"
          }
        }
      ]
    }
  ]
}
```

**Features Demonstrated:**
- ✅ **Custom License Detection**: Automatically finds non-SPDX licenses
- ✅ **License File URL**: Links to the actual license file on HuggingFace
- ✅ **SPDX Compliance**: Uses `null` for `id` when not an SPDX license

## 2. Model Dependency Relationships

### Command:
```bash
cargo run -p cli -- aeevnn/NaveenBhav --verbose
```

### Complete Output:

```json
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.6",
  "serialNumber": "urn:uuid:a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "version": 1,
  "metadata": {
    "timestamp": "2024-12-09T10:30:00Z",
    "tools": {
      "components": [
        {
          "bom-ref": "pkg:generic/rust-aibom-generator@1.0.0",
          "manufacturer": {
            "name": "Rust AIBOM Generator"
          },
          "name": "rust-aibom-generator",
          "type": "application",
          "version": "1.0"
        }
      ]
    },
    "component": {
      "type": "application",
      "bom-ref": "pkg:generic/aeevnn%2FNaveenBhav@1.0",
      "name": "NaveenBhav",
      "version": "1.0",
      "description": "No description available"
    },
    "properties": [
      {
        "name": "primaryPurpose",
        "value": "text-generation"
      },
      {
        "name": "suppliedBy", 
        "value": "aeevnn"
      }
    ]
  },
  "components": [
    {
      "type": "data",
      "bom-ref": "pkg:huggingface-dataset/nvidia/Llama-Nemotron-VLM-Dataset-v1@1.0",
      "name": "Llama-Nemotron-VLM-Dataset-v1",
      "version": "1.0",
      "description": "Training dataset",
      "group": "nvidia",
      "publisher": "nvidia",
      "supplier": {
        "name": "nvidia",
        "url": ["https://huggingface.co/datasets/nvidia"]
      },
      "manufacturer": {
        "name": "nvidia", 
        "url": ["https://huggingface.co/datasets/nvidia"]
      },
      "authors": [
        {
          "name": "nvidia"
        }
      ],
      "copyright": "NOASSERTION",
      "external-references": [
        {
          "type": "website",
          "url": "https://huggingface.co/datasets/nvidia/Llama-Nemotron-VLM-Dataset-v1",
          "comment": "Dataset repository"
        }
      ],
      "purl": "pkg:huggingface-dataset/nvidia/Llama-Nemotron-VLM-Dataset-v1@1.0"
    },
    {
      "type": "machine-learning-model",
      "bom-ref": "pkg:huggingface/deepseek-ai/DeepSeek-V3.1-Base@1.0", 
      "name": "DeepSeek-V3.1-Base",
      "version": "1.0",
      "description": "No description available",
      "group": "deepseek-ai",
      "publisher": "deepseek-ai",
      "modelCard": {
        "modelParameters": {
          "architecture_family": "transformer",
          "model_architecture": "TransformerModel",
          "task": "text-generation",
          "inputs": [
            {
              "format": "text"
            }
          ],
          "outputs": [
            {
              "format": "generated-text"
            }
          ]
        },
        "properties": [
          {
            "name": "bomFormat",
            "value": "CycloneDX"
          },
          {
            "name": "ai.model.relation",
            "value": "adapter"
          }
        ]
      },
      "purl": "pkg:huggingface/deepseek-ai/DeepSeek-V3.1-Base@1.0"
    },
    {
      "type": "machine-learning-model",
      "bom-ref": "pkg:huggingface/meta-llama/Llama-3.1-8B-Instruct@1.0",
      "name": "Llama-3.1-8B-Instruct", 
      "version": "1.0",
      "description": "No description available",
      "group": "meta-llama",
      "publisher": "meta-llama",
      "modelCard": {
        "modelParameters": {
          "architecture_family": "transformer",
          "model_architecture": "TransformerModel", 
          "task": "text-generation",
          "inputs": [
            {
              "format": "text"
            }
          ],
          "outputs": [
            {
              "format": "generated-text"
            }
          ]
        },
        "properties": [
          {
            "name": "bomFormat",
            "value": "CycloneDX"
          },
          {
            "name": "ai.model.relation",
            "value": "adapter"
          }
        ]
      },
      "purl": "pkg:huggingface/meta-llama/Llama-3.1-8B-Instruct@1.0"
    },
    {
      "type": "machine-learning-model",
      "bom-ref": "pkg:huggingface/meta-llama/Meta-Llama-3.1-8B@1.0",
      "name": "Meta-Llama-3.1-8B",
      "version": "1.0", 
      "description": "No description available",
      "group": "meta-llama",
      "publisher": "meta-llama",
      "modelCard": {
        "modelParameters": {
          "architecture_family": "transformer",
          "model_architecture": "TransformerModel",
          "task": "text-generation"
        }
      },
      "purl": "pkg:huggingface/meta-llama/Meta-Llama-3.1-8B@1.0"
    },
    {
      "type": "machine-learning-model",
      "bom-ref": "pkg:huggingface/aeevnn/NaveenBhav@1.0",
      "name": "NaveenBhav",
      "version": "1.0",
      "description": "No description available", 
      "group": "aeevnn",
      "publisher": "aeevnn",
      "modelCard": {
        "modelParameters": {
          "architecture_family": "transformer",
          "model_architecture": "TransformerModel",
          "task": "text-generation"
        }
      },
      "purl": "pkg:huggingface/aeevnn/NaveenBhav@1.0"
    }
  ],
  "dependencies": [
    {
      "ref": "pkg:huggingface/aeevnn/NaveenBhav@1.0",
      "dependsOn": [
        "pkg:huggingface-dataset/nvidia/Llama-Nemotron-VLM-Dataset-v1@1.0",
        "pkg:huggingface/deepseek-ai/DeepSeek-V3.1-Base@1.0", 
        "pkg:huggingface/meta-llama/Llama-3.1-8B-Instruct@1.0"
      ]
    },
    {
      "ref": "pkg:huggingface/meta-llama/Llama-3.1-8B-Instruct@1.0",
      "dependsOn": [
        "pkg:huggingface/meta-llama/Meta-Llama-3.1-8B@1.0"
      ]
    }
  ],
  "external-references": [
    {
      "type": "distribution",
      "url": "https://huggingface.co/aeevnn/NaveenBhav"
    }
  ]
}
```

### Key Features Demonstrated:

#### 🗂️ **Training Dataset Detection**
- **Dataset Component**: `nvidia/Llama-Nemotron-VLM-Dataset-v1` identified as `type: "data"`
- **Automatic Discovery**: Extracted from model card metadata
- **Proper PURL**: Uses `pkg:huggingface-dataset/` namespace

#### 🔗 **Model Relationship Mapping**
- **Adapter Relations**: `ai.model.relation: "adapter"` in modelCard properties
- **Base Model Chain**: NaveenBhav → Llama-3.1-8B-Instruct → Meta-Llama-3.1-8B
- **Relationship Storage**: Relations stored in modelCard, not in dependencies

#### 📋 **CycloneDX 1.6 Compliance**
- **Simple Dependencies**: `dependsOn` is a string array, not complex objects
- **No Relation/Scope**: Removed from dependency structure per spec
- **Proper Component Types**: `machine-learning-model` and `data`

#### 🔄 **Recursive Processing**
- **Full Dependency Chain**: Automatically follows all model relationships
- **Deduplication**: Prevents circular dependencies and duplicates
- **Complete Graph**: Maps the entire model ecosystem

#### 🏷️ **PURL Standards**
- **Models**: `pkg:huggingface/org/model@version`
- **Datasets**: `pkg:huggingface-dataset/org/dataset@version`
- **Consistent Versioning**: Uses semantic versioning

This comprehensive output demonstrates the tool's ability to create a complete AI Bill of Materials that provides full transparency into model dependencies, training data, and relationships while maintaining strict compliance with industry standards.

## 3. Model Relationship Representation in ModelCard

### Understanding `ai.model.relation` in ModelParameters

The tool stores model relationships in the `modelCard.modelParameters.properties` section using the standardized property name `ai.model.relation`. This approach follows CycloneDX 1.6 best practices by separating relationship metadata from dependency structure.

### Relationship Types Supported

| Relation Type | Description | Example Use Case |
|---------------|-------------|------------------|
| `adapter` | Model is an adapter of the base model | LoRA adapters, parameter-efficient fine-tuning |
| `finetuned` | Model is fine-tuned from the base model | Instruction-tuned models, domain-specific models |
| `quantized` | Model is a quantized version | GGUF, GPTQ, AWQ quantized models |
| `merged` | Model is merged from multiple models | Model merging, ensemble models |
| `distilled` | Model is distilled from a larger model | Knowledge distillation |
| `converted` | Model is converted to different format | ONNX, TensorRT conversions |
| `pruned` | Model has been pruned | Structured/unstructured pruning |
| `parent` | General parent-child relationship | Legacy or generic relationships |

### Example: Adapter Relationship

```json
{
  "type": "machine-learning-model",
  "bom-ref": "pkg:huggingface/deepseek-ai/DeepSeek-V3.1-Base@1.0",
  "name": "DeepSeek-V3.1-Base",
  "modelCard": {
    "modelParameters": {
      "architecture_family": "transformer",
      "model_architecture": "TransformerModel",
      "task": "text-generation",
      "properties": [
        {
          "name": "ai.model.relation",
          "value": "adapter"
        }
      ]
    }
  }
}
```

### Why Store Relations in ModelCard?

1. **CycloneDX 1.6 Compliance**: The specification requires simple string arrays in `dependsOn`, not complex objects with metadata
2. **Semantic Separation**: Dependencies represent "what depends on what", while relations represent "how they relate"
3. **Model-Specific Metadata**: Relations are intrinsic properties of the model, not the dependency relationship
4. **Extensibility**: Additional model metadata can be added to properties without affecting dependency structure

### Relationship Detection Logic

The tool automatically detects relationships from various sources:

1. **Model Card Data**: Explicit `base_model_relation` field
2. **Library Tags**: `peft`, `adapter-transformers`, etc.
3. **Model Tags**: `lora`, `qlora`, `adapter`, `instruction-tuning`
4. **Name Patterns**: Model names containing `gguf`, `gptq`, `awq`, `lora`, etc.
5. **Quantization Indicators**: `quantized_by` field, quantization tags

### Example: Complete Relationship Chain

In the `aeevnn/NaveenBhav` example:

```
NaveenBhav (main model)
├── nvidia/Llama-Nemotron-VLM-Dataset-v1 (training data)
├── deepseek-ai/DeepSeek-V3.1-Base (adapter relationship)
└── meta-llama/Llama-3.1-8B-Instruct (adapter relationship)
    └── meta-llama/Meta-Llama-3.1-8B (finetuned relationship)
```

Each model component contains its relationship information:
- **DeepSeek-V3.1-Base**: `"ai.model.relation": "adapter"`
- **Llama-3.1-8B-Instruct**: `"ai.model.relation": "adapter"`
- **Meta-Llama-3.1-8B**: No relation (base model)

### Benefits of This Approach

- **Standards Compliant**: Follows CycloneDX 1.6 specification exactly
- **Clear Semantics**: Separates "what depends on what" from "how they relate"
- **Rich Metadata**: Preserves important relationship information for AI governance
- **Tool Compatibility**: Works with standard CycloneDX tools and parsers
- **Future-Proof**: Extensible for additional AI-specific metadata