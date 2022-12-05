mod company;
mod errors;
mod lazy_regexps;
mod ranked_companies;
mod requirements_reader;
mod results_writer;
mod urls_modifier;

use std::env;

use crate::ranked_companies::RankedCompanies;
use crate::requirements_reader::YamlReader;
use crate::results_writer::{ConsolePrinter, CsvWriter, JsonWriter};
use crate::urls_modifier::UrlsModifier;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut ranked: RankedCompanies;

    HttpServer::new(|| {
        App::new()
        .service(new_search)
        .service(update_requirements)
        .service(best_companies)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/new")]
async fn new_search() -> impl Responder {
    // ranked = RankedCompanies::new();
    HttpResponse::Ok().body("New search!")
}

#[post("/requirements")]
async fn update_requirements() -> impl Responder {
    // ranked.update_requirements(YamlReader {path: "requirements.yaml".to_string()});
    HttpResponse::Ok().body("Requirements updated!")
}

#[post("/companies")]
async fn best_companies() -> impl Responder {

    // ranked.update_url_mappings(UrlsModifier::new("links_mapping.yaml".to_string()));
    // ranked.get_companies().await;
    // ranked.update_indicators().await;
    // ranked.filter_best_companies().await;
    // println!("{}", ranked.write_results(JsonWriter {}).await);

    HttpResponse::Ok().body("Best companies received!")
}

    // let mut ranked = RankedCompanies::new();
    // ranked.update_requirements(YamlReader {path: "requirements.yaml".to_string()});
    // ranked.update_url_mappings(UrlsModifier::new("links_mapping.yaml".to_string()));
    // ranked.get_companies().await;
    // ranked.update_indicators().await;
    // ranked.filter_best_companies().await;
    // // ranked.write_results(CsvWriter{path: "results.csv".to_string()}).await;
    // // ranked.write_results(ConsolePrinter{}).await;
    // println!("{}", ranked.write_results(JsonWriter {}).await);