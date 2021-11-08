import { initWebGL } from "../../RPGFgin/src/main";
import { Game } from "./game";

const foo = import("../../pkg/index").then((x) => {
  console.log(x);
  x.main_js();
  x.foo().then((x) => {
    console.log(x);
  });

  // const ptr = x.getVertices();
  // const [length, pArray] = Array.from(new Uint32Array(x.memory.buffer, ptr, 2));
  // const data = new Float32Array(x.memory.buffer, pArray, length);
  // console.log(data);
});

// initWebGL("canvas3d");

// function main() {
//   const game = new Game();

//   const mainLoop = () => {
//     game.render();

//     requestAnimationFrame(mainLoop);
//   };

//   mainLoop();
// }

// main();
