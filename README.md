
<p align="center">
  <img alt="Meta Secret" src="https://github.com/meta-secret/meta-secret-node/blob/main/docs/img/meta-secret-logo.jpg" width="250" />
  
  <h4 align="center"> <a href="https://apps.apple.com/app/metasecret/id1644286751">Meta Secret Mobile Application</a></h4>
  
  <h5 align="center"> <a href="https://meta-secret.org">Web site</a> </h5>
  <h5 align="center"> <a href="https://meta-secret.github.io">Shamir Secret Sharing web app</a> </h5>
</p>

## Application Design

#### Activity Diagram
```mermaid
graph TD
    User --> |split password| MSS{MetaSecret}
    MSS --> |split| Hash1
    MSS --> |split| Hash2
    MSS --> |split| Hash3
    
    User --> |recover password| MSR{MetaSecret}
    MSR --> |read| HH1[Hash1]
    MSR --> |read| HH2[Hash2]
    HH1 --> RecoverAlgo[Meta Secret: Recovery Algorithm]
    HH2 --> RecoverAlgo
    RecoverAlgo --> RP[Recovered Password]
```

#### Sequence Diagram
```mermaid
sequenceDiagram
    note over User: Split to 3 shares
    User->>+MetaSecret: Split password
    
    MetaSecret->>User: show qr1 (of hash1)
    MetaSecret->>User: show qr2 (of hash2)
    MetaSecret->>-User: show qr3 (of hash3)
    User ->> World: save qr codes in different places

    note over User: Recover from 2 shares
    User ->> World: get qr1
    User ->> World: get qr3

    User ->> MetaSecret: recover password
    User -->> MetaSecret: provide qr1
    User -->> MetaSecret: provide qr3
    MetaSecret ->> MetaSecret: recover password
    MetaSecret ->> User: show password
```

## Web Application
Meta Secret Web Cli is available on https://meta-secret.github.io

## Command Line App

#### Split secrets:
You can split and restore your secrets by using meta-secret cli app in docker.
<br>
Imagine that we want to split `top$ecret`, then the command will be: 

```bash
$ mkdir secrets
$ docker run -ti --rm -v "$(pwd)/secrets:/app/secrets" ghcr.io/meta-secret/cli:latest split --secret top$ecret 
```

It will generate json/qr(jpg) files (shares of your secert) in the `secrets` directory.

#### Restore secrets:
When it comes to restore the secret, put json or qr files (shares of your secret) into the `secrets` directory.
Then run in case of qr (if you want restore from json, just pass --from json ):

```bash
$ docker run -ti --rm -v "$(pwd)/secrets:/app/secrets" ghcr.io/meta-secret/cli:latest restore --from qr 
```

## Advice for VPS-users
If you don't want to use FileZilla to download QR-codes to see on your computer, you can see them in terminal.

#### Installation
```bash
$ brew install qrencode (MacOS)
$ apt-get install qrencode (Debian/Ubuntu)
$ dnf install qrencode (CentOS/Rocky/Alma)
```

#### Showing QR codes in terminal
```bash
$ qrencode -t ansiutf8 < meta-secret-1.json
```

Congrats! Save these codes in secure place!

Below is optional
If you would like to extract data from QR's
  * Just take a phone to scan QR
  * or screenshot the terminal and upload it on this website: [webqr.com](https://webqr.com)

<br>
