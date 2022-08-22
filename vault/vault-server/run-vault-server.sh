#!/bin/bash

docker pull ghcr.io/meta-secret/vault-server:latest

docker-compose down -v
docker-compose pull
docker-compose up
