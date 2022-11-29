mod company;
mod errors;
mod lazy_regexps;
mod requirements_reader;
mod ranked_companies;
mod results_writer;
mod urls_modifier;

use std::env;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::ranked_companies::RankedCompanies;
use crate::requirements_reader::YamlReader;
use crate::results_writer::{CsvWriter, ConsolePrinter};
use crate::urls_modifier::UrlsModifier;

#[actix_web::main]
// #[tokio::main]
async fn main() -> std::io::Result<()> {

    env::set_var("RUST_LOG","info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/search", web::get().to(search_comanies))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn search_comanies() -> impl Responder {
    backend().await;
    HttpResponse::Ok().body("Hey there!")
}

async fn backend() {

    let mut ranked = RankedCompanies::new();
    ranked.update_requirements(YamlReader{path: "requirements.yaml".to_string()});
    ranked.update_url_mappings(UrlsModifier::new("links_mapping.yaml".to_string()));
    ranked.get_companies().await;
    ranked.update_indicators().await;
    ranked.filter_best_companies().await;
    // ranked.write_results(CsvWriter{path: "results.csv".to_string()}).await;
    ranked.write_results(ConsolePrinter{}).await;

}