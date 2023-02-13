/*
An Actix microservice that uses reqwest to interface with the HuggingFace Hub REST API.

 */

use actix_web::{get, web, App, middleware, HttpResponse, HttpServer, Responder};

// GET HuggingFace Hub account info from /api/whoami-v2 GET
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
        // .header("content_type", "application/json")
        // .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await;
    // print response
    println!("{res:?}");
    HttpResponse::Ok().body("Hello")

}

// Build actix web client to make requests to the HuggingFace Hub REST API
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Check for .env file and load environment variables
    dotenv::dotenv().ok();
    // Start http server
    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // everything under '/api/' route
            .service(
                web::scope("/api")
                    .service(account)
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}



