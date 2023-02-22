# IDS721 Spring 2023 Project 2 - Rust Containerized Microservice

The [HuggingFace Hub](https://github.com/huggingface/huggingface_hub) is implemented as a Python wrapper around the HuggingFace API Endpoints. This project is a dockerized Rust microservice that acts as an API proxy for the HuggingFace API Endpoints. 

## Setup
1. Install

    ```
    $ cd hf-micro
    $ make install
    ```

2. Generate a [HuggingFace personal access token](https://huggingface.co/docs/hub/security-tokens) with write permissions
3. Configure environment variables below per choice of launching locally or within Docker

**Run locally**
1. Rename `SAMPLE_ENV` to `.env` and save
2. Set your HF personal access token in `.env`
3. Run local microservice on localhost:8080

    ```
    $ make run
    ```

**Run within Docker**
1. Set your HF personal access token in [Makefile](./hf-micro/Makefile)
2. Build Docker image

    ```
    $ make build
    ```

3. Run Docker image

    ```
    $ make rundocker
    ```

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

## ToDos

- [x] Configure GET, POST, DELETE, PUSH routes
- [x] Pass environment variables into Docker per [docs](https://docs.docker.com/compose/environment-variables/set-environment-variables/#set-environment-variables-with-docker-compose-run---env)
- [ ] Fix Docker CA certificate bug
- [ ] Unit testing
- [ ] Python benchmarking
- [ ] CI/CD & Binary Release
- [ ] AWS Deployment?


## References

* [HuggingFace Hub Python Client](https://github.com/huggingface/huggingface_hub)
* [HF access tokens](https://huggingface.co/docs/hub/security-tokens)
* [HF Hub REST API Endpoints](https://huggingface.co/docs/hub/api)
* [Actix extractors](https://actix.rs/docs/extractors/)
* [reqwest crate docs](https://crates.io/crates/reqwest)