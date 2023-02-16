mod structs;
use crate::structs::{AcctResponse, DeleteRepo, NewRepo, RepoResponse, UpdateRepo};
use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

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
    // Match res status
    match res.status() {
        reqwest::StatusCode::OK => {
            // Deserialize response
            let res_json = res.json::<AcctResponse>().await.unwrap();
            // Print response
            println!("{res_json:?}");
            let res_body = serde_json::to_string(&res_json).unwrap();
            // Return response
            HttpResponse::Ok().body(res_body)
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            HttpResponse::Ok().body("ERROR: Access token unauthorized")
        }
        _ => HttpResponse::Ok().body("ERROR: Failed to get account info"),
    }
}

// POST new repo to HuggingFace Hub https://huggingface.co/api/repos/create
pub async fn new_repo(repo_config: web::Json<NewRepo>) -> impl Responder {
    println!("POST /api/repo request received");
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
    // Match res status
    match res.status() {
        reqwest::StatusCode::OK => {
            // Deserialize response
            let res_json = res.json::<RepoResponse>().await.unwrap();
            let res_body = format!("New repo created: {}", res_json.url);
            // Return response
            HttpResponse::Ok().body(res_body)
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            HttpResponse::Ok().body("ERROR: Access token unauthorized")
        }
        _ => HttpResponse::Ok().body("ERROR: Failed to delete repo"),
    }
}

// DELETE repo from HuggingFace Hub https://huggingface.co/api/repos/delete
pub async fn delete_repo(repo_config: web::Json<DeleteRepo>) -> impl Responder {
    println!("DELETE /api/repo request received");
    // Create a new reqwest client
    let client = reqwest::Client::new();
    // Create authorization string
    let auth_token = dotenv::var("HF_ACCESS_TOKEN").expect("AUTH_TOKEN must be set");
    let auth_str = format!("Bearer {auth_token}");
    // POST request with authorization header and json body
    let res = client
        .delete("https://huggingface.co/api/repos/delete")
        .header("authorization", auth_str)
        .json(&repo_config)
        .send()
        .await
        .unwrap();
    // Match res status
    match res.status() {
        reqwest::StatusCode::OK => HttpResponse::Ok().body("SUCCESS: Deleted repo"),
        reqwest::StatusCode::NOT_FOUND => HttpResponse::Ok().body("ERROR: Repo DNE"),
        reqwest::StatusCode::UNAUTHORIZED => {
            HttpResponse::Ok().body("ERROR: Access token unauthorized")
        }
        _ => HttpResponse::Ok().body("ERROR: Failed to delete repo"),
    }
}

// PUT update repo visibility at https://huggingface.co/api/api/{type}/{namespace}/settings
pub async fn update_repo(repo_config: web::Json<UpdateRepo>) -> impl Responder {
    println!("PUT /api/repo request received");
    // Create a new reqwest client
    let client = reqwest::Client::new();
    // Create authorization string
    let auth_token = dotenv::var("HF_ACCESS_TOKEN").expect("AUTH_TOKEN must be set");
    let auth_str = format!("Bearer {auth_token}");
    // PUT request with authorization header and json body
    let url = format!(
        "https://huggingface.co/api/{}s/{}/settings",
        repo_config.repo_type, repo_config.namespace
    );
    let visibility = format!("{}", repo_config.private);
    let body = json!({ "private": visibility });
    let res = client
        .put(url)
        .header("authorization", auth_str)
        .json(&body)
        .send()
        .await
        .unwrap();
    // Match res status
    match res.status() {
        reqwest::StatusCode::OK => HttpResponse::Ok().body("SUCCESS: Repo visibility updated"),
        reqwest::StatusCode::UNAUTHORIZED => {
            HttpResponse::Ok().body("ERROR: Access token unauthorized")
        }
        _ => HttpResponse::Ok().body("ERROR: Failed to update repo visibility"),
    }
}
