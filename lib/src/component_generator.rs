use crate::*;
use crate::license_handler::LicenseHandler;
use crate::model_analyzer::ModelAnalyzer;

pub struct ComponentGenerator {
    license_handler: LicenseHandler,
    model_analyzer: ModelAnalyzer,
}

impl ComponentGenerator {
    pub fn new() -> Self {
        Self {
            license_handler: LicenseHandler::new(),
            model_analyzer: ModelAnalyzer::new(),
        }
    }

    pub fn create_dataset_component(&self, dataset_id: &str) -> Component {
        let (org, dataset_name) = self.model_analyzer.extract_organization_from_dataset_id(dataset_id);
        let version = "1.0".to_string();
        let purl = format!("pkg:huggingface-dataset/{}@{}", dataset_id, version);
        let bom_ref = purl.clone();

        Component {
            component_type: "data".to_string(),
            bom_ref: bom_ref.clone(),
            name: dataset_name.clone(),
            version: Some(version.clone()),
            description: Some("Training dataset".to_string()),
            group: Some(org.clone()),
            publisher: Some(org.clone()),
            supplier: Some(Organization {
                name: org.clone(),
                url: Some(vec![format!("https://huggingface.co/datasets/{}", org)]),
            }),
            manufacturer: Some(Organization {
                name: org.clone(),
                url: Some(vec![format!("https://huggingface.co/datasets/{}", org)]),
            }),
            authors: Some(vec![Author { name: org.clone() }]),
            copyright: Some("NOASSERTION".to_string()),
            licenses: None, // Dataset license would need separate API call
            external_references: Some(vec![ExternalReference {
                ref_type: "website".to_string(),
                url: format!("https://huggingface.co/datasets/{}", dataset_id),
                comment: Some("Dataset repository".to_string()),
            }]),
            purl: Some(purl),
            model_card: None,
        }
    }

    pub fn model_info_to_component(&self, model_info: &ModelInfo, relation: Option<String>) -> Component {
        let (org, model_name) = self.model_analyzer.extract_organization_from_model_id(&model_info.model_id);
        let version = "1.0".to_string();
        let purl = format!("pkg:huggingface/{}@{}", model_info.model_id, version);
        let bom_ref = purl.clone();

        // Extract license from tags if not available in license field
        let license_str = model_info
            .license
            .clone()
            .or_else(|| self.model_analyzer.extract_license_from_tags(&model_info.tags));

        // Create ModelCard
        let model_card = if self.model_analyzer.is_machine_learning_model(&model_info.tags) {
            let task = self.model_analyzer.determine_task(&model_info.tags);
            let architecture = self.model_analyzer.get_model_architecture(model_info);

            // Create properties array
            let mut properties = vec![
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

            // Add relation information if available
            if let Some(rel) = &relation {
                properties.push(Property {
                    name: "ai.model.relation".to_string(),
                    value: rel.clone(),
                });
            }

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
                self.license_handler.normalize_license(license, model_info)
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

    pub fn create_main_application_component(&self, main_model_id: &str) -> Component {
        let (_main_org, main_name) = self.model_analyzer.extract_organization_from_model_id(main_model_id);
        let main_purl = format!("pkg:generic/{}@1.0", main_model_id.replace("/", "%2F"));

        Component {
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
        }
    }
}