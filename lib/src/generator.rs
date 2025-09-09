use crate::component_generator::ComponentGenerator;
use crate::model_analyzer::ModelAnalyzer;
use crate::*;
use std::collections::{HashMap, HashSet};

/// Main AIBOM Generator that orchestrates the generation process
pub struct AIBOMGenerator {
    api: hf_hub::api::sync::Api,
    component_generator: ComponentGenerator,
    model_analyzer: ModelAnalyzer,
    processed_models: HashSet<String>,
    components: Vec<Component>,
    dependencies: HashMap<String, Vec<String>>,
}

impl AIBOMGenerator {
    /// Create a new AIBOM Generator instance
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let api = hf_hub::api::sync::Api::new()?;
        let component_generator = ComponentGenerator::new();
        let model_analyzer = ModelAnalyzer::new();

        Ok(Self {
            api,
            component_generator,
            model_analyzer,
            processed_models: HashSet::new(),
            components: Vec::new(),
            dependencies: HashMap::new(),
        })
    }

    /// Get model information from HuggingFace API
    pub fn get_model_info(&self, model_id: &str) -> Result<ModelInfo, Box<dyn std::error::Error>> {
        let _repo = self.api.model(model_id.to_string());
        self.model_analyzer.fetch_model_info_from_hf_api(model_id)
    }

    /// Process a model and its dependencies recursively
    pub fn process_model_recursively(
        &mut self,
        model_id: &str,
        relation: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.processed_models.contains(model_id) {
            return Ok(());
        }

        println!("Processing model: {}", model_id);
        self.processed_models.insert(model_id.to_string());

        let model_info = self.get_model_info(model_id)?;
        let dependencies = self.model_analyzer.extract_dependencies(&model_info);

        // Separate datasets from model dependencies
        let mut dataset_dependencies = Vec::new();
        let mut model_dependencies = Vec::new();

        for (dep_id, relation) in &dependencies {
            if relation.as_ref().map(|r| r.as_str()) == Some("train") {
                dataset_dependencies.push((dep_id.clone(), relation.clone()));
            } else {
                model_dependencies.push((dep_id.clone(), relation.clone()));
            }
        }

        // Process dataset dependencies - only add to dependency list, no relation stored
        let mut processed_dependencies = Vec::new();
        for (dataset_id, _) in dataset_dependencies {
            let dataset_component = self
                .component_generator
                .create_dataset_component(&dataset_id);
            let dep_purl = dataset_component.purl.clone().unwrap();

            // Add dataset component to components list
            self.components.push(dataset_component);
            processed_dependencies.push(dep_purl);
            println!("Added dataset component: {}", dataset_id);
        }

        // Process model dependencies and store their relations
        for (dep_model, dep_relation) in model_dependencies {
            if !self.processed_models.contains(&dep_model) {
                match self.process_model_recursively(&dep_model, dep_relation.clone()) {
                    Ok(_) => {
                        let dep_purl = format!("pkg:huggingface/{}@1.0", dep_model);
                        processed_dependencies.push(dep_purl);
                    }
                    Err(e) => {
                        println!("Warning: Failed to process dependency {}: {}", dep_model, e);
                        let dep_purl = format!("pkg:huggingface/{}@1.0", dep_model);
                        processed_dependencies.push(dep_purl);
                    }
                }
            } else {
                let dep_purl = format!("pkg:huggingface/{}@1.0", dep_model);
                processed_dependencies.push(dep_purl);
            }
        }

        // Create component with relation information
        let component = self
            .component_generator
            .model_info_to_component(&model_info, relation);
        let bom_ref = component.bom_ref.clone();

        self.components.push(component);

        // Record dependencies (simplified structure)
        if !processed_dependencies.is_empty() {
            self.dependencies.insert(bom_ref, processed_dependencies);
        }

        Ok(())
    }

    /// Generate complete AIBOM for a given model
    pub fn generate_aibom(
        &mut self,
        main_model_id: &str,
    ) -> Result<AIBOM, Box<dyn std::error::Error>> {
        // Process main model and all dependencies (no relation for main model)
        self.process_model_recursively(main_model_id, None)?;

        // Generate dependency list with simplified structure
        let dependencies: Vec<Dependency> = self
            .dependencies
            .iter()
            .map(|(model_ref, deps)| Dependency {
                reference: model_ref.clone(),
                depends_on: deps.clone(), // Now just Vec<String>
            })
            .collect();

        let (main_org, _main_name) = self
            .model_analyzer
            .extract_organization_from_model_id(main_model_id);

        // Create main application component
        let main_component = self
            .component_generator
            .create_main_application_component(main_model_id);

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
