
<p align="center">
  <img alt="Meta Secret" src="https://github.com/meta-secret/meta-secret-node/blob/main/docs/img/meta-secret-logo.jpg" width="250" />
</p>


## Command Line App

#### Splitting secrets:
You can split and restore your secrets by using meta-secret cli app in docker
<p>
Split:
</p>

```bash
$ mkdir secerts
$ docker run -ti --rm -v $(pwd)/secrets:/app/secrets ghcr.io/meta-secret/meta-secret-cli:latest split --secret top$ecret 
```
It will generate json/qr(jpg) files (shares of your secert) in the `secrets` directory.

## Building

## **1. Install rustc, cargo and rustfmt.**
