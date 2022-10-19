use crate::company;

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
        let table = table_extract::Table::find_first(&res.text().unwrap()).unwrap();
        println!("{:#?}", table);
    }
}