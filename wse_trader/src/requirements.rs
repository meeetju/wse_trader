use std::vec;

use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct Requirements {
    requirements: StockRequirements,
}

impl Requirements {
    pub fn new() -> Requirements {
        Requirements { requirements: StockRequirements::default() }
    }
}

#[derive(Debug, Deserialize)]
pub struct StockRequirements {
    p_e_max_limit: f32,
    roe_min_limit: f32,
    dividend_years: Vec<String>,
    p_bv_max_limit: f32,
    p_bv_g_max_limit: f32,
    ratings: Vec<String>,
    f_score: f32,
}

impl Default for StockRequirements {
    fn default() -> StockRequirements {
        StockRequirements {
            p_e_max_limit: 100.0,
            roe_min_limit: 0.0,
            dividend_years: vec![],
            p_bv_max_limit: 100.0,
            p_bv_g_max_limit: 100.0,
            ratings: vec![],
            f_score: 0.0,
        } 
    }
    
}

pub fn read_requirements(file_path: String) -> std::io::Result<Requirements> {
    let requirements = std::fs::read_to_string(file_path)?;
    Ok(toml::from_str(&requirements)?)
}