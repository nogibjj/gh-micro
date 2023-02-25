/*
An Actix microservice that uses reqwest to interface with the HuggingFace Hub REST API.
 */
use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;

// Actix web client to make requests to the HuggingFace Hub REST API
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Check for .env file and load environment variables
    dotenv::dotenv().ok();
    // Initialize logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));
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
                    // GET /api/account
                    .service(hf_micro::account)
                    // POST /api/repo
                    .service(
                        web::resource("/repo")
                            // DELETE /api/repo
                            .route(web::post().to(hf_micro::new_repo))
                            .route(web::delete().to(hf_micro::delete_repo))
                            .route(web::put().to(hf_micro::update_repo)),
                    ),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
