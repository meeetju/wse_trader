use colored::Colorize;
use log::info;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct UrlsModifier {
    pub path: String,
    map: HashMap<String, String>,
}

impl UrlsModifier {
    pub fn new(path: String) -> Self {
        Self {
            path: path.clone(),
            map: serde_any::from_file(&path).expect(&format!("Could not read from file {}", &path)),
        }
    }
    pub fn modify(self, input_url: String) -> String {
        for (default_url_content, replace_url_content) in self.map {
            if input_url.contains(&default_url_content) {
                info!(
                    "Modify link content {} -> {}, based on links_mapping.yaml",
                    &default_url_content.red(),
                    &replace_url_content.green()
                );
                return input_url.replace(&default_url_content, &replace_url_content);
            }
        }
        input_url
    }
}
