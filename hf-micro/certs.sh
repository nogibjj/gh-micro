# A bash script to generate a self-signed certificate to run on localhost and mount within the Docker image

# make local certs dir
mkdir -p ./certs
# generate ca-certificates
openssl req -x509 -newkey rsa:4096 -keyout ./certs/key.pem -out ./certs/cert.pem -sha256 -days 365 -nodes -subj '/CN=localhost'







