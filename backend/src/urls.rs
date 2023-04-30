use colored::Colorize;
use log::info;
use std::collections::HashMap;

/// Provide url for given company
#[derive(Debug, Clone, Default)]
pub struct CompanyDataUrlProvider {
    pub base_link: String,
    urls_modifier: Option<UrlsModifier>,
}

impl CompanyDataUrlProvider {
    pub fn new(base_link: String, urls_modifier: Option<UrlsModifier>) -> Self {
        Self {
            base_link,
            urls_modifier,
        }
    }
    pub fn get_company_data_url(&mut self, company_name: &str, company_ticker: &str) -> String {
        let link = format!(
            "{}{}-{}/wskazniki-finansowe",
            self.base_link, company_name, company_ticker
        )
        .to_lowercase();
        self.update_indicators_link(link)
    }

    pub fn update_indicators_link(&mut self, link: String) -> String {
        match &self.urls_modifier {
            Some(modifier) => modifier.clone().modify(link),
            None => link,
        }
    }
}

/// Modify company data link
#[derive(Debug, Clone, Default)]
pub struct UrlsModifier {
    pub path: String,
    map: HashMap<String, String>,
}

impl UrlsModifier {
    pub fn new(path: String) -> Self {
        Self {
            path: path.clone(),
            map: serde_any::from_file(&path)
                .unwrap_or_else(|_| panic!("Could not read from file {}", &path)),
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
