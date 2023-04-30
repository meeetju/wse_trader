use core::fmt::Debug;
use csv::Writer;
use std::error::Error;

use common::types::shared_types::Company;

pub trait Output {
    type Out;

    fn write(&self, companies_list: Vec<Company>) -> Result<Self::Out, Box<dyn Error>>;
}

#[derive(Debug)]
pub struct CsvWriter {
    pub path: String,
}

impl Output for CsvWriter {
    type Out = ();
    fn write(&self, companies_list: Vec<Company>) -> Result<Self::Out, Box<dyn Error>> {
        let mut wtr = Writer::from_path(&self.path)?;
        wtr.write_record([
            "name",
            "ticker",
            "link",
            "altman",
            "piotroski",
            "pe",
            "roe",
            "p_bv",
            "p_bvg",
        ])?;
        for company in companies_list {
            wtr.write_record([
                &company.name,
                &company.ticker,
                &company.link,
                &company.altman,
                &company.f_score.to_string(),
                &company.pe.to_string(),
                &company.roe.to_string(),
                &company.p_bv.to_string(),
                &company.p_bvg.to_string(),
            ])?;
        }
        wtr.flush()?;
        Ok(())
    }
}

pub struct ConsolePrinter;

impl Output for ConsolePrinter {
    type Out = ();
    fn write(&self, companies_list: Vec<Company>) -> Result<Self::Out, Box<dyn Error>> {
        println!("name,ticker,altman,piotroski,pe,roe,p_bv,p_bvg");
        for company in companies_list {
            println!(
                "{}, {}, {}, {}, {}, {}, {}, {}, {}",
                company.name,
                company.ticker,
                company.link,
                company.altman,
                company.f_score,
                company.pe,
                company.roe,
                company.p_bv,
                company.p_bvg
            );
        }
        Ok(())
    }
}

pub struct JsonWriter;

impl JsonWriter {
    fn create_json(&self, companies_list: Vec<Company>) -> String {
        serde_json::to_string(&companies_list).unwrap()
    }
}

impl Output for JsonWriter {
    type Out = String;

    fn write(&self, companies_list: Vec<Company>) -> Result<Self::Out, Box<dyn Error>> {
        Ok(self.create_json(companies_list))
    }
}
