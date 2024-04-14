import init from "./pkg/try_wasm.js";

const run = async () => {
  const my_wasm = await init("./pkg/try_wasm_bg.wasm");
  my_wasm.test();
  console.log("Memory: ", my_wasm.memory);
  document.body.textContent = "Hello, WebAssembly!";
};

addEventListener("load", (event) => run())
