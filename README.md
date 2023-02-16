# IDS721 Spring 2023 Project 2 - Rust Containerized Microservice

The [HuggingFace Hub](https://github.com/huggingface/huggingface_hub) is implemented as a Python wrapper around the HuggingFace API Endpoints. This project is a dockerized Rust microservice that acts as an API proxy for the HuggingFace API Endpoints. 

## Setup
1. Rename `SAMPLE_ENV` to `.env` and save
2. Generate a [HuggingFace personal access token](https://huggingface.co/docs/hub/security-tokens) with write permissions
3. Set your HF personal access token in `.env`

## Useage

Supported endpoints:

**GET /** -- Homepage

**GET /api/account** -- Returns your HuggingFace account info

**POST /api/repo** -- Create a new repository

    ```
    POST /api/repo json request body 
    {
        "type": <"model","dataset" or "spaces">
        "name": <repo_name>,
        "private": <true or false>
    }
    ```

**DELETE /api/repo** -- Delete a repository

    ```
    DELETE /api/repo json request body 
    {
        "type": <"model","dataset" or "spaces">
        "name": <repo_name>
    }
    ```

**PUT /api/repo** -- Update repository visibility

    ```
    PUT /api/repo json request body 
    {
        "type": <"model","dataset" or "spaces">
        "namespace": "<username>/<repo_name>" eg. "ferris/my_repo"
        "private": <true or false>
    }
    ```

**Todos**

* Docker container and deploy to AWS
* Python benchmarking

## Deploy to AWS
1. Create Cloud9 env -- install from rustup, run `source "$HOME/.cargo/env"`


## References

* [HuggingFace Hub Python Client](https://github.com/huggingface/huggingface_hub)
* [HF access tokens](https://huggingface.co/docs/hub/security-tokens)
* [HF Hub REST API Endpoints](https://huggingface.co/docs/hub/api)
* [Actix extractors](https://actix.rs/docs/extractors/)
* [reqwest crate docs](https://crates.io/crates/reqwest)