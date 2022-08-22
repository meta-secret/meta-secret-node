FROM rust:1.62

COPY target/release/meta-secret-vault-server /vault-server/
COPY Rocket.toml /vault-server

WORKDIR /vault-server

EXPOSE 8000

CMD ./meta-secret-vault-server
