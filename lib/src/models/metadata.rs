use serde::{Deserialize, Serialize};
use super::{Component, Property, Organization};

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub timestamp: String,
    pub tools: Tools,
    pub component: Component,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<Property>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tools {
    pub components: Vec<ToolComponent>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolComponent {
    #[serde(rename = "bom-ref")]
    pub bom_ref: String,
    pub manufacturer: Organization,
    pub name: String,
    #[serde(rename = "type")]
    pub component_type: String,
    pub version: String,
}

