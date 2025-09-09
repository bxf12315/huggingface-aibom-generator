use crate::*;

pub struct LicenseHandler;

impl LicenseHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn normalize_license(&self, license: &str, model_info: &ModelInfo) -> Option<LicenseInfo> {
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
                    name: None, // For SPDX licenses, only keep id
                    url: Some(format!("https://spdx.org/licenses/{}", spdx_license.name)),
                    text: None,
                });
            }
        }

        // If not found in SPDX, try to find LICENSE file URL from HuggingFace repo
        if let Some(license_url) = self.find_license_file_url(&model_info.model_id) {
            // Try to get license name from card_data.license_name first, then model_info.license, fallback to original license string
            let license_name = model_info
                .card_data
                .as_ref()
                .and_then(|card_data| card_data.get("license_name"))
                .and_then(|name| name.as_str())
                .map(|s| s.to_string())
                .or_else(|| model_info.license.clone())
                .unwrap_or_else(|| license.to_string());

            return Some(LicenseInfo {
                id: None,
                name: Some(license_name),
                url: Some(license_url),
                text: None,
            });
        }

        // If no license information is available, return None
        None
    }

    fn find_license_file_url(&self, model_id: &str) -> Option<String> {
        use reqwest::blocking::Client;
        use std::time::Duration;

        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .ok()?;

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
                    return Some(url);
                }
            }
        }

        None
    }
}