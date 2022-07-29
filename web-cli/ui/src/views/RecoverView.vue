<script lang="js">

import init, {split} from "meta-secret-web-cli";
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

        Promise.all(asyncShares).then(shares => {
          //use wasm to recover from json files
          shares.forEach(share => {
            alert("Password share: " + share.data);
          });
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
  <h1 align="center">Recover Password</h1>
  <div class="container">
    <div style="display: flex; flex-direction: column; align-items: stretch">
      <label for="file-upload" class="custom-file-upload">
        Choose QR codes
      </label>
      <input id="file-upload" class="submit-button" type='file' accept='image/*' @change="openFile" multiple>
      <div style="margin-top: 15px"></div>
      <input class="submit-button" type="button" id="recoverButton" value="Recover" @click="recoverPassword">
    </div>
  </div>

  <div id="qrImages" style="display: flex; flex-direction: column; align-items: flex-start;"></div>
</template>