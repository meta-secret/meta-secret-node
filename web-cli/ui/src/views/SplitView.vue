<script lang="js">
import QRCode from "qrcode";
import QRCodeStyling from "qr-code-styling";

import init, {split} from "meta-secret-web-cli";

export default {
  methods: {
    splitPassword() {
      init().then(() => {
        let password = document.getElementById("password").value;
        console.log("Split password: ", password);

        let qrImages = document.getElementById('qr-images');

        while (qrImages.firstChild) {
          qrImages.removeChild(qrImages.firstChild);
        }

        let shares = split(password);
        this.sharesProcessing(shares, qrImages);
      });
    },

    sharesProcessing: function (shares, qrImages) {
      shares.forEach(share => {
        let qr = document.createElement('canvas');
        qrImages.appendChild(qr);

        let shareIdText = 'share: ' + share['share_id'];
        let note1Text = document.getElementById('note1').value;
        let note2Text = document.getElementById('note2').value;

        let textImage = this.textToImage(note1Text, note2Text, shareIdText);
        let qrCodeStyling = this.generateQrCodeStyling(JSON.stringify(share), textImage);
        qrCodeStyling.append(qrImages);
        //generateQrCode(qr, share);
      });
    },

    generateQrCode(qrHtml, share) {
      QRCode.toCanvas(qrHtml, JSON.stringify(share), {errorCorrectionLevel: 'H'}, function (error) {
        if (error) {
          console.error(error)
        }
        console.log('success!');
      });
    },

    textToImage(line1, line2, line3) {
      let canvas = document.createElement("canvas");
      canvas.width = 600;
      canvas.height = 600;
      let ctx = canvas.getContext('2d');
      ctx.font = "100px Arial";
      ctx.fillText(line1, 50, 150);
      ctx.fillText(line2, 50, 300);
      ctx.fillText(line3, 50, 450);
      return canvas.toDataURL();
    },

    generateQrCodeStyling(share, textImage) {
      return new QRCodeStyling(
          {
            "width": 600,
            "height": 600,
            "data": share,
            "margin": 3,
            "qrOptions": {
              "typeNumber": "0",
              "mode": "Byte",
              "errorCorrectionLevel": "H"
            },
            "imageOptions": {
              "hideBackgroundDots": true,
              "imageSize": 0.2,
              "margin": 1
            },
            "dotsOptions": {
              "type": "dots",
              "color": "#000000",
              "gradient": null
            },
            "backgroundOptions": {
              "color": "#ffffff"
            },
            "image": textImage,
            "dotsOptionsHelper": {
              "colorType": {
                "single": true,
                "gradient": false
              },
              "gradient": {
                "linear": true,
                "radial": false,
                "color1": "#6a1a4c",
                "color2": "#6a1a4c",
                "rotation": "0"
              }
            },
            "cornersSquareOptions": {
              "type": "square",
              "color": "#000000",
              "gradient": {
                "type": "linear",
                "rotation": 0,
                "colorStops": [
                  {
                    "offset": 0,
                    "color": "#000000"
                  },
                  {
                    "offset": 1,
                    "color": "#8d8b8b"
                  }
                ]
              }
            },
            "cornersSquareOptionsHelper": {
              "colorType": {
                "single": true,
                "gradient": false
              },
              "gradient": {
                "linear": true,
                "radial": false,
                "color1": "#000000",
                "color2": "#000000",
                "rotation": "0"
              }
            },
            "cornersDotOptions": {
              "type": "",
              "color": "#000000"
            },
            "cornersDotOptionsHelper": {
              "colorType": {
                "single": true,
                "gradient": false
              },
              "gradient": {
                "linear": true,
                "radial": false,
                "color1": "#000000",
                "color2": "#000000",
                "rotation": "0"
              }
            },
            "backgroundOptionsHelper": {
              "colorType": {
                "single": true,
                "gradient": false
              },
              "gradient": {
                "linear": true,
                "radial": false,
                "color1": "#ffffff",
                "color2": "#ffffff",
                "rotation": "0"
              }
            }
          }
      );
    }
  }
}
</script>

<template>
  <h1 align="center">Split Password</h1>
  <div class="container">
    <div style="display: flex; flex-direction: column; align-items: flex-start">
      <div>
        <label for="note1">Note1:</label>
        <input class="input-element" type="text" id="note1" value="" max="10" size="10">
      </div>

      <div>
        <label for="note2">Note2:</label>
        <input class="input-element" type="text" id="note2" value="" max="10" size="10">
      </div>

      <label for="password">password:</label>
      <div style="display: flex; flex-direction: column; align-items: stretch">

        <input class="input-element" type="text" id="password" value="top$ecret" size="150">
        <input class="submit-button" type="button" id="splitButton" value="Split" @click="splitPassword">
      </div>
    </div>
  </div>

  <div id="shares"></div>

  <div id="qr-images"></div>
</template>

<style>
.input-element {
  width: 100%;
  padding: 12px;
  margin: 6px 0 4px;
  border: 1px solid #ccc;
  background: #fafafa;
  color: #000;
  font-family: sans-serif;
  font-size: 12px;
  line-height: normal;
  box-sizing: border-box;
  border-radius: 2px;
}

.submit-button {
  background-color: #FFFFFF;
  border: 1px solid rgb(209, 213, 219);
  border-radius: .3rem;
  box-sizing: border-box;
  color: #111827;
  font-family: "Inter var", ui-sans-serif, system-ui, -apple-system, system-ui, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";
  font-size: .875rem;
  font-weight: 600;
  line-height: 1.25rem;
  padding: .75rem 1rem;
  text-align: center;
  text-decoration: none #D1D5DB solid;
  text-decoration-thickness: auto;
  box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
  cursor: pointer;
  user-select: none;
  -webkit-user-select: none;
  touch-action: manipulation;
}

.submit-button:hover {
  background-color: rgb(249, 250, 251);
}

.submit-button:focus {
  outline: 2px solid transparent;
  outline-offset: 2px;
}

.submit-button:focus-visible {
  box-shadow: none;
}
</style>