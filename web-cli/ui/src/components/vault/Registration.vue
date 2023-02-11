<script lang="ts">
import {defineComponent} from 'vue'
import init, {create_meta_vault, generate_user_credentials, get_vault, register} from "meta-secret-web-cli";
import {RegistrationStatus, VaultInfoStatus} from "@/model/models";
import router from "@/router";

import "@/common/DbUtils"
import {AppState} from "@/stores/app-state"

export default defineComponent({

  async setup() {
    const appState = AppState();
    return {
      appState: appState,
      joinComponent: false,

      vaultName: '',
      deviceName: ''
    }
  },

  watch: {},

  methods: {

    async generateVault() {
      await init();
      await create_meta_vault(this.vaultName, this.deviceName);
      await generate_user_credentials();

      let vault = await get_vault();

      if (vault.data.vaultInfo === VaultInfoStatus.NotFound) {
        await this.userRegistration();
      }

      // Unknown status means, user is not a member of a vault
      if (vault.data.vaultInfo === VaultInfoStatus.Unknown) {
        //join to the vault or choose another vault name
        this.joinComponent = true;
      }
    },

    async join() {
      await init();
      //send join request
      console.log("js user sig: ", JSON.parse(localStorage.user).userSig);
      return await this.userRegistration();

    },

    async userRegistration() {
      let registrationStatus = await register();
      console.log("registration status: ", registrationStatus.data);
      switch (registrationStatus.data) {
        case RegistrationStatus.Registered:
          // register button gets unavailable, vaultName kept in local storage
          router.push({path: '/vault/secrets'})
          return;
        case RegistrationStatus.AlreadyExists:
          alert("Join request has been sent, please wait for approval");
          return;
        default:
          alert("Unknown error!!!!! Unknown registration status! Invalid response from server");
          return;
      }
    },

    isNewVault() {
      return this.appState.metaVault == undefined;
    },
  }
})

</script>

<template>
  <div v-if="this.isNewVault()">
    <div class="container flex items-center max-w-md py-2">
      <label>User:</label>
    </div>

    <div class="container flex items-center justify-center max-w-md border-b border-t border-l border-r py-2 px-4">
      <label>@</label>
      <input
          :class="$style.nicknameUserInput"
          type="text"
          placeholder="vault name"
          aria-label="vault_name"
          v-model="vaultName"
      >
      <input
          :class="$style.nicknameUserInput"
          type="text"
          placeholder="device name"
          aria-label="device_name"
          v-model="deviceName"
      >
      <button
          class="flex-shrink-0 bg-teal-500 hover:bg-teal-700 border-teal-500 hover:border-teal-700 text-sm border-4 text-white py-1 px-2 rounded"
          type="button"
          @click="generateVault"
      >
        Register
      </button>
    </div>
  </div>

  <div v-if="joinComponent">
    <div class="container flex items-center max-w-md py-2 px-4">
      <label :class="$style.joinLabel">
        Vault already exists, would you like to join?
      </label>
      <button
          class="flex-shrink-0 bg-teal-500 hover:bg-teal-700 border-teal-500 hover:border-teal-700 text-sm border-4 text-white py-1 px-4 rounded"
          type="button"
          @click="join"
      >
        Join
      </button>
    </div>
  </div>
</template>

<style module>
.joinLabel {
  @apply appearance-none bg-transparent border-none w-full text-gray-700 mr-3 py-1 leading-tight focus:outline-none
}

.nicknameUserInput {
  @apply appearance-none bg-transparent border-none w-full text-gray-700 mr-3 py-1 px-2 leading-tight focus:outline-none
}
</style>