#!/usr/bin/env bash

set -e

# Building rust app using docker multi-build is super slow.
# We need an optimization that pre builds all the dependencies in a target dir and then
# the cli docker image can use meta-secret-cli app.
# Use cases: the target dir can be cached by github actions or locally
# The problem explained:
# - https://dev.to/rogertorres/first-steps-with-docker-rust-30oi
# - github issue https://github.com/rust-lang/cargo/issues/2644
docker build -t meta-secret-builder --file Dockerfile-builder .

echo execute cargo build
docker run -ti --rm -v $(pwd)/target:/src/target meta-secret-builder

echo pwd: $(pwd)

echo list curr directory
ls -la

docker build -t ghcr.io/meta-secret/meta-secret-cli:latest --file Dockerfile-cli .
