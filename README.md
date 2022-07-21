
<p align="center">
  <img alt="Meta Secret" src="https://github.com/meta-secret/meta-secret-node/blob/main/docs/img/meta-secret-logo.jpg" width="250" />
</p>


## Command Line App

#### Split secrets:
You can split and restore your secrets by using meta-secret cli app in docker.
<br>
Imagine that we want to split `top$ecret`, then the command will be: 

```bash
$ mkdir secrets
$ docker run -ti --rm -v $(pwd)/secrets:/app/secrets ghcr.io/meta-secret/meta-secret-cli:latest split --secret top$ecret 
```

It will generate json/qr(jpg) files (shares of your secert) in the `secrets` directory.

#### Restore secrets:
When it comes to restore the secret, put json or qr files (shares of your secret) into the `secrets` directory.
Then run in case of qr (if you want restore from json, just pass --from json ):

```bash
$ docker run -ti --rm -v $(pwd)/secrets:/app/secrets ghcr.io/meta-secret/meta-secret-cli:latest restore --from qr 
```
