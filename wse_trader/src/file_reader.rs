use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct Requirements {
    requirements: StockRequirements,
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

pub fn read_requirements(file_path: String) -> std::io::Result<Requirements> {
    let requirements = std::fs::read_to_string(file_path)?;
    Ok(toml::from_str(&requirements)?)
}