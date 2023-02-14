/*
An Actix microservice that uses reqwest to interface with the HuggingFace Hub REST API.
 */

mod structs;
use crate::structs::AcctResponse;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};

// Home route
#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Rust HuggingFace API Interface!")
}

// GET HuggingFace Hub account info from https://huggingface.co/api/whoami-v2
#[get("/account")]
async fn account() -> impl Responder {
    // Create a new reqwest client
    let client = reqwest::Client::new();
    // Create authorization string
    let auth_token = dotenv::var("HF_ACCESS_TOKEN").expect("AUTH_TOKEN must be set");
    let auth_str = format!("Bearer {auth_token}");
    // GET request with authorization header
    let res = client
        .get("https://huggingface.co/api/whoami-v2")
        .header("authorization", auth_str)
        .send()
        .await
        .unwrap();
    // Deserialize response
    let res_json = res.json::<AcctResponse>().await.unwrap();
    // Print response
    println!("{res_json:?}");
    let res_body = serde_json::to_string(&res_json).unwrap();
    // Return response
    HttpResponse::Ok().body(res_body)
}

// Actix web client to make requests to the HuggingFace Hub REST API
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Check for .env file and load environment variables
    dotenv::dotenv().ok();
    // Start http server
    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // homepage
            .service(home)
            // define under '/api/' route
            .service(web::scope("/api").service(account))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
