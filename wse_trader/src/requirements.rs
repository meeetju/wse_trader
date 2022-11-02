use std::vec;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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

impl StockRequirements {

    pub fn new() -> Self {
        StockRequirements::default()
    }

    pub fn update(mut self, path: String) -> Self {
        self = self.read_from_yaml(path);
        self
    }

    fn read_from_yaml(self, path: String) -> Self {
        let f = std::fs::File::open(&path).expect("Could not open file.");
        serde_yaml::from_reader(f).expect("Could not read values.")
    }

}
