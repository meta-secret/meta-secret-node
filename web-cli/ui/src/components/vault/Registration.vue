<script lang="ts">
import {defineComponent} from 'vue'
import init, {generate_security_box, get_user_sig, get_vault, register} from "meta-secret-web-cli";
import {
  type DeviceInfo,
  RegistrationStatus,
  type UserSecurityBox,
  type UserSignature,
  VaultInfoStatus
} from "@/model/models";
import router from "@/router";

export interface User {
  securityBox?: UserSecurityBox,
  userSig?: UserSignature
}

export default defineComponent({
  data() {
    let defaultUser: User = {};

    return {
      user: defaultUser,
      userId: '',
      joinComponent: false
    }
  },
  mounted() {
    if (localStorage.userId) {
      this.userId = localStorage.userId;
    }

    if (localStorage.user) {
      this.user = JSON.parse(localStorage.user);
    }
  },

  /**
   * https://v2.vuejs.org/v2/cookbook/client-side-storage.html
   */
  watch: {
    user(newUser: User) {
      localStorage.user = JSON.stringify(newUser);
    },
  },

  methods: {
    async generateUser() {
      init().then(async () => {
        let device: DeviceInfo = {
          deviceId: "yay",
          deviceName: "d1"
        }

        let securityBox = generate_security_box(this.userId);
        let userSig = get_user_sig(securityBox, device);
        console.log("generated user sig: ", userSig);

        this.user = {
          securityBox: securityBox,
          userSig: userSig
        };
        this.initUser();

        let vault = await get_vault(userSig);
        console.log("vault: ", JSON.stringify(vault));

        if (vault.data.vaultInfo === VaultInfoStatus.NotFound) {
          await this.userRegistration();
        }

        // Unknown status means, user is not a member of a vault
        if (vault.data.vaultInfo === VaultInfoStatus.Unknown) {
          //join to the vault or choose another vault name
          this.joinComponent = true;
        }
      })
    },

    async join() {
      init().then(async () => {
        //send join request
        console.log("js user sig: ", JSON.parse(localStorage.user).userSig);
        return await this.userRegistration();
      })
    },

    async userRegistration() {
      let userSig = JSON.parse(localStorage.user).userSig;
      console.log("User registration with: " + userSig);

      let registrationStatus = await register(userSig);
      console.log("registration status: ", registrationStatus.data);
      switch (registrationStatus.data) {
        case RegistrationStatus.Registered:
          // register button gets unavailable, userId kept in local storage
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

    cleanUpUser() {
      localStorage.setItem('userId', '');
    },

    isNewUser() {
      if (localStorage.userId) {
        if (localStorage.userId !== '') {
          return false;
        }
      }

      return true;
    },

    initUser() {
      localStorage.userId = this.userId;
      localStorage.user = JSON.stringify(this.user);
    }
  }
})

</script>

<template>
  <div>
    <div class="container flex items-center max-w-md py-2">
      <label>User:</label>
    </div>

    <div class="container flex items-center justify-center max-w-md border-b border-t border-l border-r py-2 px-4">
      <label>@</label>
      <input
          class="appearance-none bg-transparent border-none w-full text-gray-700 mr-3 py-1 px-2 leading-tight focus:outline-none"
          type="text"
          placeholder="user_id"
          aria-label="Full name"
          v-model="userId"
          v-bind:disabled="!isNewUser()"
      >
      <button
          class="flex-shrink-0 bg-teal-500 hover:bg-teal-700 border-teal-500 hover:border-teal-700 text-sm border-4 text-white py-1 px-2 rounded"
          type="button"
          @click="generateUser"
          v-if="isNewUser()"
      >
        Register
      </button>
    </div>
  </div>

  <div v-if="joinComponent">
    <div class="container flex items-center max-w-md py-2 px-4">
      <label
          class="appearance-none bg-transparent border-none w-full text-gray-700 mr-3 py-1 leading-tight focus:outline-none">
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
