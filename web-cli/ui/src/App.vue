<script lang="ts">
import NavBar from "@/components/NavBar.vue";
import {defineComponent, onBeforeUnmount, onMounted} from "vue";
import init, {WasmMetaServer} from "meta-secret-web-cli";

async function setupMetaServer() {
  console.log("Setup meta server");

  let polling: any = null;

  onMounted(async () => {
    console.log("Setup meta server scheduler");
    polling = setInterval(async () => {
      await init();
      await WasmMetaServer.run_server();
    }, 3000);
  });

  onBeforeUnmount(async () => {
    clearInterval(polling);
  });
  return polling;
}

export default defineComponent({
  components: {NavBar},

  async setup() {
    let polling = await setupMetaServer();
    return {
      polling: polling
    }
  }
});

</script>

<template>
  <header>
    <NavBar/>
  </header>

  <div class="py-4"/>

    <div>
      <RouterView/>
    </div>
</template>

<style>

.container {
  display: flex;
  justify-content: flex-start;
  max-width: 1376px;
  margin: 0 auto;
}

</style>
