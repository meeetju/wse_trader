mod company;
mod errors;
mod lazy_regexps;
mod ranked_companies;
mod requirements_reader;
mod results_writer;
mod urls_modifier;

use std::env;
use std::sync::Arc;

use crate::ranked_companies::RankedCompanies;
use crate::requirements_reader::YamlReader;
use crate::results_writer::{ConsolePrinter, CsvWriter, JsonWriter};
use crate::urls_modifier::UrlsModifier;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use futures::lock::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let ranked = Arc::new(Mutex::new(RankedCompanies::new()));

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(ranked.clone()))
        .service(new_search)
        .service(update_requirements)
        .service(best_companies)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/new")]
async fn new_search(ranked: web::Data<Arc<Mutex<RankedCompanies>>>) -> impl Responder {
    ranked.lock().await.clear();
    HttpResponse::Ok().body("New search!")
}

#[get("/requirements")]
async fn update_requirements(ranked: web::Data<Arc<Mutex<RankedCompanies>>>) -> impl Responder {
    ranked.lock().await.update_requirements(YamlReader {path: "requirements.yaml".to_string()});
    HttpResponse::Ok().body("Requirements updated!")
}

#[get("/companies")]
async fn best_companies(ranked: web::Data<Arc<Mutex<RankedCompanies>>>) -> impl Responder {

    ranked.lock().await.update_url_mappings(UrlsModifier::new("links_mapping.yaml".to_string()));
    ranked.lock().await.get_companies().await;
    ranked.lock().await.update_indicators().await;
    ranked.lock().await.filter_best_companies().await;
    // ranked.lock().await.write_results(ConsolePrinter{}).await;
    let companies = ranked.lock().await.write_results(JsonWriter {}).await;
    println!("{}", &companies);

    HttpResponse::Ok().body(companies)
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