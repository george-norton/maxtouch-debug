<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from "vue";
import { invoke } from "@tauri-apps/api/core";
const timer = ref();
const modes = ref([ { name: "Mutual Capacitance Delta Values", value: 16 },
                    { name: "Mutual Capacitance Reference Values", value: 17 }]);
const mode = ref(modes.value[0]);

onMounted(() => {
  invoke("connect");
  // TODO: If the timer updates too quickly, we will hang.
  timer.value = setInterval(async function () {
    const data : ArrayBuffer = await invoke("get_debug_image", { mode: mode.value.value });
    let imgData = new Blob([data], { type: 'application/octet-binary' });
    let link = URL.createObjectURL(imgData);

    const img = document.getElementById('img');
    if (img) {
      img.onload = () => URL.revokeObjectURL(link);
      img.setAttribute("src", link);
    }
  }, 750);
})

onBeforeUnmount(() => {
  clearInterval(timer.value);
  timer.value = null;
})

</script>

<template>
  <div>
    <div class="container">
      <div class="spacer" />
      <div class="plot">
        <img id="img" src="" alt="Maxtouch sensor debug data"
          style='image-rendering: pixelated; object-fit: contain; width: 100%; height: 100%;' />
      </div>
      <div class="spacer" />
      <div class="toolbar">
        <Select v-model="mode" editable :options="modes" optionLabel="name" style="width: 250pt" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  max-height: 100vh;
  height: 100%;
  flex-direction: column;
  flex-wrap: wrap;
  background-color: #f6f6f6;
}

.spacer {
  flex: 1;
}

.plot {
  flex: 100;
  height: 100%;
}

.toolbar {
  flex: 0;
}
</style>