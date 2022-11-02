use crate::company::{self, Company};
use crate::requirements::{StockRequirements};
use crate::results_writer::{CsvWriter};
use regex::{Regex};

#[derive(Debug)]
pub struct RankedCompanies {
    companies_list: Vec<company::Company>,
    requirements: StockRequirements,
    url: String,
    results_writer: CsvWriter
}

impl RankedCompanies {
    pub fn new(requirements: StockRequirements, results_writer: CsvWriter) -> Self  {
        let companies_list = Vec::new();
        let requirements = requirements;
        let url = "https://www.biznesradar.pl/spolki-rating/akcje_gpw".to_string();
        let results_writer = results_writer;
        Self {companies_list, requirements, url, results_writer}
    }

    pub fn get_companies(&mut self) -> &mut Self {
        let res = reqwest::blocking::get(self.url.clone()).unwrap();
        let content = res.text().unwrap();
        let table = table_extract::Table::find_first(&content).unwrap();

        for row in table.into_iter() {
            let cells = row.as_slice();
            
            if !Self::is_the_row_with_data(&cells) {continue;} 
            let mut company = Company::default();

            match self.get_name(cells[0].clone()) {
                Ok(content) => {company.name = content.to_string();},
                Err(_) => continue
            }

            match self.get_ticker(cells[0].clone()) {
                Ok(content) => {company.ticker = content.to_string();},
                Err(_) => continue
            }

            match self.get_altman_rating(cells[2].clone()) {
                Ok(content) => {company.altman = content.to_string();},
                Err(_) => continue
            }

            match self.get_piotroski_f_score(cells[3].clone()) {
                Ok(content) => {company.f_score = content.parse().unwrap()},
                Err(_) => continue
            }

            if self.is_altman_ok(company.altman.clone()) && self.is_piotroski_ok(company.f_score.clone()) {
                self.companies_list.push(company);
            }
        } 
        self
    }

    fn is_the_row_with_data(cells: &[String]) -> bool {cells.len() == 4}

    pub fn update_indicators(&mut self) -> &mut Self {
        let mut companies_after_update = vec![];    
        for mut company in self.companies_list.clone().into_iter() {
            let indicators_link = company.clone().get_indicators_link();
            let res = reqwest::blocking::get(&indicators_link).unwrap();
            let content = res.text().unwrap();
            let table = table_extract::Table::find_first(&content).unwrap();

            println!("Getting data from {}", &indicators_link);

            let rows: Vec<&[String]> = table.into_iter().map(|row| row.as_slice()).collect();

            match self.get_float_value(rows[0][1].clone()) {
                Ok(content) => {company.pe = content.parse().unwrap();},
                Err(_) => continue
            }

            match self.get_float_value(rows[10][1].clone()) {
                Ok(content) => {company.roe = content.parse().unwrap();},
                Err(_) => continue
            }

            match self.get_float_value(rows[1][1].clone()) {
                Ok(content) => {company.p_bv = content.parse().unwrap();},
                Err(_) => continue
            }

            match self.get_float_value(rows[2][1].clone()) {
                Ok(content) => {company.p_bvg = content.parse().unwrap();},
                Err(_) => continue
            }

            if self.is_pe_ok(company.pe.clone()) && self.is_roe_ok(company.roe.clone()) && self.is_p_bv_ok(company.p_bv.clone()) && self.is_p_bvg_ok(company.p_bvg.clone()) {
                companies_after_update.push(company);
            }
    
        }
        self.companies_list = companies_after_update;
        self
    }

    pub fn print_results(self) {
        for company in self.companies_list.into_iter() {
            println!("{:#?}", company);
        }
    }

    pub fn write_results(self) {
        self.results_writer.write(self.companies_list);
    }

    fn is_altman_ok(&self, altman: String) -> bool {
        self.requirements.ratings.contains(&altman)
    }

    fn is_piotroski_ok(&self, f_score: f32) -> bool {
        self.requirements.f_score_min_limit <= f_score
    }

    fn is_pe_ok(&self, pe: f32) -> bool {
        self.requirements.p_e_max_limit >= pe
    }

    fn is_roe_ok(&self, roe: f32) -> bool {
        self.requirements.roe_min_limit <= roe
    }

    fn is_p_bv_ok(&self, p_bv: f32) -> bool {
        self.requirements.p_bv_max_limit >= p_bv
    }

