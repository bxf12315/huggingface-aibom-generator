use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Dependency {
    #[serde(rename = "ref")]
    pub reference: String,
    #[serde(rename = "dependsOn")]
    pub depends_on: Vec<DependencyReference>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DependencyReference {
    #[serde(rename = "ref")]
    pub reference: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}