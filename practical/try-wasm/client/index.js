import * as my_wasm from "try-wasm";
// import * as wasm from "hello-wasm-pack";

const run = async () => {
  my_wasm.test();
  console.log("Memory: ", my_wasm.memory);
  document.body.textContent = "Hello, WebAssembly!";
};

addEventListener("load", (event) => run())
