use crate::company::Company;
use std::error::Error;
use csv::Writer;
use core::fmt::Debug;

pub trait Output {
    fn write(&self, companies_list: Vec<Company>) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug)]
pub struct CsvWriter {
    pub path: String
}

impl Output for CsvWriter {
    fn write(&self, companies_list: Vec<Company>) -> Result<(), Box<dyn Error>> {
        let mut wtr = Writer::from_path(&self.path)?;
        wtr.write_record(&["name", "ticker", "altman", "piotroski", "pe", "roe", "p_bv", "p_bvg"])?;
        for company in companies_list {
            wtr.write_record(&[&company.name, &company.ticker, &company.altman, &company.f_score.to_string(), &company.pe.to_string(), &company.roe.to_string(), &company.p_bv.to_string(), &company.p_bvg.to_string()])?;
        }
        wtr.flush()?;
        Ok(())
    }
}

pub struct ConsolePrinter {}

impl Output for ConsolePrinter {
    fn write(&self, companies_list: Vec<Company>) -> Result<(), Box<dyn Error>> {
        println!("name,ticker,altman,piotroski,pe,roe,p_bv,p_bvg");
        for company in companies_list {
            println!("{}, {}, {}, {}, {}, {}, {}, {}", company.name, company.ticker, company.altman, company.f_score, company.pe, company.roe, company.p_bv, company.p_bvg);
        }
        Ok(())
    }
}