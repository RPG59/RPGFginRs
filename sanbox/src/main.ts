import { initWebGL } from "../../RPGFgin/src/main";
import { Game } from "./game";

const foo = import('../../pkg/index').then(x => {
  console.log(x);
  // x.main_js();
  x.foo().then(x => {
    console.log(x);
    
  })
})


// initWebGL("canvas3d");

function main() {
  const game = new Game();

  const mainLoop = () => {
    game.render();

    requestAnimationFrame(mainLoop);
  };

  mainLoop();
}

// main();