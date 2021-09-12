export let gl: WebGL2RenderingContext = null;

export function initWebGL(canvasID: string): void {
  if (!canvasID) throw new Error("Invalid element id");

  const canvasEL = document.getElementById(canvasID);

  if (canvasEL instanceof HTMLCanvasElement) {
    canvasEL.width = window.innerWidth;
    canvasEL.height = window.innerHeight;

    init(canvasEL);
  } else {
    throw new Error("Invalid canvas id");
  }
}

function init(canvasEL: HTMLCanvasElement): void {
  let context = canvasEL.getContext("webgl2");
  if (context instanceof WebGL2RenderingContext) {
    gl = context;

    const [, maxLineWidth] = gl.getParameter(gl.ALIASED_LINE_WIDTH_RANGE);

    gl.enable(gl.DEPTH_TEST);
    gl.lineWidth(maxLineWidth);
  } else {
    throw new Error("WEBGL2 it not supported!");
  }
}
