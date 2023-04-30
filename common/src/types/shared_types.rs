use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsArray;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StockRequirements {
    pub p_e_max_limit: f32,
    pub roe_min_limit: f32,
    pub dividend_years: Vec<String>,
    pub p_bv_max_limit: f32,
    pub p_bv_g_max_limit: f32,
    pub ratings: Vec<String>,
    pub f_score_min_limit: f32,
}

impl Default for StockRequirements {
    fn default() -> Self {
        Self {
            p_e_max_limit: 100.0,
            roe_min_limit: 0.0,
            dividend_years: vec![],
            p_bv_max_limit: 100.0,
            p_bv_g_max_limit: 100.0,
            ratings: vec![],
            f_score_min_limit: 0.0,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, FieldNamesAsArray)]
pub struct Company {
    pub name: String,
    pub ticker: String,
    pub link: String,
    pub altman: String,
    pub f_score: f32,
    pub pe: f32,
    pub roe: f32,
    pub p_bv: f32,
    pub p_bvg: f32,
    #[serde(skip)] // TEMP TODO
    pub dividend_years: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub address: String,
    pub port: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            address: "0.0.0.0".to_owned(),
            port: "00".to_owned(),
        }
    }
}

impl ServerConfig {
    pub fn get_url(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}
