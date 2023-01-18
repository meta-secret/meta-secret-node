<script lang="ts">
import {defineComponent} from 'vue'
import init, {cluster_distribution, get_meta_passwords, get_vault} from "meta-secret-web-cli";
import type {UserSignature} from "@/model/UserSignature";
import type {MetaPasswordsData} from "@/model/MetaPasswordsData";
import type {User} from "@/components/vault/Registration.vue";
import type {VaultInfoData} from "@/model/VaultInfoData";

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
      newPassword: '',
      passwordStorage: defaultPasswordStorage,
      secrets: {}
    }
  },
  created() {
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
      console.log(JSON.stringify(this.secrets, null, 2))
    });
  },

  methods: {
    addPassword() {
      init().then(async () => {
        console.log("Add new password!");
        let user = JSON.parse(localStorage.user) as User;

        let userSig = this.getUserSig();
        let vaultResponse = await get_vault(userSig);
        let vaultInfo = vaultResponse.data as VaultInfoData

        let id = Math.random().toString(36).substring(2,7)
        await cluster_distribution(id, this.newPassword, user.securityBox, userSig, vaultInfo.vault);
      });
    },

    restore() {
      init().then(async () => {
        alert("Restore password!")
      });
    },

    getUserSig() {
      let user = JSON.parse(localStorage.user) as User;
      if (user.userSig) {
         return user.userSig;
      } else {
        throw new Error("Critical error. User signature not present");
      }
    }
  }
})

</script>


<template>
  <div class="py-2"/>

  <div class="container flex items-center justify-center max-w-md border-b border-t border-l border-r py-2 px-4">
    <label>pass: </label>
    <input type="text" :class="$style.passwordInput" placeholder="top$ecret" v-model="newPassword">
    <button :class="$style.addButton" @click="addPassword">
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
              {{ secret.id.name }}
            </div>
            <div class="text-sm text-gray-600 dark:text-gray-200">
              {{ secret.id.id.slice(0, 12) }}
            </div>
          </div>
          <button :class="$style.actionButtonText" @click="restore">
            Restore
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

.passwordInput {
  @apply appearance-none bg-transparent border-none w-full text-gray-700 mr-3 py-1 px-2 leading-tight focus:outline-none
}

.addButton {
  @apply flex-shrink-0 bg-orange-400 border-orange-500 text-sm border-2 text-white py-1 px-4 rounded;
  @apply hover:bg-orange-700 hover:border-orange-700;
}

.actionButtonText {
  @apply flex justify-end w-24 text-right
}
</style>
