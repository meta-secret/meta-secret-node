<script lang="ts">
import init, {get_vault, membership as wasmMembership} from "meta-secret-web-cli";
import type {UserSignature} from "@/model/UserSignature";
import type {VaultInfoData} from "@/model/VaultInfoData";
import type {JoinRequest} from "@/model/JoinRequest";
import type {User} from "@/components/vault/Registration.vue";
import {MembershipRequestType} from "@/model/MembershipRequestType";
import router from "@/router";

export interface DeviceUiElement {
  userSig: UserSignature
  status: string
}

export default {
  data() {
    let emptyDevices: Array<DeviceUiElement> = []
    return {
      devices: emptyDevices
    }
  },

  created() {
    init().then(async () => {
      let userSig = JSON.parse(localStorage.user).userSig as UserSignature;

      let vaultResp = await get_vault(userSig);
      let vault = vaultResp.data as VaultInfoData;
      console.log("vault: ", JSON.stringify(vault, null, 2));

      let activeDevices = getDevices(vault.vault.signatures, "active");
      let pendingDevices = getDevices(vault.vault.pendingJoins, "pending");

      this.devices = activeDevices.concat(pendingDevices);
    });

    function getDevices(signatures: Array<UserSignature>, status: string) {
      return signatures.map(memberSig => {
        let el: DeviceUiElement = {
          userSig: memberSig,
          status: status
        }

        return el;
      });
    }
  },

  methods: {
    accept(deviceInfo: DeviceUiElement) {
      init().then(async () => {
        await this.membership(deviceInfo, MembershipRequestType.Accept);
      });
    },

    decline(deviceInfo: DeviceUiElement) {
      init().then(async () => {
        await this.membership(deviceInfo, MembershipRequestType.Decline);
      });
    },

    async membership(deviceInfo: DeviceUiElement, requestType: MembershipRequestType) {
      let user = JSON.parse(localStorage.user) as User;
      let joinRequest = this.getJoinRequest(user, deviceInfo);

      let membershipResult = wasmMembership(joinRequest, requestType);
      console.log("membership operation: ", membershipResult)
      //TODO check the operation status

      router.push({path: '/vault/devices'})
    },

    getJoinRequest(user: User, deviceInfo: DeviceUiElement) {
      let userSig;
      if (user.userSig) {
        userSig = user.userSig;
      } else {
        throw new Error("Critical error. User signature not present");
      }
      let joinRequest: JoinRequest = {
        member: userSig,
        candidate: deviceInfo.userSig
      }
      return joinRequest;
    }
  }
}
</script>

<template>
  <div class="py-4"/>

  <!-- https://www.tailwind-kit.com/components/list -->
  <div :class="$style.devices">
    <div :class="$style.listHeader">
      <h3 :class="$style.listTitle">
        Devices
      </h3>
      <p :class="$style.listDescription">
        Detailed information about user devices
      </p>
    </div>
    <ul class="w-full flex flex-col divide-y divide p-2">
      <li v-for="deviceInfo in devices" :key="deviceInfo.userSig.device.deviceId" class="flex flex-row">
        <div class="flex items-center flex-1 p-4 cursor-pointer select-none">
          <div class="flex-1 pl-1 mr-16">
            <div class="font-medium dark:text-white">
              {{ deviceInfo.userSig.device.deviceName }}
            </div>
            <div class="text-sm text-gray-600 dark:text-gray-200">
              {{ deviceInfo.userSig.device.deviceId }}
            </div>
          </div>
          <div class="text-xs text-gray-600 dark:text-gray-200">
            {{ deviceInfo.status }}
          </div>
          <button v-if="deviceInfo.status === 'pending'" :class="$style.actionButtonText" @click="accept(deviceInfo)">
            Accept
          </button>
          <button v-if="deviceInfo.status === 'pending'" :class="$style.actionButtonText" @click="decline(deviceInfo)">
            Decline
          </button>
        </div>
      </li>
    </ul>
  </div>
</template>


<style module>
.devices {
  @apply container max-w-md flex flex-col items-center justify-center w-full;
  @apply mx-auto bg-white rounded-lg shadow dark:bg-gray-800;
}

.actionButtonText {
  @apply flex justify-end w-24 text-right
}

.listHeader {
  @apply w-full px-4 py-5 border-b sm:px-6
}

.listTitle {
  @apply text-lg font-medium leading-6 text-gray-900 dark:text-white
}

.listDescription {
  @apply max-w-2xl mt-1 text-sm text-gray-500 dark:text-gray-200
}

</style>
