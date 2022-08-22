#!/bin/bash

set -e

# Must be run from root directory
cargo build -p meta-secret-vault-server --release

VAULT_SERVER_DIR="vault/vault-server"
VAULT_SERVER_TARGET_DIR="${VAULT_SERVER_DIR}/target"

rm -rf "${VAULT_SERVER_TARGET_DIR}/**"

mkdir -p ${VAULT_SERVER_TARGET_DIR}
cp -rf target/release/meta-secret-vault-server ${VAULT_SERVER_TARGET_DIR}
chmod +x ${VAULT_SERVER_TARGET_DIR}/meta-secret-vault-server

cd ${VAULT_SERVER_DIR}
docker build -t "ghcr.io/meta-secret/vault-server:latest" .

#echo "run meta-secret-server"
#docker run -ti --rm -p 8000:8000 --name meta-secret-vault-server ghcr.io/meta-secret/vault-server:latest
