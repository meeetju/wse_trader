use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use env_logger::Builder as LoggerBuilder;
use log::info;
use std::io::Write;
use std::{include_bytes, str};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut logger_builder = LoggerBuilder::new();
    logger_builder
        .parse_default_env()
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .filter(None, log::LevelFilter::Info)
        .init();

    let address = "127.0.0.1";
    let port = 8765;

    info!("### Run Third Party Mock Server ###");
    info!("address: {address}:{port}");
    info!("Mocked url: http://{address}:{port}/spolki-rating/akcje_gpw");
    info!("Mocked url: http://{address}:{port}/notowania/gpw/monnari-mon/wskazniki-finansowe");
    info!("Mocked url: http://{address}:{port}/notowania/gpw/xtb-xtb/wskazniki-finansowe");

    HttpServer::new(move || App::new().service(spolki).service(monnari).service(xtb))
        .bind((address, port))?
        .run()
        .await
}

#[get("/spolki-rating/akcje_gpw")]
async fn spolki() -> impl Responder {
    info!("Mocked response for /spolki-rating/akcje_gpw");
    let bytes = include_bytes!("../files/biznes_radar_response.txt");
    let webpage_body = str::from_utf8(bytes).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(webpage_body)
}

#[get("/notowania/gpw/monnari-mon/wskazniki-finansowe")]
async fn monnari() -> impl Responder {
    info!("Mocked response for /notowania/gpw/monnari-mon/wskazniki-finansowe");
    let bytes = include_bytes!("../files/MONNARI.txt");
    let webpage_body = str::from_utf8(bytes).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(webpage_body)
}

#[get("/notowania/gpw/xtb-xtb/wskazniki-finansowe")]
async fn xtb() -> impl Responder {
    info!("Mocked response for /notowania/gpw/xtb-xtb/wskazniki-finansowe");
    let bytes = include_bytes!("../files/XTB.txt");
    let webpage_body = str::from_utf8(bytes).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(webpage_body)
}
