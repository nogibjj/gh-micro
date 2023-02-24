# Set your HF_ACCESS_TOKEN here
USR_TOKEN = <YOUR_ACCESS_TOKEN>

install:
	cargo clean &&\
		cargo build -j 1

build:
	docker-compose build

rundocker:
	docker run -e HF_ACCESS_TOKEN=${USR_TOKEN} -p 8080:8080 hf-micro

mntcerts:
	docker run -e HF_ACCESS_TOKEN=${USR_TOKEN} -p 8080:8080 -v /certs/cert.pem:/usr/local/share/ca-certificates/ hf-micro

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

all: format lint test run