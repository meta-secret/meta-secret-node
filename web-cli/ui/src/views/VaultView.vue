<script lang="ts">
import {defineComponent} from 'vue'
import type {BoxKeyPair} from 'tweetnacl';
import nacl from 'tweetnacl';

interface Vault {
  userId?: string;
  keyPair?: BoxKeyPair;
}

interface Share {
  msg: string
}

interface PasswordStorage {
  shares: Array<Share>
}

export default defineComponent({
  data() {
    let defaultVault: Vault = {};
    let defaultPasswordStorage: PasswordStorage = {
      shares: []
    };

    return {
      vault: defaultVault,
      userId: '',
      passwordStorage: defaultPasswordStorage
    }
  },
  mounted() {
    if (localStorage.vault) {
      this.vault = localStorage.vault;
    }

    if (localStorage.passwordStorage) {
      this.passwordStorage = localStorage.passwordStorage;
    }
  },

  watch: {
    vault(newVault) {
      localStorage.vault = newVault;
    }
  },

  methods: {
    generateVault() {
      //if (this.vault.empty) {
      //return this.vault;
      //}

      this.vault = {
        keyPair: nacl.box.keyPair(),
        userId: this.userId
      };

      console.log("user:", this.vault.userId, "has been registered");
    },

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
    },

    alert() {
      alert('yay');
    }
  }
})

</script>

<style>

</style>

<template>

  <div class="flex justify-center py-6">
    <p class="text-2xl">Distributed Vault</p>
  </div>

  <div class="py-4"></div>

  <div class="container flex items-center justify-center max-w-md border-b border-t py-2 px-4">
    <label>@</label>
    <input
        class="appearance-none bg-transparent border-none w-full text-gray-700 mr-3 py-1 px-2 leading-tight focus:outline-none"
        type="text"
        placeholder="user_id"
        aria-label="Full name"
        v-model="userId"
    >
    <button
        class="flex-shrink-0 bg-teal-500 hover:bg-teal-700 border-teal-500 hover:border-teal-700 text-sm border-4 text-white py-1 px-2 rounded"
        type="button"
        @click="generateVault"
    >
      Register
    </button>
  </div>

  <div class="py-4"/>

  <div class="container flex flex-col items-center justify-center max-w-md py-2 px-4">
    <div class="">
      <p class="text-xl py-2">Password</p>
    </div>

    <div class="flex py-4 px-4">
      <input class="border px-2" type="text">
      <button class="px-4 border" @click="addPassword">Add</button>
    </div>
  </div>

  <div class="container flex flex-col py-4 px-4 border max-w-md">
    <div class="flex flex-col py-2 hover:cursor-pointer" @click="alert">
      <div class="px-2">Password</div>
      <div class="px-4">id: 12345678</div>
      <div class="px-4">share: my share 1</div>
    </div>


  </div>


</template>
