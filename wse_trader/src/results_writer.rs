use crate::company::{self, Company};
use std::error::Error;
use csv::Writer;

pub fn write(companies_list: Vec<Company>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path("results.csv")?;
    wtr.write_record(&["name", "ticker", "altman", "piotroski", "pe", "roe", "p_bv", "p_bvg"])?;
    for company in companies_list {
        wtr.write_record(&[&company.name, &company.ticker, &company.altman, &company.f_score.to_string(), &company.pe.to_string(), &company.roe.to_string(), &company.p_bv.to_string(), &company.p_bvg.to_string()])?;
    }
    wtr.flush()?;
    Ok(())
}