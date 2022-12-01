use crate::company::Company;
use core::fmt::Debug;
use csv::Writer;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;

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
        wtr.write_record(&[
            "name",
            "ticker",
            "altman",
            "piotroski",
            "pe",
            "roe",
            "p_bv",
            "p_bvg",
        ])?;
        for company in companies_list {
            wtr.write_record(&[
                &company.name,
                &company.ticker,
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
                "{}, {}, {}, {}, {}, {}, {}, {}",
                company.name,
                company.ticker,
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
        let mut companies: Vec<JsonCompany> = Vec::new();

        for company in companies_list {
            let cmp = JsonCompany {
                name: company.name,
                ticker: company.ticker,
                altman: company.altman,
                piotroski: company.f_score.to_string(),
                pe: company.pe.to_string(),
                roe: company.roe.to_string(),
                p_bv: company.p_bv.to_string(),
                p_bvg: company.p_bvg.to_string(),
            };

            companies.push(cmp);
        }

        serde_json::to_string(&companies).unwrap()
    }
}

impl Output for JsonWriter {
    type Out = String;

    fn write(&self, companies_list: Vec<Company>) -> Result<Self::Out, Box<dyn Error>> {
        Ok(self.create_json(companies_list))
    }
}

#[derive(Serialize)]
pub struct JsonCompany {
    name: String,
    ticker: String,
    altman: String,
    piotroski: String,
    pe: String,
    roe: String,
    p_bv: String,
    p_bvg: String,
}
