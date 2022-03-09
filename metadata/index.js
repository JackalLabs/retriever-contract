const express = require('express');
const app = express();
const port = 5555;
const fs = require("fs");

const PImage = require("pureimage");
const font = PImage.registerFont('./Poppins-Regular.ttf','Poppins');

PImage.decodePNGFromStream(fs.createReadStream("./jackal.png")).then((img) => {
    font.load(() => {
        app.get('/:name', (req, res) => {
    
            let name = req.params.name + ".rns";
        
            const img1 = PImage.make(700, 700);
            const ctx = img1.getContext('2d');
            ctx.fillStyle = '#040d21';
            ctx.fillRect(0,0,700,700);
        
            ctx.fillStyle = '#ffffff';
            ctx.font = "38pt 'Poppins'";
    
            let w = ctx.measureText(name).width;
            ctx.fillText(name, 350 - w / 2, 620);

            ctx.drawImage(img,
                0, 0, img.width, img.height, // source dimensions
                350 - (img.width / 4), 20, img.width / 2, img.height / 2                 // destination dimensions
            );
        
            PImage.encodePNGToStream(img1, res).then(() => {
            }).catch((e)=>{
            });
        });
        
        app.listen(port, () => {
          console.log(`ibcname app listening on port ${port}`)
        });
    });
});




