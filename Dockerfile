FROM rust:latest as builder
ARG token
ENV HF_ACCESS_TOKEN $token
ENV APP hf-micro
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && \
    # install ca-certificates
    apt-get install -y ca-certificates && \
    # update ca-certificates
    update-ca-certificates && \
    # clean up
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
#export this actix web service to port 8080 and 0.0.0.0
EXPOSE 8080
CMD ["hf-micro"]