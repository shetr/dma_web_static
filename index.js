import init, { gcd, lib_init, test_panic } from "./pkg/dma_web.js";
//import * as dma from "./pkg/dma_web.js";

init().then(() => {
    lib_init();
});

const aInput = document.getElementById("aInput");
const bInput = document.getElementById("bInput");
const computeButton = document.getElementById("computeButton");
const panicButton = document.getElementById("panicButton");
const result = document.getElementById("result");

computeButton.addEventListener('click', event => {
    let a = parseInt(aInput.value);
    let b = parseInt(bInput.value);
    result.innerText = gcd(a, b).toString();
});

panicButton.addEventListener('click', event => {
    test_panic();
});
