import { FlatCanvas, Point2d, Color } from "./rust_wasm_test";
import { memory } from "./rust_wasm_test_bg";

const width = 256;
const height = 256;

const canvas = document.getElementById("main-canvas");
const context = canvas.getContext('2d');
const imageData = context.createImageData(width, height);

const flatCanvas = FlatCanvas.new(width, height);

flatCanvas.draw_line(Point2d.new(0, 0), Point2d.new(256, 32), Color.black());

const renderLoop = () => {
    const canvasPtr = flatCanvas.canvas();
    const rawCanvas = new Uint8Array(memory.buffer, canvasPtr, width * height * 4);

    imageData.data.set(rawCanvas);

    context.putImageData(imageData, 0, 0);

    requestAnimationFrame(renderLoop);
};


requestAnimationFrame(renderLoop);
