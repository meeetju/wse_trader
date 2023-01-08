use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
        .service(new_search)
        .service(set_requirements)
        .service(get_companies)
        // .route("/search", web::get().to(search_comanies))
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}

#[get("/")]
async fn new_search() -> impl Responder {
    let response = reqwest::get("http://127.0.0.1:8080/new").await.unwrap().text().await;
    println!("{:?}", response);
    HttpResponse::Ok().body("New Search OK!")
}

#[get("/set_requirements")]
async fn set_requirements() -> impl Responder {
    let response = reqwest::get("http://127.0.0.1:8080/requirements").await.unwrap().text().await;
    println!("{:?}", response);
    HttpResponse::Ok().body("Requirements set OK!")
}

#[get("/get_companies")]
async fn get_companies() -> impl Responder {
    let response = reqwest::get("http://127.0.0.1:8080/companies").await.unwrap().text().await;
    println!("{:?}", response);
    HttpResponse::Ok().body("Best companies received OK!")
}
