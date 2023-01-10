use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result, HttpRequest};
use actix_files::NamedFile;
use env_logger;
use core::panic;
use std::{env, process::CommandArgs};
use std::collections::HashMap;

#[derive(serde_derive::Deserialize)]
struct FormData {
    pub altman: String,
    // pub piotroski: String,
    // pub pe: String,
    // pub roe: String,
    // pub p_bv: String,
    // pub p_bvg: String
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
        .service(new_search)
        .service(set_requirements)
        .service(update_requirements)
        .service(get_companies)
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

// #[get("/set_requirements")]
// async fn set_requirements() -> impl Responder {
//     let response = reqwest::get("http://127.0.0.1:8080/requirements").await.unwrap().text().await;
//     println!("{:?}", response);
//     HttpResponse::Ok().body("Requirements set OK!")
// }

#[post("/set_requirements")]
async fn set_requirements(form: web::Form<FormData>) -> impl Responder {
    println!("Value to show: {}", form.altman);
    // println!("Value to show: {}", form.piotroski);
    let response = reqwest::get("http://127.0.0.1:8080/requirements").await.unwrap().text().await;
    println!("{:?}", response);
    HttpResponse::Ok().body("Requirements set OK!")
}

#[get("/update_requirements")]
async fn update_requirements(req: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("html/requirements.html")?)
}

#[get("/get_companies")]
async fn get_companies() -> impl Responder {
    let response = reqwest::get("http://127.0.0.1:8080/companies").await.unwrap().text().await;
    let mut result = String::new();

    let companies = match response {
        Ok(content) => content,
        Err(_) => "".to_string()
    };

    if companies.is_empty() {
        panic!("Something went wrong!")
    }

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
    

    let companies_json: Vec<HashMap<String, String>> = serde_json::from_str(&companies).unwrap();

    let headers: Vec<String> = vec![
        "name".to_string(), 
        "ticker".to_string(),
        "link".to_string(), 
        "altman".to_string(), 
        "piotroski".to_string(), 
        "pe".to_string(), 
        "roe".to_string(), 
        "p_bv".to_string(), 
        "p_bvg".to_string()
        ];

    result += "<tr>";
    for header in headers.clone() {
        result += "<td>";
        result += &header;
        result += "</td>";
    }
    result += "</tr>";

    for company in companies_json {
        result += "<tr>";
        for header in headers.clone() {
            result += "<td>";

            match header.as_str() {
                "link" => {
                    let link = company.get(&header).unwrap();
                    result += &format!("<a href=\"{link}\">{link}</a>");
                },
                _ => {result += company.get(&header).unwrap();}
            }

            result += "</td>";
        }
        result += "</tr>";
    }

    result += "</table>";

    HttpResponse::Ok().content_type("text/html").body(result)
}
