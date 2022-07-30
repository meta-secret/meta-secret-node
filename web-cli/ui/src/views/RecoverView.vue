<script lang="js">

import init, {restore_password} from "meta-secret-web-cli";
import QrScanner from 'qr-scanner';

export default {
  methods: {
    recoverPassword() {
      init().then(() => {
        let imagesElement = document.getElementById("qrImages");
        let qrCodes = imagesElement.getElementsByTagName('img');

        let asyncShares = [];

        Array.from(qrCodes).forEach(qr => {
          asyncShares.push(QrScanner.scanImage(qr, {returnDetailedScanResult: true}));
        });

        Promise.all(asyncShares)
            .then(qrShares => {
              let passwordBox = document.getElementById("securityBox");

              //use wasm to recover from json files
              let shares = qrShares.map(share => JSON.parse(share.data));
              console.log("restore password, js!");
              let password = JSON.stringify(restore_password(shares));

              let passwordEl = document.createElement("div");
              passwordEl.innerHTML = password;
              passwordBox.appendChild(passwordEl);
            })
            .catch(err => {
              alert("Error recovering password: " + err)
            });
      });
    },

    openFile(event) {
      let input = event.target;

      Array.from(input.files).forEach(qr => {
        let reader = new FileReader();

        reader.onload = function () {
          let dataURL = reader.result;
          let outputImg = document.createElement('img');
          outputImg.style.margin = "0 0 0 0";
          outputImg.src = dataURL;

          let imagesElement = document.getElementById("qrImages");
          imagesElement.appendChild(outputImg);
        };

        reader.readAsDataURL(qr);
      });
    }
  }
}
</script>

<template>
  <div style="display:flex; justify-content: center">
    <h1>Recover Password</h1>
  </div>

  <div style="display:flex; margin: 10px"></div>

  <div class="container">
    <div style="display: flex; flex-direction: column; align-items: stretch">
      <label for="file-upload" class="custom-file-upload">
        Choose QR codes
      </label>
      <input id="file-upload" class="submit-button" type='file' accept='image/*' @change="openFile" multiple>
      <div style="margin-top: 15px"></div>
      <input class="submit-button" type="button" id="recoverButton" value="Recover" @click="recoverPassword">
    </div>

    <div id="securityBox" class="container security-box">
      <h3>Password:</h3>
      <p id="passwordBox"></p>
    </div>
  </div>

  <div id="qrImages" style="display: flex; flex-direction: column; align-items: flex-start;"></div>
</template>

<style>
.security-box {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  min-width: 600px;
  margin-top: 25px;
  margin-left: 50px;
  padding: 10px;
  border: 1px solid rgb(209, 213, 219);
}
</style>