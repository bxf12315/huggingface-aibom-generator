use crate::*;

pub struct ModelAnalyzer;

impl ModelAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn fetch_model_info_from_hf_api(
        &self,
        model_id: &str,
    ) -> Result<ModelInfo, Box<dyn std::error::Error>> {
        use reqwest::blocking::Client;
        use std::time::Duration;

        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .danger_accept_invalid_certs(false)
            .build()?;

        let url = format!("https://huggingface.co/api/models/{}", model_id);

        println!("Fetching model info from: {}", url);

        let response = client
            .get(&url)
            .header("User-Agent", "rust-aibom-generator/1.0")
            .send()?;

        if !response.status().is_success() {
            return Err(format!("API request failed with status: {}", response.status()).into());
        }

        let model_info = response.json::<ModelInfo>()?;
        println!("Successfully fetched model info for: {}", model_id);
        Ok(model_info)
    }

    pub fn extract_dependencies(&self, model_info: &ModelInfo) -> Vec<(String, Option<String>)> {
        let mut dependencies = Vec::new();

        // Only extract dependencies from explicit model card data
        if let Some(card_data) = &model_info.card_data {
            // Check for explicit base_model field (can be string or array)
            if let Some(base_model) = card_data.get("base_model") {
                let mut relation = card_data
                    .get("base_model_relation")
                    .and_then(|r| r.as_str())
                    .map(|s| s.to_string());

                // If no explicit relation, try to infer from other fields
                if relation.is_none() {
                    relation = self.infer_relation_from_metadata(model_info, card_data);
                }

                if let Some(base_model_str) = base_model.as_str() {
                    dependencies.push((base_model_str.to_string(), relation.clone()));
                    println!(
                        "Found base_model dependency: {} (relation: {:?})",
                        base_model_str, relation
                    );
                } else if let Some(base_model_array) = base_model.as_array() {
                    for base_model_item in base_model_array {
                        if let Some(base_model_str) = base_model_item.as_str() {
                            dependencies.push((base_model_str.to_string(), relation.clone()));
                            println!(
                                "Found base_model dependency: {} (relation: {:?})",
                                base_model_str, relation
                            );
                        }
                    }
                }
            }

            // Check for parent_model field (some models use this)
            if let Some(parent_model) = card_data.get("parent_model") {
                if let Some(parent_model_str) = parent_model.as_str() {
                    dependencies.push((parent_model_str.to_string(), Some("parent".to_string())));
                    println!("Found parent_model dependency: {}", parent_model_str);
                }
            }

            // Check for datasets used to train the model
            if let Some(datasets) = card_data.get("datasets") {
                if let Some(datasets_array) = datasets.as_array() {
                    for dataset in datasets_array {
                        if let Some(dataset_str) = dataset.as_str() {
                            dependencies.push((dataset_str.to_string(), Some("train".to_string())));
                            println!("Found training dataset dependency: {}", dataset_str);
                        }
                    }
                } else if let Some(dataset_str) = datasets.as_str() {
                    dependencies.push((dataset_str.to_string(), Some("train".to_string())));
                    println!("Found training dataset dependency: {}", dataset_str);
                }
            }

            // Also check for train_dataset field (alternative naming)
            if let Some(train_dataset) = card_data.get("train_dataset") {
                if let Some(dataset_str) = train_dataset.as_str() {
                    dependencies.push((dataset_str.to_string(), Some("train".to_string())));
                    println!("Found training dataset dependency: {}", dataset_str);
                }
            }
        }

        // Remove duplicates and self-references
        dependencies.sort_by(|a, b| a.0.cmp(&b.0));
        dependencies.dedup();
        dependencies.retain(|(dep, _)| dep != &model_info.model_id);

        // Log warning if no dependencies found
        if dependencies.is_empty() {
            println!(
                "Warning: No explicit dependencies found for model: {}. Consider adding base_model, parent_model, or dependencies fields to the model card.",
                model_info.model_id
            );
        }

        dependencies
    }

    fn infer_relation_from_metadata(
        &self,
        model_info: &ModelInfo,
        card_data: &serde_json::Value,
    ) -> Option<String> {
        // Check library_name first (highest priority for specific model types)
        if let Some(library_name) = card_data.get("library_name") {
            if let Some(lib_str) = library_name.as_str() {
                match lib_str {
                    "adapter-transformers" | "adapters" => {
                        return Some("adapter".to_string());
                    }
                    "peft" => {
                        return Some("lora".to_string());
                    }
                    _ => {}
                }
            }
        }

        // Check for quantized_by field
        if let Some(quantized_by) = card_data.get("quantized_by") {
            if quantized_by.is_string() {
                return Some("quantized".to_string());
            }
        }

        // Check tags for relation indicators
        for tag in &model_info.tags {
            let tag_lower = tag.to_lowercase();

            // Check for base_model:relation:model format
            if tag_lower.starts_with("base_model:") {
                let parts: Vec<&str> = tag_lower.split(':').collect();
                if parts.len() >= 3 && parts[0] == "base_model" {
                    match parts[1] {
                        "finetune" | "finetuned" => return Some("finetuned".to_string()),
                        "adapter" => return Some("adapter".to_string()),
                        "lora" | "qlora" => return Some("lora".to_string()),
                        "quantized" | "quantization" => return Some("quantized".to_string()),
                        "merged" | "merge" => return Some("merged".to_string()),
                        "distilled" | "distillation" => return Some("distilled".to_string()),
                        _ => {}
                    }
                }
            } else {
                // Check for simple tag matches
                match tag_lower.as_str() {
                    "lora" | "qlora" => return Some("lora".to_string()),
                    "adapter" => return Some("adapter".to_string()),
                    "instruction-tuning" | "chat" => return Some("finetuned".to_string()),
                    "distillation" => return Some("distilled".to_string()),
                    "onnx" | "tensorrt" => return Some("converted".to_string()),
                    "pruning" => return Some("pruned".to_string()),
                    _ => {}
                }
            }
        }

        // Check for merge indicators (multiple base models or merge tags)
        if let Some(base_model) = card_data.get("base_model") {
            let is_merge = if let Some(base_model_array) = base_model.as_array() {
                base_model_array.len() > 1
            } else {
                false
            } || model_info
                .tags
                .iter()
                .any(|tag| tag.to_lowercase().contains("merge"));

            if is_merge {
                return Some("merged".to_string());
            }
        }

        // Check model name patterns for common relations
        let model_name = model_info.model_id.to_lowercase();
        if model_name.contains("gguf")
            || model_name.contains("gptq")
            || model_name.contains("awq")
            || model_name.contains("int4")
            || model_name.contains("int8")
        {
            Some("quantized".to_string())
        } else if model_name.contains("lora") || model_name.contains("qlora") {
            Some("lora".to_string())
        } else if model_name.contains("adapter") {
            Some("adapter".to_string())
        } else if model_name.contains("merge") {
            Some("merged".to_string())
        } else if model_name.contains("finetune") || model_name.contains("ft") {
            Some("finetuned".to_string())
        } else if model_name.contains("instruct") || model_name.contains("chat") {
            Some("finetuned".to_string())
        } else if model_name.contains("distil") {
            Some("distilled".to_string())
        } else if model_name.contains("onnx") {
            Some("converted".to_string())
        } else if model_name.contains("prune") {
            Some("pruned".to_string())
        } else {
            None
        }
    }

    pub fn extract_organization_from_model_id(&self, model_id: &str) -> (String, String) {
        let parts: Vec<&str> = model_id.split('/').collect();
        if parts.len() >= 2 {
            (parts[0].to_string(), parts[1].to_string())
        } else {
            ("huggingface".to_string(), model_id.to_string())
        }
    }

    pub fn extract_organization_from_dataset_id(&self, dataset_id: &str) -> (String, String) {
        let parts: Vec<&str> = dataset_id.split('/').collect();
        if parts.len() >= 2 {
            (parts[0].to_string(), parts[1].to_string())
        } else {
            ("huggingface".to_string(), dataset_id.to_string())
        }
    }

    pub fn is_machine_learning_model(&self, tags: &[String]) -> bool {
        // Check if it contains machine learning related tags
        let ml_tags = [
            "text-generation",
            "conversational",
            "text-classification",
            "feature-extraction",
            "translation",
            "summarization",
            "question-answering",
            "fill-mask",
            "token-classification",
            "image-classification",
            "object-detection",
            "image-segmentation",
            "audio-classification",
            "automatic-speech-recognition",
            "text-to-speech",
            "reinforcement-learning",
        ];

        tags.iter().any(|tag| ml_tags.contains(&tag.as_str()))
    }

    pub fn determine_task(&self, tags: &[String]) -> String {
        for tag in tags {
            match tag.as_str() {
                "text-generation" => return "text-generation".to_string(),
                "conversational" => return "conversational".to_string(),
                "text-classification" => return "text-classification".to_string(),
                "feature-extraction" => return "feature-extraction".to_string(),
                "translation" => return "translation".to_string(),
                _ => continue,
            }
        }
        "text-generation".to_string()
    }

    pub fn get_model_architecture(&self, model_info: &ModelInfo) -> String {
        // Try to extract architecture from model card data
        if let Some(card_data) = &model_info.card_data {
            if let Some(architecture) = card_data.get("architecture") {
                if let Some(arch_str) = architecture.as_str() {
                    return arch_str.to_string();
                }
            }
            // Also check for architectures array in config
            if let Some(architectures) = card_data.get("architectures") {
                if let Some(arch_array) = architectures.as_array() {
                    if let Some(first_arch) = arch_array.first() {
                        if let Some(arch_str) = first_arch.as_str() {
                            return arch_str.to_string();
                        }
                    }
                }
            }
        }

        // Default to generic transformer if no specific architecture found
        "TransformerModel".to_string()
    }

    pub fn extract_license_from_tags(&self, tags: &[String]) -> Option<String> {
        for tag in tags {
            if tag.starts_with("license:") {
                return Some(tag.strip_prefix("license:").unwrap().to_string());
            }
        }
        None
    }
}
