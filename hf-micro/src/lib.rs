mod structs;
use crate::structs::{AcctResponse, NewRepo, RepoResponse};
use actix_web::{get, web, HttpResponse, Responder};

// Home route
#[get("/")]
async fn home() -> impl Responder {
    println!("GET / request received");
    HttpResponse::Ok().body("Welcome to the Rust HuggingFace API Interface!")
}

// GET HuggingFace Hub account info from https://huggingface.co/api/whoami-v2
#[get("/account")]
async fn account() -> impl Responder {
    println!("GET /api/account request received");
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

// POST new repo to HuggingFace Hub https://huggingface.co/api/repos/create
pub async fn newrepo(repo_config: web::Json<NewRepo>) -> impl Responder {
    println!("POST /api/newrepo request received");
    // Create a new reqwest client
    let client = reqwest::Client::new();
    // Create authorization string
    let auth_token = dotenv::var("HF_ACCESS_TOKEN").expect("AUTH_TOKEN must be set");
    let auth_str = format!("Bearer {auth_token}");
    // POST request with authorization header and json body
    let res = client
        .post("https://huggingface.co/api/repos/create")
        .header("authorization", auth_str)
        .json(&repo_config)
        .send()
        .await
        .unwrap();
    // Deserialize response
    let res_json = res.json::<RepoResponse>().await.unwrap();
    let res_body = serde_json::to_string(&res_json).unwrap();
    // Return response
    HttpResponse::Ok().body(res_body)
}