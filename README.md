# IDS721 Spring 2023 Project 2 - Rust Containerized Microservice

A dockerized Rust microservice that acts as the user interfacing layer to the HuggingFace REST API.

## Setup
1. Rename `SAMPLE_ENV` to `.env` and save
2. Generate a [HuggingFace personal access token](https://huggingface.co/docs/hub/security-tokens) with write permissions
3. Set your HF personal access token in `.env`

## Useage

Supported endpoints:

* GET / -- landing page
* GET /api/accounts -- returns HF account info

**Todos**

* POST /newrepo



## References
* [HF access tokens](https://huggingface.co/docs/hub/security-tokens)
* [reqwest crate docs](https://crates.io/crates/reqwest)
* [HF Hub REST API Endpoints](https://huggingface.co/docs/hub/api)
* [Parsing reqwest to JSON](https://www.youtube.com/watch?v=ogpE4hviXyA&t=111s)