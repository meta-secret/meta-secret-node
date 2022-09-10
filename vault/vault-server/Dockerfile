FROM rust:1.62

COPY meta-secret-vault-server /vault-server/
COPY Rocket.toml /vault-server

WORKDIR /vault-server

EXPOSE 80

CMD ./meta-secret-vault-server
