<script lang="ts">
import {defineComponent} from 'vue'
import init, {create_meta_vault, generate_user_credentials, get_vault, register} from "meta-secret-web-cli";
import {RegistrationStatus, VaultInfoData, VaultInfoStatus} from "@/model/models";
import router from "@/router";

import "@/common/DbUtils"
import {AppState} from "@/stores/app-state"

export default defineComponent({

  async setup() {
    console.log("Registration component. Init")

    const appState = AppState();
    return {
      appState: appState,
      vaultName: '',
      deviceName: ''
    }
  },

  watch: {},

  methods: {

    async generateVault() {
      console.log("Generate vault");

      await init();

      await create_meta_vault(this.vaultName, this.deviceName);
      await generate_user_credentials();

      let vault: VaultInfoData = await get_vault();

      if (vault.vaultInfo === VaultInfoStatus.NotFound) {
        await this.userRegistration();
      }

      // Unknown status means, user is not a member of a vault
      if (vault.vaultInfo === VaultInfoStatus.Unknown) {
        //join to the vault or choose another vault name
        alert("join to the vault or choose another vault name");
        this.appState.joinComponent = true;
      }
    },

    async join() {
      console.log("Registration component. Join cluster");

      await init();
      //send join request
      return await this.userRegistration();
    },

    async userRegistration() {
      console.log("Registration component. Start user registration");

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

    isEmptyEnv() {
      return this.appState.metaVault == undefined;
    },
  }
})

</script>

<template>
  <div v-if="this.isEmptyEnv()">
    <div class="container flex items-center max-w-md py-2">
      <label>User:</label>
    </div>

    <div class="container flex items-center justify-center max-w-md border-b border-t border-l border-r py-2 px-2">
      <label>@</label>
      <input
          :class="$style.nicknameUserInput"
          type="text"
          placeholder="vault name"
          v-model="vaultName"
      >
      <input :class="$style.nicknameUserInput" type="text" placeholder="device name" v-model="deviceName">

      <button
          :class="$style.registrationButton"
          @click="generateVault"
          v-if="!this.appState.joinComponent"
      >
        Register
      </button>
    </div>
  </div>

  <div v-if="this.appState.joinComponent">
    <div class="container flex items-center max-w-md py-2">
      <label :class="$style.joinLabel">
        Vault already exists, would you like to join?
      </label>
      <button :class="$style.joinButton" @click="join"> Join</button>
    </div>
  </div>
</template>

<style module>
.joinLabel {
  @apply appearance-none bg-transparent border-none w-full text-gray-700 mr-3 py-1 leading-tight focus:outline-none;
}

.registrationButton {
  @apply flex-shrink-0 bg-teal-500 border-teal-500 text-sm border-4 text-white py-1 px-4 rounded;
  @apply hover:bg-teal-700 hover:border-teal-700;
}

.joinButton {
  @apply flex-shrink-0 bg-teal-500;
  @apply hover:bg-teal-700 border-teal-500 hover:border-teal-700 text-sm border-4 text-white py-1 px-4 rounded;
}

.nicknameUserInput {
  @apply appearance-none bg-transparent border-none;
  @apply w-full text-gray-700 mr-3 py-1 px-2 leading-tight focus:outline-none;
}
</style>