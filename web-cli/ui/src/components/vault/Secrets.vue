<script lang="ts">
import {defineComponent} from 'vue'
import init, {get_meta_passwords} from "meta-secret-web-cli";
import type {UserSignature} from "@/model/UserSignature";
import type {MetaPasswordsData} from "@/model/MetaPasswordsData";

interface Share {
  msg: string
}

interface PasswordStorage {
  shares: Array<Share>
}

export default defineComponent({
  data() {
    let defaultPasswordStorage: PasswordStorage = {
      shares: []
    };

    return {
      userId: '',
      passwordStorage: defaultPasswordStorage,
      secrets: {}
    }
  },
  mounted() {
    if (localStorage.userId) {
      this.userId = localStorage.userId;
    }

    if (localStorage.passwordStorage) {
      this.passwordStorage = localStorage.passwordStorage;
    }

    init().then(async () => {
      let userSig = JSON.parse(localStorage.user).userSig as UserSignature;
      let passwordsResp = await get_meta_passwords(userSig);
      this.secrets = passwordsResp.data as MetaPasswordsData;
    });
  },

  methods: {
    addPassword() {
      alert("Add new password!")
    }
  }
})

</script>


<template>
  <div class="py-2"/>

  <div :class="$style.newSecret">
    <button
        class="flex-shrink-0 bg-orange-400 hover:bg-orange-700 border-orange-500 hover:border-orange-700 text-sm border-2 text-white py-1 px-4 rounded"
        type="button"
        @click="addPassword"
    >
      Add
    </button>
  </div>

  <div class="py-4"/>

  <!-- https://www.tailwind-kit.com/components/list -->
  <div :class="$style.secrets">
    <ul class="w-full flex flex-col divide-y divide p-2">
      <li v-for="secret in this.secrets.passwords" :key="secret.id" class="flex flex-row">
        <div class="flex items-center flex-1 p-4 cursor-pointer select-none">
          <div class="flex-1 pl-1 mr-16">
            <div class="font-medium dark:text-white">
              {{ secret.id.id }}
            </div>
            <div class="text-sm text-gray-600 dark:text-gray-200">
              {{ secret.id.name }}
            </div>
          </div>
          <div class="text-xs text-gray-600 dark:text-gray-200">
            metameta
          </div>
        </div>
      </li>
    </ul>
  </div>
</template>

<style module>
.secrets {
  @apply container max-w-md flex flex-col items-center justify-center w-full py-20;
  @apply mx-auto bg-white shadow dark:bg-gray-800;
}

.newSecret {
  @apply container max-w-md flex flex-row justify-end mx-auto dark:bg-gray-800;
}
</style>
