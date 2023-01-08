use serde::Deserialize;
use std::vec;

#[derive(Debug, Deserialize, Clone)]
pub struct StockRequirements {
    pub p_e_max_limit: f32,
    pub roe_min_limit: f32,
    pub dividend_years: Vec<String>,
    pub p_bv_max_limit: f32,
    pub p_bv_g_max_limit: f32,
    pub ratings: Vec<String>,
    pub f_score_min_limit: f32,
}

pub trait Read {
    fn read(&self) -> StockRequirements;
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

pub struct YamlReader {
    pub path: String,
}

impl Read for YamlReader {
    fn read(&self) -> StockRequirements {
        let f = std::fs::File::open(&self.path).expect("Could not open file.");
        let requirements: StockRequirements =
            serde_yaml::from_reader(f).expect("Could not read values.");
        requirements
    }
}
