<script lang="ts">
import {defineComponent, onBeforeUnmount, onMounted} from "vue";
import init, {cluster_distribution, recover,} from "meta-secret-web-cli";

function setupDbSync() {
  console.log("Setup db sync");

  let polling: any = null;

  onMounted(() => {
    console.log("Setup Db sync scheduler");
    polling = setInterval(async () => {
      console.log("db_sync!!!!!!!!!!!!!!!!11");
      //await db_sync();
    }, 3000);
  });

  onBeforeUnmount(async () => {
    clearInterval(polling);
  });
  return polling;
}

export default defineComponent({
  async setup() {
    console.log("Secrets Component. Init");

    let polling = setupDbSync();

    await init();

    let passwordsResp = {}; //await get_meta_passwords();
    let secrets = {};//passwordsResp.data as MetaPasswordsData;

    return {
      newPassword: "",
      newPassDescription: "",

      secrets: secrets,
      polling: polling,
    };
  },

  methods: {
    async addPassword() {
      await init();
      await cluster_distribution(this.newPassDescription, this.newPassword);
    },

    async recover() {
      await init();
      console.log("Recover password!");
      await recover();
    },
  },
});
</script>

<template>
  <div class="py-2" />

  <div :class="$style.newPasswordDiv">
    <div class="flex items-center">
      <label>description: </label>
      <input
        type="text"
        :class="$style.passwordInput"
        placeholder="my meta secret"
        v-model="newPassDescription"
      />
    </div>
    <div class="flex items-center">
      <label>secret: </label>
      <input
        type="text"
        :class="$style.passwordInput"
        placeholder="top$ecret"
        v-model="newPassword"
      />
    </div>
    <div class="flex justify-end">
      <button :class="$style.addButton" @click="addPassword">Add</button>
    </div>
  </div>

  <div class="py-4" />

  <!-- https://www.tailwind-kit.com/components/list -->
  <div :class="$style.secrets">
    <ul class="w-full flex flex-col divide-y divide p-2">
      <li
        v-for="secret in this.secrets.passwords"
        :key="secret.id"
        class="flex flex-row"
      >
        <div class="flex items-center flex-1 p-4 cursor-pointer select-none">
          <div class="flex-1 pl-1 mr-16">
            <div class="font-medium dark:text-white">
              {{ secret.id.name }}
            </div>
            <div class="text-sm text-gray-600 dark:text-gray-200">
              {{ secret.id.id.slice(0, 12) }}
            </div>
          </div>
          <button :class="$style.actionButtonText" @click="recover">
            Recover
          </button>
        </div>
      </li>
    </ul>
  </div>
</template>

<style module>
.secrets {
  @apply container max-w-md flex flex-col items-center justify-center w-full;
  @apply mx-auto bg-white shadow dark:bg-gray-800;
}

.newPasswordDiv {
  @apply block max-w-md mx-auto items-center justify-center max-w-md border-b border-t border-l border-r py-2 px-4;
}

.passwordInput {
  @apply appearance-none bg-transparent border-none w-full text-gray-700 mr-3 py-1 px-2 leading-tight focus:outline-none;
}

.addButton {
  @apply flex-shrink-0 bg-orange-400 border-orange-500 text-sm border-2 text-white py-1 px-4 rounded;
  @apply hover:bg-orange-700 hover:border-orange-700;
}

.actionButtonText {
  @apply flex justify-end w-24 text-right;
}
</style>