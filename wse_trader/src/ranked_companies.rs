use crate::company;
use regex::{Regex, Error};
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
                
            let ticker = get_ticker(&cells[0]).unwrap();
            let altman = get_altman_rating(&cells[2]).unwrap();
            println!("{:#?}, {:#?}", ticker, altman);
            
        } 
        
    }
}

fn get_ticker(cell: &str) -> Result<&str, &str> {
    let ticker = Regex::new(r">([A-Z0-9]+)[ (A-Z0-9.\-)]*<").unwrap().find(cell);
    match ticker {
        Some(content) => Ok(&cell[content.start()+1..content.start()+4]),
        None => Err("Ticker not found")
    }
}

fn get_altman_rating(cell: &str) -> Result<&str, &str> {
    let ticker = Regex::new(r">([A-D]+[+-]?)</span").unwrap().find(cell);
    match ticker {
        Some(content) => Ok(&cell[content.start()+1..content.start()+4]),
        None => Err("Altman rating not found")
    }
}