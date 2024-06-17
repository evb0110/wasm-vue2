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
import {heavyComputation} from "@/services/heavyComputation";

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