    fn is_p_bvg_ok(&self, p_bvg: f32) -> bool {
        self.requirements.p_bv_g_max_limit >= p_bvg
    }

    fn get_ticker(&self, html: String) -> Result<String, String> {
        let re = Regex::new(r">([A-Z0-9]{3})").unwrap();
        self.get_regex_from_html(html, re, "Ticker not found".to_string())
    }
    
    fn get_name(&self, html: String) -> Result<String, String> {
        let re = Regex::new(r"\(([A-Z0-9]*)\)").unwrap();
        self.get_regex_from_html(html, re, "Ticker not found".to_string())
    }
    
    fn get_altman_rating(&self, html: String) -> Result<String, String> {
        let re = Regex::new(r">([A-D]{1,3}[\+\-]*)</span>").unwrap();
        self.get_regex_from_html(html, re, "Altman rating not found".to_string())
    }
    
    fn get_piotroski_f_score(&self, html: String) -> Result<String, String> {
        let re = Regex::new(r">([0-9])</span>").unwrap();
        self.get_regex_from_html(html, re, "Piotroski F-Score not found".to_string())
    }
    
    fn get_float_value(&self, html: String) -> Result<String, String> {
        let re = Regex::new(r">([0-9]*.[0-9]*)%?</div>").unwrap();
        self.get_regex_from_html(html.clone(), re, "Float value not found".to_string())
    }
    
    fn get_regex_from_html(&self, html: String, re: Regex, message: String) -> Result<String, String> {
        let captures_collection = re.captures_iter(&html).collect::<Vec<regex::Captures<>>>();
        match captures_collection.get(0) {
            Some(captures) => {
                match captures.get(1) {
                    Some(content) => Ok(html[content.start()..content.end()].to_string()),
                    None => Err(message)
                }
            },
            None => Err(message)
        }
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_ticker_is_extracted() {
//         let html = "<a class=\"s_tt s_tt_sname_IFC\" href=\"/rating/IFC\">IFC (IFCAPITAL)</a>";
//         let result = get_ticker(html);

//         assert!(result.unwrap() == "IFC", "Ticker not extracted properly: {:#?}", result);
//     }

//     #[test]
//     fn test_name_is_extracted() {
//         let html = "<a class=\"s_tt s_tt_sname_IFC\" href=\"/rating/IFC\">IFC (IFCAPITAL)</a>";
//         let result = get_name(html);

//         assert!(result.unwrap() == "IFCAPITAL", "Ticker not extracted properly: {:#?}", result);
//     }
    
//     #[test]
//     fn test_altman_is_extracted() {
//         let html = "<span style=\"color:#03AD01\">AAA</span>";
//         let result = get_altman_rating(html);

//         assert!(result.unwrap() == "AAA", "Ticker not extracted properly: {:#?}", result);

//         let html = "<span style=\"color:#595959\">BBB+</span>";
//         let result = get_altman_rating(html);

//         assert!(result.unwrap() == "BBB+", "Ticker not extracted properly: {:#?}", result);

//         let html = "<span style=\"color:#BD2222\">B-</span>";
//         let result = get_altman_rating(html);

//         assert!(result.unwrap() == "B-", "Ticker not extracted properly: {:#?}", result);
//     }

//     #[test]
//     fn test_piotroski_is_extracted() {
//         let html = "<span style=\"color:#2D832C\">7</span>";
//         let result = get_piotroski_f_score(html);

//         assert!(result.unwrap() == "7", "Ticker not extracted properly: {:#?}", result);
//     }

//     #[test]
//     fn test_pe_is_extracted() {
//         let html = "<div class=\"field field-name-field-c-z field-type-number-decimal field-label-hidden\"><div class=\"field-items\"><div class=\"field-item even\">24.06</div></div></div>";
//         let result = get_float_value(html);

//         assert!(result.unwrap() == "24.06", "Ticker not extracted properly: {:#?}", result);
//     }

//     #[test]
//     fn test_roe_is_extracted() {
//         let html = "<div class=\"field field-name-field-roe field-type-number-decimal field-label-hidden\"><div class=\"field-items\"><div class=\"field-item even\">5.73%</div></div></div>";
//         let result = get_float_value(html);

//         assert!(result.unwrap() == "5.73", "Ticker not extracted properly: {:#?}", result);
//     }
// }