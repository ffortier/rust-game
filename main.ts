import init, { Game } from 'rust-game';
import './styles.css';

const filterStrength = 20;

let frameTime = 0;
let lastFrame = Date.now();

const measureFPS = () => {
    const thisFrame = Date.now();
    const thisFrameTime = thisFrame - lastFrame
    frameTime += (thisFrameTime - frameTime) / filterStrength;
    lastFrame = thisFrame;
};

const fpsOut = document.getElementById('fps') as HTMLElement;

setInterval(() => {
    fpsOut.innerHTML = (1000 / frameTime).toFixed(1) + " fps";
}, 1000);

init().then(() => {
    const game = new Game({
        devMode: true,
    });

    game.addEventListener('frame', measureFPS)

    game.run();
});