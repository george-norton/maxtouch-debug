<script setup lang="ts">
  import { ref, onMounted, onBeforeUnmount, watch } from "vue";
  import { invoke } from "@tauri-apps/api/core";
  const timer = ref();
  const modes = ref([{ name: "Mutual Capacitance Delta Values", value: 16 },
  { name: "Mutual Capacitance Reference Values", value: 17 }]);
  const mode = ref(modes.value[0]);
  const connected = ref(false);
  const info_block = ref();
  const mouse_mode = ref(false);

  onMounted(() => {
    connect();

    watch(mouse_mode, (enabled) => {
      invoke("set_mouse_mode", { enable: enabled });
    });
  })

  onBeforeUnmount(() => {
    clearInterval(timer.value);
    timer.value = null;
  })

  // TODO: The device connection should not be owned by the debug plot. Other components also need to share it.
  function connect() {
    invoke("connect").then((info) => {
      connected.value = true;
      info_block.value = info;
      (invoke("get_mouse_mode") as Promise<boolean>).then((enabled) => {
        mouse_mode.value = enabled;
      });
      timer.value = setInterval(function () {
        const debug_mode = mode.value.value;
        var low_limit = -128;
        var high_limit = 700;
        if (debug_mode == 17) {
          // The 1066 and 336 sensors have different signal limit ranges.
          if (info_block.value.family_id === 164) {
            low_limit = 17500;
            high_limit = 31000;
          }
          else {
            low_limit = 1200;
            high_limit = 14600;
          }
        }
        (invoke("get_debug_image", { mode: debug_mode, low: low_limit, high: high_limit }) as Promise<ArrayBuffer>)
          .then((data) => {
            let imgData = new Blob([data], { type: 'application/octet-binary' });
            let link = URL.createObjectURL(imgData);
            const img = document.getElementById('img');
            if (img != null) {
              img.onload = () => URL.revokeObjectURL(link);
              img.setAttribute("src", link);
              img.style.display = "block";
            }
          })
          .catch((e) => {
            console.log(e);
            connected.value = false;

            // If an error occured, try to connect to a device in 1 second
            if (timer.value != null) {
              clearInterval(timer.value);
              timer.value = null;
            }
            setTimeout(connect, 1000);
          });
      }, 750);
    }).catch((e) => {
      console.log(e);
      connected.value = false;
      // If we failed to connect, try again in 1 second
      setTimeout(connect, 1000);
    });
  }

</script>

<template>
  <div>
    <div class="container">
      <div class="spacer" />
      <div class="plot">
        <img v-if="!connected" src="../assets/not_found.png" alt="Device not connected"
          style='object-fit: contain; width: 100%; height: 100%; 
          vertical-align: middle;' />
        <img v-else id="img" src="" alt="Maxtouch sensor debug data"
          style='image-rendering: pixelated; object-fit: contain; width: 100%; height: 100%; display: none' />
      </div>
      <div class="spacer" />
      <div class="toolbar">
        <Select v-model="mode" editable :options="modes" optionLabel="name" style="width: 250pt" />
        <ToggleButton v-model="mouse_mode" onLabel="Force digitizer mode" offLabel="Force mouse mode" />
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