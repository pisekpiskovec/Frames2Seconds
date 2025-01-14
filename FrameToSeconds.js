import readline from 'node:readline';
import clipboard from 'clipboardy';

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

rl.question("Frames: ", answer =>{
    const maxDotSeconds = 999999999;
    const maxFPS = 29.97;
    const secondsPlaces = Math.trunc((maxDotSeconds * answer) / maxFPS);
    var result = "";
    for(var i = 0; i < maxDotSeconds.toString().length - secondsPlaces.toString().length; i++) result += "0";
    result += secondsPlaces.toString();
    console.log(result);
    clipboard.write(result);
    rl.close();
});
