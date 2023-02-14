/*
An Actix microservice that uses reqwest to interface with the HuggingFace Hub REST API.

 */

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

// Nested struct helper
#[derive(Serialize, Deserialize, Debug)]
struct Nest<T> {
    #[serde(flatten)]
    inner: T
}

// Define AccessToken struct
#[derive(Serialize, Deserialize, Debug)]
struct AccessToken {
    #[serde(rename = "displayName")]
    display_name: String,
    role: String,
}
// Define Token struct
#[derive(Serialize, Deserialize, Debug)]
struct Auth {
    #[serde(rename = "type")]
    auth_type: String,
    #[serde(rename = "accessToken")]
    access_token: Nest<AccessToken>,
}
// Define acct response struct
#[derive(Serialize, Deserialize, Debug)]
struct AcctResponse {
    #[serde(rename = "type")]
    acct_type: String,
    id: String,
    name: String,
    fullname: String,
    email: String,
    #[serde(rename = "emailVerified")]
    email_verified: bool,
    plan: String,
    #[serde(rename = "canPay")]
    can_pay: bool,
    #[serde(rename = "isPro")]
    is_pro: bool,
    #[serde(rename = "periodEnd")]
    period_end: Option<String>,
    #[serde(rename = "avatarUrl")]
    avatar_url: String,
    orgs: Vec<String>,
    auth: Nest<Auth>,
}

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
            // define under '/api/' route
            .service(web::scope("/api").service(account))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
