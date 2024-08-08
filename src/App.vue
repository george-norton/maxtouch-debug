<script setup lang="ts">
import DebugPlot from "./components/DebugPlot.vue";
import RegisterMap from "./components/RegisterMap.vue";
</script>

<template>
  <div class="container">
    <Splitter style="height: 100%;" class="mb-8">
        <SplitterPanel class="flex items-center justify-center" :size="75">
          <DebugPlot style="height: 100%;"/>
        </SplitterPanel>
        <SplitterPanel class="flex items-center justify-center"  style="overflow: scroll;" :size="25">
          <RegisterMap />
        </SplitterPanel>
    </Splitter>
  </div>
</template>

<style scoped>

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  padding: 0;
  margin: 0;
  border: 0;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  position: fixed;
  width: 100%;
  height: 100%;
}

.plot {
  flex: 70%;
}

.registers {
  flex: 30%;
  overflow: scroll;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }

}
</style>
