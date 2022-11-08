#!/bin/bash

set -e

cargo build --release

DOCKER_BUILD_DIR="target/release/docker/"

mkdir -p "${DOCKER_BUILD_DIR}"

cp Dockerfile "${DOCKER_BUILD_DIR}"
cp app/Rocket.toml "${DOCKER_BUILD_DIR}"
cp app/mongodb-config.json "${DOCKER_BUILD_DIR}"
cp target/release/meta-secret-vault-server "${DOCKER_BUILD_DIR}"

cd ${DOCKER_BUILD_DIR}

docker build -t ghcr.io/meta-secret/vault-server:latest .
