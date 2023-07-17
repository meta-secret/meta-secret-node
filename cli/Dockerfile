# https://docs.docker.com/develop/develop-images/multistage-build/
FROM rust:1.71.0 as build
COPY ./ /build
WORKDIR /build
RUN cargo test
RUN cargo build --release

FROM ubuntu:22.10

COPY --from=build /build/target/release/meta-secret-cli /app/meta-secret-cli
COPY config.yaml /app/config.yaml

WORKDIR /app
ENTRYPOINT ["./meta-secret-cli"]
