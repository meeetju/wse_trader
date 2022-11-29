use crate::company::{self, Company};
use crate::requirements_reader::{StockRequirements, Read};
use crate::results_writer::Output;
use crate::urls_modifier::UrlsModifier;
use crate::lazy_regexps::{RE_ALTMAN, RE_FLOAT, RE_F_SCORE, RE_NAME, RE_TICKER};
use crate::errors::NotFoundError;

use std::sync::Arc;
use futures::lock::Mutex;
use colored::Colorize;
use log::{info, warn};

#[derive(Debug)]
pub struct RankedCompanies {
    companies_list: Arc<Mutex<Vec<company::Company>>>,
    requirements: StockRequirements,
    url: String,
    urls_modifier: UrlsModifier
}

impl RankedCompanies {
    pub fn new() -> Self  {
        Self {
            companies_list: Arc::new(Mutex::new(Vec::new())),
            requirements: StockRequirements::default(),
            url: "https://www.biznesradar.pl/spolki-rating/akcje_gpw".to_string(),
            urls_modifier: UrlsModifier::default()
        }
    }

    pub fn update_requirements<T>(&mut self, reader: T)
        where T: Read,
    {
        self.requirements = reader.read();
    }

    pub fn update_url_mappings(&mut self, modifier: UrlsModifier) {
        self.urls_modifier = modifier;
    }

    pub async fn get_companies(&mut self) {
        let client = reqwest::Client::new(); 
        let response = client.get(self.url.clone()).send().await.unwrap();
        let content = response.text().await.unwrap();
        
        let table = table_extract::Table::find_first(&content).unwrap();

        for row in table.into_iter() {
            let cells = row.as_slice();
            
            if !Self::is_the_row_with_data(cells) {continue;} 
            let mut company = Company::default();

            match Self::get_name(cells[0].clone()) {
                Ok(content) => {company.name = content.to_string();},
                Err(_) => match Self::get_ticker(cells[0].clone()) {
                                Ok(content) => {company.name = content.to_string();},
                                Err(_) => continue
                        }
            }

            match Self::get_ticker(cells[0].clone()) {
                Ok(content) => {company.ticker = content.to_string();},
                Err(_) => continue
            }

            match Self::get_altman_rating(cells[2].clone()) {
                Ok(content) => {company.altman = content.to_string();},
                Err(_) => continue
            }

            match Self::get_piotroski_f_score(cells[3].clone()) {
                Ok(content) => {company.f_score = content.parse().unwrap()},
                Err(_) => continue
            }

            if self.is_altman_ok(&company.altman) && self.is_piotroski_ok(company.f_score) {
                self.companies_list.lock().await.push(company);
            }
        } 
    }

    pub async fn update_indicators(&mut self) {
        Self::update_indicators_async(&self.companies_list, self.urls_modifier.clone()).await;
    }

    async fn update_indicators_async(companies_list: &Arc<Mutex<Vec<company::Company>>>, modifier: UrlsModifier) {

        let size = companies_list.lock().await.len();
        let mut handlers = Vec::new();

        for i in 0..size {

            let modifier = modifier.clone();
            let list = Arc::clone(companies_list);
            let handle = tokio::spawn(async move {
                Self::update_company_indicators(list, i, modifier).await;
            });
            handlers.push(handle)
        }

        for handler in handlers {
            handler.await.unwrap();
        }
    }

    async fn update_company_indicators(companies_list: Arc<Mutex<Vec<company::Company>>>, index: usize, modifier: UrlsModifier){

        let company = &mut companies_list.lock().await[index];

        let mut indicators_link = company.clone().get_indicators_link();
        indicators_link = modifier.modify(indicators_link);

        let client = reqwest::Client::new(); 
        let response = client.get(&indicators_link).send().await.unwrap();
        let content = response.text().await.unwrap();

        let table = table_extract::Table::find_first(&content).unwrap();
        info!("Getting data from {}", &indicators_link);
        let rows: Vec<&[String]> = table.into_iter().map(|row| row.as_slice()).collect();

        match rows.len() {
            11 => {
                if let Ok(content) = Self::get_float_value(rows[0][1].clone()) {company.pe = content.parse().unwrap();}
                if let Ok(content) = Self::get_float_value(rows[10][1].clone()) {company.roe = content.parse().unwrap();}
                if let Ok(content) = Self::get_float_value(rows[1][1].clone()) {company.p_bv = content.parse().unwrap();}
                if let Ok(content) = Self::get_float_value(rows[2][1].clone()) {company.p_bvg = content.parse().unwrap();}
            },
            _ => {
                let warning = format!("{}" , NotFoundError::WrongLinkError(indicators_link)).yellow();
                warn!("{}", warning);
            }

        }
    }

