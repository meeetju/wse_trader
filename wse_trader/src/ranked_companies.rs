use crate::company::{self, Company};
use crate::requirements::{Requirements, read_requirements};
use regex::{Regex};

#[derive(Debug)]
pub struct RankedCompanies {
    companies_list: Vec<company::Company>,
    requirements: Requirements,
}

impl RankedCompanies {
    pub fn new() -> RankedCompanies  {
        let companies = Vec::new();
        let requirements = Requirements::new();
        RankedCompanies{companies_list: companies, requirements: requirements}
    }

    pub fn update_requirements(&mut self , requirements: Requirements) -> &mut Self {
        // println!("{:#?}", self.requirements);
        self.requirements = requirements;
        // println!("{:#?}", self.requirements);
        self
    }

    pub fn get_companies(&mut self, url: &str) -> &mut Self {
        let res = reqwest::blocking::get(url).unwrap();
        let content = res.text().unwrap();
        let table = table_extract::Table::find_first(&content).unwrap();

        for row in table.into_iter() {
            let cells = row.as_slice();
            
            if cells.len() != 4 {
                continue;
            } 
            
            let name: String;
            match get_name(&cells[0]) {
                Ok(content) => {name = content.to_string();},
                Err(_) => continue
            }

            let ticker: String;
            match get_ticker(&cells[0]) {
                Ok(content) => {ticker = content.to_string();},
                Err(_) => continue
            }

            let altman: String;
            match get_altman_rating(&cells[2]) {
                Ok(content) => {altman = content.to_string();},
                Err(_) => continue
            }

            let f_score: f32;
            match get_piotroski_f_score(&cells[3]) {
                Ok(content) => {f_score = content.parse().unwrap()},
                Err(_) => continue
            }

            let mut company = Company::default();
            company.name = name;
            company.ticker = ticker.to_string();
            company.altman = altman.to_string();
            company.f_score = f_score;

            self.companies_list.push(company);
            break
        } 
        self
    }

    pub fn update_indicators(&mut self) -> &mut Self {
        for mut company in self.companies_list.iter_mut() {
            let temp_company = company.clone();
            let indicators_link = temp_company.get_indicators_link();
            let res = reqwest::blocking::get(&indicators_link).unwrap();
            let content = res.text().unwrap();
            let table = table_extract::Table::find_first(&content).unwrap();

            println!("Getting data from {}", &indicators_link);

            for row in table.into_iter() {
                let cells = row.as_slice();

                let pe: f32;
                match get_float_value(&cells[1]) {
                    Ok(content) => {pe = content.parse().unwrap();},
                    Err(_) => continue
                }

                let roe: f32;
                match get_float_value(&cells[1]) {
                    Ok(content) => {roe = content.parse().unwrap();},
                    Err(_) => continue
                }

                let p_bv: f32;
                match get_float_value(&cells[1]) {
                    Ok(content) => {p_bv = content.parse().unwrap();},
                    Err(_) => continue
                }

                let p_bvg: f32;
                match get_float_value(&cells[1]) {
                    Ok(content) => {p_bvg = content.parse().unwrap();},
                    Err(_) => continue
                }

                company.pe = pe;
                company.roe = roe;
                company.p_bv = p_bv;
                company.p_bvg = p_bvg;
            }
        }
        self
    }

    pub fn print_results(self) {
        for company in self.companies_list.into_iter() {
            println!("{:#?}", company);
        }
    }

    // fn is_altman_ok(self, altman: String) -> bool {
    //     self.requirements.
    // }
}

fn get_ticker(html: &str) -> Result<&str, &str> {
    let re = Regex::new(r">([A-Z0-9]{3})").unwrap();
    get_regex_from_html(html, re, "Ticker not found")
}

fn get_name(html: &str) -> Result<&str, &str> {
    let re = Regex::new(r"\(([A-Z0-9]*)\)").unwrap();
    get_regex_from_html(html, re, "Ticker not found")
}

fn get_altman_rating(html: &str) -> Result<&str, &str> {
    let re = Regex::new(r">([A-D]{1,3}[\+\-]*)</span>").unwrap();
    get_regex_from_html(html, re, "Altman rating not found")
}

fn get_piotroski_f_score(html: &str) -> Result<&str, &str> {
    let re = Regex::new(r">([0-9])</span>").unwrap();
    get_regex_from_html(html, re, "Piotroski F-Score not found")
}

fn get_float_value(html: &str) -> Result<&str, &str> {
    let re = Regex::new(r">([0-9]*.[0-9]*)%?</div>").unwrap();
    get_regex_from_html(html, re, "Float value not found")
}

fn get_regex_from_html<'a>(html: &'a str, re: Regex, message: &'a str) -> Result<&'a str, &'a str> {
    let captures_collection = re.captures_iter(html).collect::<Vec<regex::Captures<'_>>>();
    match captures_collection.get(0) {
        Some(captures) => {
            match captures.get(1) {
                Some(content) => Ok(&html[content.start()..content.end()]),
                None => Err(&message)
            }
        },
        None => Err(&message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticker_is_extracted() {
        let html = "<a class=\"s_tt s_tt_sname_IFC\" href=\"/rating/IFC\">IFC (IFCAPITAL)</a>";
        let result = get_ticker(html);

        assert!(result.unwrap() == "IFC", "Ticker not extracted properly: {:#?}", result);
    }

    #[test]
    fn test_name_is_extracted() {
        let html = "<a class=\"s_tt s_tt_sname_IFC\" href=\"/rating/IFC\">IFC (IFCAPITAL)</a>";
        let result = get_name(html);

        assert!(result.unwrap() == "IFCAPITAL", "Ticker not extracted properly: {:#?}", result);
    }
    
    #[test]
    fn test_altman_is_extracted() {
        let html = "<span style=\"color:#03AD01\">AAA</span>";
        let result = get_altman_rating(html);

        assert!(result.unwrap() == "AAA", "Ticker not extracted properly: {:#?}", result);

        let html = "<span style=\"color:#595959\">BBB+</span>";
        let result = get_altman_rating(html);

        assert!(result.unwrap() == "BBB+", "Ticker not extracted properly: {:#?}", result);

        let html = "<span style=\"color:#BD2222\">B-</span>";
        let result = get_altman_rating(html);

        assert!(result.unwrap() == "B-", "Ticker not extracted properly: {:#?}", result);
    }

    #[test]
    fn test_piotroski_is_extracted() {
        let html = "<span style=\"color:#2D832C\">7</span>";
        let result = get_piotroski_f_score(html);

        assert!(result.unwrap() == "7", "Ticker not extracted properly: {:#?}", result);
    }

    #[test]
    fn test_pe_is_extracted() {
        let html = "<div class=\"field field-name-field-c-z field-type-number-decimal field-label-hidden\"><div class=\"field-items\"><div class=\"field-item even\">24.06</div></div></div>";
        let result = get_float_value(html);

        assert!(result.unwrap() == "24.06", "Ticker not extracted properly: {:#?}", result);
    }

    #[test]
    fn test_roe_is_extracted() {
        let html = "<div class=\"field field-name-field-roe field-type-number-decimal field-label-hidden\"><div class=\"field-items\"><div class=\"field-item even\">5.73%</div></div></div>";
        let result = get_float_value(html);

        assert!(result.unwrap() == "5.73", "Ticker not extracted properly: {:#?}", result);
    }
}