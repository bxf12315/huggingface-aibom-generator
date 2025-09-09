use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::Property;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelCard {
    #[serde(rename = "modelParameters", skip_serializing_if = "Option::is_none")]
    pub model_parameters: Option<ModelParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<Property>>,
    #[serde(
        rename = "quantitativeAnalysis",
        skip_serializing_if = "Option::is_none"
    )]
    pub quantitative_analysis: Option<QuantitativeAnalysis>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelParameters {
    #[serde(rename = "architectureFamily", skip_serializing_if = "Option::is_none")]
    pub architecture_family: Option<String>,
    #[serde(rename = "modelArchitecture", skip_serializing_if = "Option::is_none")]
    pub model_architecture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<InputOutputData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<InputOutputData>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputOutputData {
    pub format: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuantitativeAnalysis {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphics: Option<Value>,
    #[serde(rename = "performanceMetrics", skip_serializing_if = "Option::is_none")]
    pub performance_metrics: Option<Vec<PerformanceMetric>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PerformanceMetric {
    #[serde(rename = "type")]
    pub metric_type: String,
    pub value: String,
}