    pub async fn filter_best_companies(&mut self) {
        let mut companies_after_update = vec![];
        for company in self.companies_list.lock().await.iter() {
            if self.is_pe_ok(company.pe) && self.is_roe_ok(company.roe) && self.is_p_bv_ok(company.p_bv) && self.is_p_bvg_ok(company.p_bvg) {
                companies_after_update.push(company.clone());
            }
        }
        self.companies_list = Arc::new(Mutex::new(companies_after_update));
    }

    pub async fn write_results<T>(self, writer: T) 
    where T: Output,
    {
        match writer.write(self.companies_list.lock().await.to_vec()) {
            Ok(_) => (),
            Err(msg) => println!("{:#?}", msg)
        }
    }

    fn is_the_row_with_data(cells: &[String]) -> bool {cells.len() == 4}

    fn is_altman_ok(&self, altman: &String) -> bool {
        self.requirements.ratings.contains(altman)
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

    fn get_ticker(html: String) -> Result<String, NotFoundError> {
        Self::get_regex_from_html(html, RE_TICKER, NotFoundError::TickerError())
    }
    
    fn get_name(html: String) -> Result<String, NotFoundError> {
        Self::get_regex_from_html(html, RE_NAME, NotFoundError::NameError())
    }
    
    fn get_altman_rating(html: String) -> Result<String, NotFoundError> {
        Self::get_regex_from_html(html, RE_ALTMAN, NotFoundError::AltmanError())
    }
    
    fn get_piotroski_f_score(html: String) -> Result<String, NotFoundError> {
        Self::get_regex_from_html(html, RE_F_SCORE, NotFoundError::FScoreError())
    }
    
    fn get_float_value(html: String) -> Result<String, NotFoundError> {
        Self::get_regex_from_html(html, RE_FLOAT, NotFoundError::FloatError())
    }
    
    fn get_regex_from_html(html: String, re: &lazy_regex::Lazy<lazy_regex::Regex>, error: NotFoundError) -> Result<String,  NotFoundError> {
        let captures_collection = re.captures_iter(&html).collect::<Vec<regex::Captures<>>>();
        match captures_collection.get(0) {
            Some(captures) => {
                match captures.get(1) {
                    Some(content) => Ok(html[content.start()..content.end()].to_string()),
                    None =>  Err(error)
                }
            },
            None => Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticker_is_extracted() {
        let html = "<a class=\"s_tt s_tt_sname_IFC\" href=\"/rating/IFC\">IFC (IFCAPITAL)</a>".to_string();
        let result = RankedCompanies::get_ticker(html);

        assert!(result.as_ref().unwrap() == "IFC", "Ticker not extracted properly: {:#?}", result);
    }

    #[test]
    fn test_name_is_extracted() {
        let html = "<a class=\"s_tt s_tt_sname_IFC\" href=\"/rating/IFC\">IFC (IFCAPITAL)</a>".to_string();
        let result = RankedCompanies::get_name(html);

        assert!(result.as_ref().unwrap() == "IFCAPITAL", "Ticker not extracted properly: {:#?}", result);
    }
    
    #[test]
    fn test_altman_is_extracted() {
        let html = "<span style=\"color:#03AD01\">AAA</span>".to_string();
        let result = RankedCompanies::get_altman_rating(html);

        assert!(result.as_ref().unwrap() == "AAA", "Ticker not extracted properly: {:#?}", result);

        let html = "<span style=\"color:#595959\">BBB+</span>".to_string();
        let result = RankedCompanies::get_altman_rating(html);

        assert!(result.as_ref().unwrap() == "BBB+", "Ticker not extracted properly: {:#?}", result);

        let html = "<span style=\"color:#BD2222\">B-</span>".to_string();
        let result = RankedCompanies::get_altman_rating(html);

        assert!(result.as_ref().unwrap() == "B-", "Ticker not extracted properly: {:#?}", result);
    }

    #[test]
    fn test_piotroski_is_extracted() {
        let html = "<span style=\"color:#2D832C\">7</span>".to_string();
        let result = RankedCompanies::get_piotroski_f_score(html);

        assert!(result.as_ref().unwrap() == "7", "Ticker not extracted properly: {:#?}", result);
    }

    #[test]
    fn test_pe_is_extracted() {
        let html = "<div class=\"field field-name-field-c-z field-type-number-decimal field-label-hidden\"><div class=\"field-items\"><div class=\"field-item even\">24.06</div></div></div>".to_string();
        let result = RankedCompanies::get_float_value(html);

        assert!(result.as_ref().unwrap() == "24.06", "Ticker not extracted properly: {:#?}", result);
    }

    #[test]
    fn test_roe_is_extracted() {
        let html = "<div class=\"field field-name-field-roe field-type-number-decimal field-label-hidden\"><div class=\"field-items\"><div class=\"field-item even\">5.73%</div></div></div>".to_string();
        let result = RankedCompanies::get_float_value(html);

        assert!(result.as_ref().unwrap() == "5.73", "Ticker not extracted properly: {:#?}", result);
    }
}