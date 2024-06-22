<template>
  <div id="app">
    <div class="large-font">
      <div>
        Hello from WebAssembly!
        <div>Input: {{ input }}</div>
        <div>Result: {{ result }}</div>
      </div>
    </div>
  </div>
</template>

<script>
import { heavyComputation } from "@/services/heavyComputation";
import * as wasm from 'wasm-rust';

// console.log(wasm.add(1, 2));
// console.log(wasm.subtract(3, 4));
// console.log(wasm.multiply(5, 6));

const n = 5000;
//
//
// console.time('wasm');
// console.log(wasm.heavy_computation(n));
// console.timeEnd('wasm');
//
// console.time('js');
// console.log(heavy_computation(n));
// console.timeEnd('js');

const randomArray = Array.from({ length: 50000000 }, () => Math.random());

console.time('wasm-sum');
console.log(
    wasm.sum(randomArray),
)
console.timeEnd('wasm-sum');

console.time('js-sum');
console.log(sum(randomArray));
console.timeEnd('js-sum');

function sum_of_squares(n) {
    let sum = 0;
    for (let i = 0; i < n; i++) {
        sum += i * i;
    }
    return sum;
}

function sum(arr) {
    return arr.reduce((acc, val) => acc + val, 0);
}

function sum2(arr) {
    let sum = 0n;
    for (let i = 0; i < arr.length; i++) {
        sum += arr[i];
    }
    return sum;
}

function heavy_computation(n) {
  let sum = BigInt(0);
  for (let i = BigInt(0); i < n; i++) {
    sum += BigInt(1);
  }
  return sum;
}

function fibonacci(n) {
  if (n <= 1) {
    return n;
  }
  return fibonacci(n - 1) + fibonacci(n - 2);
}


export default {
  name: 'App',
  data() {
    return {
      input: 42,
      result: 0,
    };
  },
  async mounted() {
    this.result = await heavyComputation(this.input);
  },
}
</script>

<style>
.large-font {
  display: flex;
  justify-content: center;
  font-size: 5em;
}
</style>
