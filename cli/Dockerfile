FROM alpine:3.15

COPY target/x86_64-unknown-linux-gnu/release/meta-secret-cli /app/meta-secret-cli
COPY cli/config.yaml /app/config.yaml

WORKDIR /app
ENTRYPOINT ["./meta-secret-cli"]
