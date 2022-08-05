import { provideFASTDesignSystem, allComponents, neutralLayer1, fillColor } from '@microsoft/fast-components';
import init, { Game } from 'rust-game';
import './styles.css';

provideFASTDesignSystem().register(allComponents);

fillColor.setValueFor(document.body, neutralLayer1.getValueFor(document.body));

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
        container: document.getElementById("game") ?? undefined,
    });

    game.addEventListener('frame', measureFPS)

    document.getElementById('run')?.addEventListener('click', () => game.run());
    document.getElementById('stop')?.addEventListener('click', () => game.stop());
    document.getElementById('reset')?.addEventListener('click', () => game.reset());
});