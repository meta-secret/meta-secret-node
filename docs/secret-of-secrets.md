### MetaSecret Design Doc

The why:
 - ДЕЦЕНТРАЛИЗОВАННОЕ! хранилище секретов (паролей)

The how:
 - SSS - Shamir Secret Sharing: 
    - schema 3:2 
    - parts: [1,2,3] -> [1], [2], [3]. We need any 2
    - how it works: https://meta-secret.github.io/split
 - problema: 
    - как не забыть свой логин/пароль от самого мета секрета?
        - catch 22: https://en.wikipedia.org/wiki/Catch-22_(logic)
        - Secret Zero Problem https://github.com/meta-secret/meta-secret-node/blob/main/docs/MetaSecretPresentation.pdf
    - how meta secret solves the problem: https://github.com/meta-secret/meta-secret-node/blob/main/README.md

 - solution:
    - SSS - разорвать пароль
    - catch 22? 
       - Мы используем подход на основе finger/face/pin - встроенная и биометрическая аутентификация на самих устройствах (то есть все виды). 
       - Поверх встроенных безопасностей в каждом девайсе, мы строим децентрализованную аутентификацию для доступа к секретам. Мы это называем passwordless. 
       - AAA: auth, access. 
       - как это все работает, включая децентрализованную аутентификацию https://youtu.be/JTFiaN6etFY
    - децентрализованная аутентификация:
       - vault: [device1[pk], device2[pk], device3[pk]]
       - deviceX: generate sk/pk
       - add device to a vault: SEND your pk
    - crypto!!! 
       encryption:
       - end to end encryption! AEAD: https://en.wikipedia.org/wiki/Authenticated_encryption
       - device1: 
          - split(secret, 3) -> [share1, share2, share3]
          - enc_share_2 = encrypt(secret, device2.pk)
          - replicate: send(enc_share_2, device2)
          - repeat for device 3
       decrypt:
          - так же как сплит только наоборот

### Философко-статьевое
 - [статья объясняющая механику и проблему](https://vc.ru/tribuna/622604-metasecret-decentralizovannoe-hranilishe-sekretov-parolei-s-shifrovaniem)
 - [та же статья но только более суровая и упоротая, мемовая, ат души](https://vc.ru/flood/621986-kak-nadezhno-ne-zabyt-nadezhnyi-parol)
 - [habrovich: философка про сложность паролей](https://habr.com/ru/articles/720606/)
   
