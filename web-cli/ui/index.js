import * as wasm from "meta-secret-web-cli";
import QRCode from 'qrcode'

let splitButton = document.getElementById("splitButton");
splitButton.onclick = function () {
    let pass = document.getElementById("password").value;
    splitPassword(pass);
}

function splitPassword(password) {
    console.log("Split password");

    let qrImages = document.getElementById('qr-images');

    while (qrImages.firstChild) {
        qrImages.removeChild(qrImages.firstChild);
    }

    let shares = wasm.split(password);

    shares.forEach(share => {
        let qr = document.createElement('canvas');
        qrImages.appendChild(qr);

        QRCode.toCanvas(qr, JSON.stringify(share), { errorCorrectionLevel: 'H' }, function (error) {
            if (error) {
                console.error(error)
            }
            console.log('success!');
        });
    });
}