#[derive(Debug)]
pub struct Company{
    pub ticker: String,
    pub altman: String,
    pub f_score: f32,
    pub pe: f32,
    pub roe: f32,
    pub p_bv: f32,
    pub p_bvg: f32,
    pub dividend_years: Vec<String>, 
}

impl Default for Company {
    fn default() -> Company {
        Company { 
            ticker: "".to_string(),
            altman: "".to_string(),
            f_score: 0.0,
            pe: 0.0,
            roe: 0.0,
            p_bv: 0.0,
            p_bvg: 0.0,
            dividend_years: vec![],
        }
    }
}