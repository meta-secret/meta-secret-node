
rm -rf target
mkdir -p target/core
cp -R ../core target
docker build -t meta-secret/web-cli-builder:latest --file Dockerfile-builder .

