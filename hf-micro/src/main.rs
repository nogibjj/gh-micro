/*
An Actix microservice that uses reqwest to interface with the HuggingFace Hub REST API.
 */
// mod structs;
// use structs::{AcctResponse, NewRepo};
use serde::Deserialize;

use actix_web::{middleware, web, App, HttpResponse, Responder, HttpServer};

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
            .service(hf_micro::home)
            // define under '/api/' route
            .service(
                web::scope("/api")
                    .service(hf_micro::account)
                    .service(
                        web::resource("/newrepo")
                            .route(web::post().to(hf_micro::newrepo)),
                    )
                )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
