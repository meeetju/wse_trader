use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        App::new().service(hello).service(echo)
        // .route("/search", web::get().to(search_comanies))
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

// async fn search_comanies() -> impl Responder {
//     let content = backend().await;
//     HttpResponse::Ok().body(content)
// }
