<script lang="ts">
import {defineComponent} from 'vue'

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
      secrets: [{id: "secret1", description: "auf secret"}]
    }
  },
  mounted() {
    if (localStorage.userId) {
      this.userId = localStorage.userId;
    }

    if (localStorage.passwordStorage) {
      this.passwordStorage = localStorage.passwordStorage;
    }
  },

  methods: {
    addPassword() {
      console.log("Add password into a distributed storage");

      //get a password
      //split
      //encrypt your share
      //add your share into password db
      let share1: Share = {msg: 'share_1_password'};
      let share2: Share = {msg: 'share_2_password'};
      let share3: Share = {msg: 'share_3_password'};
      let shares: Array<Share> = [share1, share2, share3];

      //crypto-js to cypher shares with AES? Private key is a password
      this.passwordStorage.shares.push(share1);
    }
  }
})

</script>


<template>
  <div class="py-4"/>

  <!-- https://www.tailwind-kit.com/components/list -->
  <div :class="$style.secrets">
    <ul class="w-full flex flex-col divide-y divide p-2">
      <li v-for="secret in secrets" :key="secret.id" class="flex flex-row">
        <div class="flex items-center flex-1 p-4 cursor-pointer select-none">
          <div class="flex-1 pl-1 mr-16">
            <div class="font-medium dark:text-white">
              {{ secret.id }}
            </div>
            <div class="text-sm text-gray-600 dark:text-gray-200">
              {{ secret.description }}
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
  @apply container max-w-md flex flex-col items-center justify-center w-full;
  @apply mx-auto bg-white rounded-lg shadow dark:bg-gray-800;
}
</style>
