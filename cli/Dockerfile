# https://docs.docker.com/develop/develop-images/multistage-build/
FROM rust:1.65.0 as build
COPY ./ /build
WORKDIR /build
RUN cargo test
RUN cargo build --release

FROM ubuntu:23.04

COPY --from=build /build/target/release/meta-secret-cli /app/meta-secret-cli
COPY config.yaml /app/config.yaml

WORKDIR /app
ENTRYPOINT ["./meta-secret-cli"]
