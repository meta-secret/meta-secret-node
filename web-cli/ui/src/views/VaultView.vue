<script lang="ts">
import {defineComponent} from 'vue'
import init, {generate_security_box, get_user_sig} from "meta-secret-web-cli";
import * as models from '@/model/models';

interface DeviceInfo {
  deviceId: string;
  deviceName: string
}

interface User {

  userSignature?: UserSignature
}

interface UserSecurityBox {
  vaultName: string;
  keyManager?: KeyManager;

}

interface UserSignature {
  vaultName: string;
  device: DeviceInfo;
  publicKey: Base64EncodedText,
  transportPublicKey: Base64EncodedText
}

interface KeyManager {
  dsa: SerializedDsaKeyPair;
  transport: SerializedTransportKeyPair
}

interface SerializedDsaKeyPair {
  keyPair: Base64EncodedText;
  publicKey: Base64EncodedText
}

interface SerializedTransportKeyPair {
  secretKey: Base64EncodedText;
  publicKey: Base64EncodedText
}

interface Base64EncodedText {
  base64Text: string
}

interface Share {
  msg: string
}

interface PasswordStorage {
  shares: Array<Share>
}

export default defineComponent({
  data() {
    let defaultUser: User = {};
    let defaultPasswordStorage: PasswordStorage = {
      shares: []
    };

    return {
      user: defaultUser,
      userId: '',
      passwordStorage: defaultPasswordStorage
    }
  },
  mounted() {
    if (localStorage.user) {
      this.user = localStorage.user;
    }

    if (localStorage.passwordStorage) {
      this.passwordStorage = localStorage.passwordStorage;
    }
  },

  watch: {
    user(newUser) {
      localStorage.user = newUser;
    }
  },

  methods: {
    generateUser() {
      init().then(() => {
        let device: DeviceInfo = {
          deviceId: "yay",
          deviceName: "d1"
        }

        let securityBox = generate_security_box("test_vault");
        let userSig = get_user_sig(securityBox, device);

        //this.user = {
        //    keyManager
        //};

        let xxx: models.Base64EncodedText = {
          base64Text: "qwe"
        }

        console.log("base64::: ", JSON.stringify(xxx, null, 2))

        console.log("security box:", JSON.stringify(securityBox, null, 2), "has been registered");
        console.log("user sig:", JSON.stringify(userSig, null, 2), "has been registered");
      })
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
        @click="generateUser"
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
