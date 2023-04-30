mod cli;
mod errors;
mod types;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use common::types::shared_types::{Company, ServerConfig, StockRequirements};
use core::panic;
use env_logger::Builder as LoggerBuilder;
use errors::Error;
use log::{error, info};
use serde_qs::Config;
use std::io::Write;
use std::{include_bytes, str};
use types::BackendUrls;

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

    let server_config_cli = cli::Cli::parse();
    let own_server_config = ServerConfig {
        address: server_config_cli.oa.unwrap(),
        port: server_config_cli.op.unwrap(),
    };
    let remote_server_config = ServerConfig {
        address: server_config_cli.ra.unwrap(),
        port: server_config_cli.rp.unwrap(),
    };

    info!("### Fontend ready ###");
    info!(
        "Using local address: {:#?}, remote address: {:#?}",
        own_server_config.get_url(),
        remote_server_config.get_url()
    );

    let own_address = own_server_config.address.clone();
    let own_port = own_server_config.port.parse::<u16>().unwrap();
    let remote_address = remote_server_config.address.clone();
    let remote_port = remote_server_config.port.parse::<u16>().unwrap();

    let backend_urls = BackendUrls {
        clear_requirements: format!("http://{remote_address}:{remote_port}/clear_requirements"),
        update_requirements: format!("http://{remote_address}:{remote_port}/update_requirements"),
        clear_results: format!("http://{remote_address}:{remote_port}/clear_reults"),
        search_companies: format!("http://{remote_address}:{remote_port}/search_companies"),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(backend_urls.clone()))
            .service(new_search)
            .service(set_requirements)
            .service(get_companies)
    })
    .bind((own_address, own_port))?
    .run()
    .await?;

    Ok(())
}

#[get("/index.html")]
async fn new_search() -> impl Responder {
    info!("New search at index.html");
    let bytes = include_bytes!("../html/requirements.html");
    let reqiurements = str::from_utf8(bytes).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(reqiurements)
}

#[post("/set_requirements")]
async fn set_requirements(
    backend_url: web::Data<BackendUrls>,
    requirements: String,
) -> impl Responder {
    let url = backend_url.update_requirements.clone();
    info!("Received requirements: {url} \n {:#?}", requirements);

    let serde_qs_config = Config::new(5, false);

    let in_requirements: StockRequirements =
        serde_qs_config.deserialize_str(&requirements).unwrap();
    info!(
        "Request: Order setting requirements: {url} \n {:#?}",
        in_requirements
    );

    let client = reqwest::Client::new();
    let response = client.post(&url).json(&in_requirements).send().await;
    info!("Response: {:#?}", response.unwrap().text().await);

    let requirements_ready_body = "
    <!DOCTYPE html>
    <html>
        <body>
            <h2>Requirements updated</h2>
            <form action=\"/get_companies\">
                <input type=\"submit\" value=\"Run search\" />
            </form>
        </body>
    </html>
    ";

    HttpResponse::Ok()
        .content_type("text/html")
        .body(requirements_ready_body)
}

#[get("/get_companies")]
async fn get_companies(backend_url: web::Data<BackendUrls>) -> impl Responder {
    let url = backend_url.search_companies.clone();
    info!("Request: Order seraching companies: {url}");
    let response = reqwest::get(&url).await.unwrap().text().await;

    let companies = match response {
        Ok(content) => content,
        Err(_) => "".to_string(),
    };

    if companies.is_empty() {
        panic!("Something went wrong!")
    }

    let mut result = String::new();

    result += "
    <!DOCTYPE html>
    <html>
    <style>
    table, th, td {
      border:1px solid black;
      border-collapse: collapse;
    }
    </style>
    <body>
    <h2>Best Warsaw Stock Exchange listed companies</h2>
    ";
    result += "<table style='width:100%'>";

    let companies_json: Vec<Company> = serde_json::from_str(&companies).unwrap();

    let headers = Company::FIELD_NAMES_AS_ARRAY;

    result += "<tr>";
    for header in headers.clone() {
        if *header != "base_link" {
            result += "<td>";
            result += *header;
            result += "</td>";
        }
    }
    result += "</tr>";

    for company in companies_json {
        result += "<tr>";
        for header in headers.clone() {
            if *header != "base_link" {
                result += "<td>";
            }

            match *header {
                "link" => {
                    result += &format!("<a href=\"{}\">{}</a>", company.link, company.link);
                }
                "name" => {
                    result += &company.name;
                }
                "ticker" => {
                    result += &company.ticker;
                }
                "altman" => {
                    result += &company.altman;
                }
                "f_score" => {
                    result += &company.f_score.to_string();
                }
                "pe" => {
                    result += &company.pe.to_string();
                }
                "roe" => {
                    result += &company.roe.to_string();
                }
                "p_bv" => {
                    result += &company.p_bv.to_string();
                }
                "p_bvg" => {
                    result += &company.p_bvg.to_string();
                }
                _ => {}
            }

            result += "</td>";
        }
        result += "</tr>";
    }

    result += "</table>";

    result += "
        <br>
        <form action=\"/index.html\">
            <input type=\"submit\" value=\"New search\" />
        </form>
    ";

    result += "</body>";

    info!("Publish companies");
    HttpResponse::Ok().content_type("text/html").body(result)
}
