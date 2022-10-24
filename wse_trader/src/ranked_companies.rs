use crate::company;
use regex::{Regex, Error, CaptureMatches};
use serde::__private::de::Content;

#[derive(Debug)]
pub struct RankedCompanies {
    companies_list: Vec<company::Company>,
}

impl RankedCompanies {
    pub fn new() -> RankedCompanies {
        let mut companies = Vec::new();
        RankedCompanies{companies_list: companies}
    }

    pub fn read_companies(self, url: &str) {
        let res = reqwest::blocking::get(url).unwrap();
        let content = res.text().unwrap();
        // println!("{:#?}", content);
        let table = table_extract::Table::find_first(&content).unwrap();
        for row in table.into_iter() {
            let cells = row.as_slice();
            
            if cells.len() != 4 {
                continue;
            } 
                
            let ticker = get_ticker(&cells[0]);
            let altman = get_altman_rating(&cells[2]);
            let f_score = get_piotroski_f_score(&cells[3]);
            println!("{:#?}, {:#?}, {:#?}", ticker, altman, f_score);
            // panic!("");
            
        } 
        
    }
}

fn get_ticker(html: &str) -> Result<&str, &str> {
    let re = Regex::new(r">([A-Z0-9]{3})").unwrap();
    get_regex_from_html(html, re, "Ticker not found")
}

fn get_altman_rating(html: &str) -> Result<&str, &str> {
    let re = Regex::new(r">([A-D]{1,3})(\+|\-|)</span>").unwrap();
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