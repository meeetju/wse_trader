use crate::company::{self, Company};
use regex::{Regex};

#[derive(Debug)]
pub struct RankedCompanies {
    companies_list: Vec<company::Company>,
}

impl RankedCompanies {
    pub fn new() -> RankedCompanies  {
        let mut companies = Vec::new();
        RankedCompanies{companies_list: companies}
    }

    
    pub fn read_companies(&mut self, url: &str) -> &mut Self {
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
        } 
        self
    }

    pub fn update_indicators(self) -> (){
        for company in self.companies_list.into_iter() {
            let res = reqwest::blocking::get(company.get_indicators_link()).unwrap();
            let content = res.text().unwrap();
            let table = table_extract::Table::find_first(&content).unwrap();
            println!("{:#?}", table);
            panic!("")
        }
    }
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