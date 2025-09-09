use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct ModelInfo {
    #[serde(rename = "modelId")]
    pub model_id: String,
    pub tags: Vec<String>,
    #[allow(dead_code)]
    pub library_name: Option<String>,
    #[serde(rename = "createdAt")]
    #[allow(dead_code)]
    pub created_at: Option<String>,
    #[serde(rename = "lastModified")]
    #[allow(dead_code)]
    pub last_modified: Option<String>,
    pub license: Option<String>,
    #[serde(rename = "cardData")]
    pub card_data: Option<Value>,
    #[allow(dead_code)]
    pub siblings: Option<Vec<Value>>,
    #[allow(dead_code)]
    pub sha: Option<String>,
}