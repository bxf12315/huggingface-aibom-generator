use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Dependency {
    #[serde(rename = "ref")]
    pub reference: String,
    #[serde(rename = "dependsOn")]
    pub depends_on: Vec<String>,
}