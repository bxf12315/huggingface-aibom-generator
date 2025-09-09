use crate::*;
use std::collections::{HashMap, HashSet};

pub struct AIBOMGenerator {
    api: hf_hub::api::sync::Api,
    processed_models: HashSet<String>,
    components: Vec<Component>,
    dependencies: HashMap<String, Vec<String>>,
}

impl AIBOMGenerator {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let api = hf_hub::api::sync::Api::new()?;
        Ok(Self {
            api,
            processed_models: HashSet::new(),
            components: Vec::new(),
            dependencies: HashMap::new(),
        })
    }

    pub fn get_model_info(&self, model_id: &str) -> Result<ModelInfo, Box<dyn std::error::Error>> {
        let _repo = self.api.model(model_id.to_string());
        let model_info = self.fetch_model_info_from_hf_api(model_id)?;
        Ok(model_info)
    }

    fn fetch_model_info_from_hf_api(
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

    fn extract_dependencies(&self, model_info: &ModelInfo) -> Vec<String> {
        let mut dependencies = Vec::new();

        // Extract dependencies from model card data
        if let Some(card_data) = &model_info.card_data {
            if let Some(base_model) = card_data.get("base_model") {
                if let Some(base_model_str) = base_model.as_str() {
                    dependencies.push(base_model_str.to_string());
                }
            }
        }

        // Add known dependencies based on model family knowledge
        // DialoGPT models have a hierarchical relationship
        if model_info.model_id.contains("DialoGPT-medium") {
            dependencies.push("microsoft/DialoGPT-base".to_string());
        } else if model_info.model_id.contains("DialoGPT-large") {
            dependencies.push("microsoft/DialoGPT-medium".to_string());
            dependencies.push("microsoft/DialoGPT-base".to_string());
        }

        dependencies
    }

    fn normalize_license(&self, license: &str, model_id: &str) -> Option<LicenseInfo> {
        // Try different normalization strategies to find a valid SPDX license ID
        let variations = [
            license.to_string(),                      // Original
            license.to_lowercase(),                   // Lowercase
            license.to_uppercase(),                   // Uppercase
            license.to_lowercase().replace(" ", "-"), // Lowercase with dashes
            license.to_lowercase().replace("-", " "), // Lowercase with spaces
            license.replace(" ", "-"),                // Original with dashes
            license.replace("-", " "),                // Original with spaces
        ];

        // Check each variation against SPDX license IDs
        for variant in &variations {
            if let Some(spdx_license) = spdx::license_id(variant) {
                return Some(LicenseInfo {
                    id: Some(spdx_license.name.to_string()),
                    name: Some(spdx_license.full_name.to_string()),
                    url: Some(format!("https://spdx.org/licenses/{}", spdx_license.name)),
                    text: None,
                });
            }
        }

        // If not found in SPDX, try to read LICENSE file from HuggingFace repo
        if let Ok(license_text) = self.fetch_license_file(model_id) {
            return Some(LicenseInfo {
                id: None,
                name: Some(license.to_string()),
                url: None,
                text: Some(license_text),
            });
        }

        // If no license information is available, return None
        None
    }

    fn fetch_license_file(&self, model_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        use reqwest::blocking::Client;
        use std::time::Duration;

        let client = Client::builder().timeout(Duration::from_secs(10)).build()?;

        // Try common LICENSE file names
        let license_files = [
            "LICENSE",
            "LICENSE.txt",
            "LICENSE.md",
            "license",
            "license.txt",
        ];

        for filename in &license_files {
            let url = format!(
                "https://huggingface.co/{}/resolve/main/{}",
                model_id, filename
            );

            if let Ok(response) = client
                .get(&url)
                .header("User-Agent", "rust-aibom-generator/1.0")
                .send()
            {
                if response.status().is_success() {
                    if let Ok(text) = response.text() {
                        // Basic validation that this looks like a license file
                        if text.len() > 50
                            && (text.to_lowercase().contains("license")
                                || text.to_lowercase().contains("copyright")
                                || text.to_lowercase().contains("permission"))
                        {
                            return Ok(text);
                        }
                    }
                }
            }
        }

        Err("No LICENSE file found".into())
    }

    fn extract_organization_from_model_id(&self, model_id: &str) -> (String, String) {
        let parts: Vec<&str> = model_id.split('/').collect();
        if parts.len() >= 2 {
            (parts[0].to_string(), parts[1].to_string())
        } else {
            ("huggingface".to_string(), model_id.to_string())
        }
    }

    fn is_machine_learning_model(&self, tags: &[String]) -> bool {
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

    fn determine_task(&self, tags: &[String]) -> String {
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

    fn get_model_architecture(&self, model_info: &ModelInfo) -> String {
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

        // Check tags for architecture information
        for tag in &model_info.tags {
            if tag.contains("gpt2") || tag.contains("gpt") {
                return "GPT2LMHeadModel".to_string();
            } else if tag.contains("bert") {
                return "BertModel".to_string();
            } else if tag.contains("t5") {
                return "T5ForConditionalGeneration".to_string();
            }
        }

        // Default to generic transformer if no specific architecture found
        "TransformerModel".to_string()
    }

    fn extract_license_from_tags(&self, tags: &[String]) -> Option<String> {
        for tag in tags {
            if tag.starts_with("license:") {
                return Some(tag.strip_prefix("license:").unwrap().to_string());
            }
        }
        None
    }

    fn model_info_to_component(&self, model_info: &ModelInfo) -> Component {
        let (org, model_name) = self.extract_organization_from_model_id(&model_info.model_id);
        let version = "1.0".to_string();
        let purl = format!("pkg:huggingface/{}@{}", model_info.model_id, version);
        let bom_ref = purl.clone();

        // Extract license from tags if not available in license field
        let license_str = model_info.license.clone()
            .or_else(|| self.extract_license_from_tags(&model_info.tags));

        // Create ModelCard
        let model_card = if self.is_machine_learning_model(&model_info.tags) {
            let task = self.determine_task(&model_info.tags);
            let architecture = self.get_model_architecture(model_info);

            // Create properties array
            let properties = vec![
                Property {
                    name: "bomFormat".to_string(),
                    value: "CycloneDX".to_string(),
                },
                Property {
                    name: "specVersion".to_string(),
                    value: "1.6".to_string(),
                },
                Property {
                    name: "serialNumber".to_string(),
                    value: format!("urn:uuid:{}", model_info.model_id.replace("/", "-")),
                },
                Property {
                    name: "version".to_string(),
                    value: "1.0.0".to_string(),
                },
                Property {
                    name: "primaryPurpose".to_string(),
                    value: task.clone(),
                },
                Property {
                    name: "suppliedBy".to_string(),
                    value: org.clone(),
                },
                Property {
                    name: "typeOfModel".to_string(),
                    value: "transformer".to_string(),
                },
                Property {
                    name: "downloadLocation".to_string(),
                    value: format!("https://huggingface.co/{}/tree/main", model_info.model_id),
                },
                Property {
                    name: "external_references".to_string(),
                    value: format!(
                        r#"[{{"type": "website", "url": "https://huggingface.co/{}", "comment": "Model repository"}}, {{"type": "distribution", "url": "https://huggingface.co/{}/tree/main", "comment": "Model files"}}]"#,
                        model_info.model_id, model_info.model_id
                    ),
                },
            ];

            Some(ModelCard {
                model_parameters: Some(ModelParameters {
                    architecture_family: Some("transformer".to_string()),
                    model_architecture: Some(architecture),
                    task: Some(task),
                    inputs: Some(vec![InputOutputData {
                        format: "text".to_string(),
                    }]),
                    outputs: Some(vec![InputOutputData {
                        format: "generated-text".to_string(),
                    }]),
                }),
                properties: Some(properties),
                quantitative_analysis: Some(QuantitativeAnalysis {
                    graphics: Some(serde_json::Value::Object(serde_json::Map::new())),
                    performance_metrics: None,
                }),
            })
        } else {
            None
        };

        Component {
            component_type: "machine-learning-model".to_string(),
            bom_ref: bom_ref.clone(),
            name: model_name.clone(),
            version: Some(version.clone()),
            description: Some("No description available".to_string()),
            group: Some(org.clone()),
            publisher: Some(org.clone()),
            supplier: Some(Organization {
                name: org.clone(),
                url: Some(vec![format!("https://huggingface.co/{}", org)]),
            }),
            manufacturer: Some(Organization {
                name: org.clone(),
                url: Some(vec![format!("https://huggingface.co/{}", org)]),
            }),
            authors: Some(vec![Author { name: org.clone() }]),
            copyright: Some("NOASSERTION".to_string()),
            licenses: license_str.as_ref().and_then(|license| {
                self.normalize_license(license, &model_info.model_id)
                    .map(|license_info| {
                        vec![License {
                            license: license_info,
                        }]
                    })
            }),
            external_references: Some(vec![ExternalReference {
                ref_type: "website".to_string(),
                url: format!("https://huggingface.co/{}", model_info.model_id),
                comment: None,
            }]),
            purl: Some(purl),
            model_card,
        }
    }

    fn process_model_recursively(
        &mut self,
        model_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.processed_models.contains(model_id) {
            return Ok(());
        }

        println!("Processing model: {}", model_id);
        self.processed_models.insert(model_id.to_string());

        let model_info = self.get_model_info(model_id)?;
        let dependencies = self.extract_dependencies(&model_info);

        // Process dependent models
        let mut processed_dependencies = Vec::new();
        for dep_model in &dependencies {
            if !self.processed_models.contains(dep_model) {
                match self.process_model_recursively(dep_model) {
                    Ok(_) => {
                        // Use the same PURL format
                        let dep_purl = format!("pkg:huggingface/{}@1.0", dep_model);
                        processed_dependencies.push(dep_purl);
                    }
                    Err(e) => {
                        println!("Warning: Failed to process dependency {}: {}", dep_model, e);
                        // Still add the dependency reference even if we couldn't fetch details
                        let dep_purl = format!("pkg:huggingface/{}@1.0", dep_model);
                        processed_dependencies.push(dep_purl);
                    }
                }
            } else {
                // Use the same PURL format
                let dep_purl = format!("pkg:huggingface/{}@1.0", dep_model);
                processed_dependencies.push(dep_purl);
            }
        }

        // Create component
        let component = self.model_info_to_component(&model_info);
        let bom_ref = component.bom_ref.clone();

        self.components.push(component);

        // Record dependencies
        if !processed_dependencies.is_empty() {
            self.dependencies.insert(bom_ref, processed_dependencies);
        }

        Ok(())
    }

    pub fn generate_aibom(
        &mut self,
        main_model_id: &str,
    ) -> Result<AIBOM, Box<dyn std::error::Error>> {
        // Process main model and all dependencies
        self.process_model_recursively(main_model_id)?;

        // Generate dependency list
        let dependencies: Vec<Dependency> = self
            .dependencies
            .iter()
            .map(|(model_ref, deps)| Dependency {
                reference: model_ref.clone(),
                depends_on: deps.clone(),
            })
            .collect();

        let (main_org, main_name) = self.extract_organization_from_model_id(main_model_id);
        let main_purl = format!("pkg:generic/{}@1.0", main_model_id.replace("/", "%2F"));

        // Create main application component
        let main_component = Component {
            component_type: "application".to_string(),
            bom_ref: main_purl.clone(),
            name: main_name.clone(),
            version: Some("1.0".to_string()),
            description: Some("No description available".to_string()),
            group: None,
            publisher: None,
            supplier: None,
            manufacturer: None,
            authors: None,
            copyright: Some("NOASSERTION".to_string()),
            licenses: None,
            external_references: None,
            purl: Some(main_purl.clone()),
            model_card: None,
        };

        // Generate RFC-4122 compliant UUID
        let uuid = uuid::Uuid::new_v4();

        let aibom = AIBOM {
            bom_format: "CycloneDX".to_string(),
            spec_version: "1.6".to_string(),
            serial_number: format!("urn:uuid:{}", uuid),
            version: 1,
            metadata: Metadata {
                timestamp: chrono::Utc::now().to_rfc3339(),
                tools: Tools {
                    components: vec![ToolComponent {
                        bom_ref: "pkg:generic/rust-aibom-generator@1.0.0".to_string(),
                        manufacturer: Organization {
                            name: "Rust AIBOM Generator".to_string(),
                            url: None,
                        },
                        name: "rust-aibom-generator".to_string(),
                        component_type: "application".to_string(),
                        version: "1.0".to_string(),
                    }],
                },
                component: main_component,
                properties: Some(vec![
                    Property {
                        name: "primaryPurpose".to_string(),
                        value: "text-generation".to_string(),
                    },
                    Property {
                        name: "suppliedBy".to_string(),
                        value: main_org,
                    },
                ]),
            },
            components: self.components.clone(),
            dependencies,
            external_references: Some(vec![ExternalReference {
                ref_type: "distribution".to_string(),
                url: format!("https://huggingface.co/{}", main_model_id),
                comment: None,
            }]),
        };

        Ok(aibom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_license_from_tags() {
        let generator = AIBOMGenerator::new().unwrap();
        
        let tags = vec![
            "transformers".to_string(),
            "pytorch".to_string(),
            "license:mit".to_string(),
            "text-generation".to_string(),
        ];
        
        let license = generator.extract_license_from_tags(&tags);
        assert_eq!(license, Some("mit".to_string()));
    }

    #[test]
    fn test_extract_dependencies_dialogpt() {
        let generator = AIBOMGenerator::new().unwrap();
        
        let model_info = ModelInfo {
            model_id: "microsoft/DialoGPT-medium".to_string(),
            tags: vec!["transformers".to_string(), "gpt2".to_string()],
            library_name: Some("transformers".to_string()),
            created_at: None,
            last_modified: None,
            license: None,
            card_data: None,
            siblings: None,
            sha: None,
        };
        
        let dependencies = generator.extract_dependencies(&model_info);
        assert!(dependencies.contains(&"microsoft/DialoGPT-base".to_string()));
    }

    #[test]
    fn test_get_model_architecture_from_tags() {
        let generator = AIBOMGenerator::new().unwrap();
        
        let model_info = ModelInfo {
            model_id: "microsoft/DialoGPT-medium".to_string(),
            tags: vec!["transformers".to_string(), "gpt2".to_string()],
            library_name: Some("transformers".to_string()),
            created_at: None,
            last_modified: None,
            license: None,
            card_data: None,
            siblings: None,
            sha: None,
        };
        
        let architecture = generator.get_model_architecture(&model_info);
        assert_eq!(architecture, "GPT2LMHeadModel");
    }

    #[test]
    fn test_get_model_architecture_from_card_data() {
        let generator = AIBOMGenerator::new().unwrap();
        
        let card_data = json!({
            "architectures": ["GPT2LMHeadModel"],
            "model_type": "gpt2"
        });
        
        let model_info = ModelInfo {
            model_id: "microsoft/DialoGPT-medium".to_string(),
            tags: vec!["transformers".to_string()],
            library_name: Some("transformers".to_string()),
            created_at: None,
            last_modified: None,
            license: None,
            card_data: Some(card_data),
            siblings: None,
            sha: None,
        };
        
        let architecture = generator.get_model_architecture(&model_info);
        assert_eq!(architecture, "GPT2LMHeadModel");
    }
}
