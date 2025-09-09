use serde::{Deserialize, Serialize};
use super::{Metadata, Component, Dependency, ExternalReference};

#[derive(Serialize, Deserialize, Debug)]
pub struct AIBOM {
    #[serde(rename = "bomFormat")]
    pub bom_format: String,
    #[serde(rename = "specVersion")]
    pub spec_version: String,
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    pub version: u32,
    pub metadata: Metadata,
    pub components: Vec<Component>,
    pub dependencies: Vec<Dependency>,
    #[serde(rename = "externalReferences", skip_serializing_if = "Option::is_none")]
    pub external_references: Option<Vec<ExternalReference>>,
}