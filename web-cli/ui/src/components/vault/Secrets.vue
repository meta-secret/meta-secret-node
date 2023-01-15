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
  <div class="container flex flex-col items-center justify-center max-w-md py-2 px-4">
    <div class="flex">
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