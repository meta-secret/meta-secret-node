import * as wasm from "meta-secret-web-cli";
import QRCode from 'qrcode'
import QRCodeStyling from "qr-code-styling";

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

        let shareIdText = 'share: ' + share['share_id'];
        let note1Text = document.getElementById('note1').value;
        let note2Text = document.getElementById('note2').value;

        let textImage = textToImage(note1Text, note2Text, shareIdText);
        let qrCodeStyling = generateQrCodeStyling(JSON.stringify(share), textImage);
        qrCodeStyling.append(qrImages);
        //generateQrCode(qr, share);
    });
}

function generateQrCode(qrHtml, share) {
    QRCode.toCanvas(qrHtml, JSON.stringify(share), { errorCorrectionLevel: 'H' }, function (error) {
        if (error) {
            console.error(error)
        }
        console.log('success!');
    });
}

function textToImage(line1, line2, line3) {
    let canvas = document.createElement("canvas");
    canvas.width = 600;
    canvas.height = 600;
    let ctx = canvas.getContext('2d');
    ctx.font = "100px Arial";
    ctx.fillText(line1,50,150);
    ctx.fillText(line2,50,300);
    ctx.fillText(line3,50,450);
    return canvas.toDataURL();
}

function generateQrCodeStyling(share, textImage) {
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