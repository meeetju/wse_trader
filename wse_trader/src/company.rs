#[derive(Clone, Debug)]
pub struct Company{
    pub name: String,
    pub ticker: String,
    base_link: String,
    pub altman: String,
    pub f_score: f32,
    pub pe: f32,
    pub roe: f32,
    pub p_bv: f32,
    pub p_bvg: f32,
    pub dividend_years: Vec<String>, 
}

impl Company {
    pub fn get_indicators_link(self) -> String {
        format!("{}{}-{}/wskazniki-finansowe", self.base_link, self.name, self.ticker)
    }
}

impl Default for Company {
    fn default() -> Self {
        Self { 
            name: "".to_string(),
            ticker: "".to_string(),
            base_link: "https://strefainwestorow.pl/notowania/gpw/".to_string(),
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