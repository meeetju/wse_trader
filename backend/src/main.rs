mod cli;
mod errors;
mod lazy_regexps;
mod ranked_companies;
mod requirements_reader;
mod results_writer;
mod urls;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use env_logger::Builder as LoggerBuilder;
use futures::lock::Mutex;
use log::{error, info};
use std::io::Write;
use std::sync::Arc;

use crate::errors::Error;
use crate::ranked_companies::RankedCompanies;
use crate::requirements_reader::{WebJsonReader, YamlReader};
use crate::results_writer::JsonWriter;
use crate::urls::{CompanyDataUrlProvider, UrlsModifier};
use common::types::shared_types::{ServerConfig, StockRequirements};

#[actix_web::main]
async fn main() {
    match run().await {
        Ok(_) => info!("Bye!"),
        Err(error) => error!("Something went wrong: {}", error),
    }
}

async fn run() -> Result<(), Error> {
    let mut logger_builder = LoggerBuilder::new();
    logger_builder
        .parse_default_env()
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .filter(None, log::LevelFilter::Info)
        .init();

    let cli_params = cli::Cli::parse();

    let server_config = ServerConfig {
        address: cli_params.oa.unwrap(),
        port: cli_params.op.unwrap(),
    };

    info!("### Backend ready ###");
    info!("Using local address: {:#?}", server_config.get_url());

    let address = server_config.address.clone();
    let port = server_config.port.parse::<u16>().unwrap();

    let mut ranked = RankedCompanies::new();
    ranked.set_companies_list_url(cli_params.companies_list_url);

    let company_data_url_provider = CompanyDataUrlProvider::new(
        cli_params.company_indicators_url,
        Some(UrlsModifier::new("links_mapping.yaml".to_string())),
    );

    ranked.set_company_data_url_provider(company_data_url_provider);
    ranked.update_requirements(YamlReader {
        path: "requirements.yaml".to_string(),
    })?;

    let ranked = Arc::new(Mutex::new(ranked));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ranked.clone()))
            .service(update_requirements)
            .service(best_companies)
    })
    .bind((address, port))?
    .run()
    .await?;

    Ok(())
}

#[post("/update_requirements")]
async fn update_requirements(
    requirements: web::Json<StockRequirements>,
    ranked: web::Data<Arc<Mutex<RankedCompanies>>>,
) -> impl Responder {
    info!("Update results requested");

    let in_requirements: StockRequirements = requirements.to_owned();
    ranked.lock().await.update_requirements(WebJsonReader {
        in_requirements: in_requirements.clone(),
    });
    HttpResponse::Ok().body("Requirements updated!")
}

#[get("/search_companies")]
async fn best_companies(ranked: web::Data<Arc<Mutex<RankedCompanies>>>) -> impl Responder {
    info!("Search companies requested");

    ranked.lock().await.get_companies().await;
    ranked.lock().await.update_indicators().await;
    ranked.lock().await.filter_best_companies().await;
    let companies = ranked.lock().await.write_results(JsonWriter {}).await;
    info!("{}", &companies);

    HttpResponse::Ok().body(companies)
}
