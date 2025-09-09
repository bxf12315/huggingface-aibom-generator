use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Organization {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalReference {
    #[serde(rename = "type")]
    pub ref_type: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}