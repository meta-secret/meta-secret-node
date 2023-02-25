<script lang="ts">
import type {UserSignature} from "@/model/UserSignature";
import init, {get_vault, membership} from "meta-secret-web-cli";
import type {VaultInfoData} from "@/model/VaultInfoData";
import type {DeviceUiElement} from "@/stores/app-state";
import {AppState} from "@/stores/app-state"
import router from "@/router";
import {MembershipRequestType} from "@/model/MembershipRequestType";


export default {

  async setup() {
    await init();

    const appState = AppState();

    let vaultResp = await get_vault();
    let vault = vaultResp.data as VaultInfoData;
    console.log("vault: ", JSON.stringify(vault, null, 2));

    if (vault.vault) {
      let activeDevices = getDevices(vault.vault.signatures, "active");
      let pendingDevices = getDevices(vault.vault.pendingJoins, "pending");
      appState.devices = activeDevices.concat(pendingDevices);

      return {
        appState: appState
      }
    }

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
    async accept(deviceInfo: DeviceUiElement) {
      await init();
      await this.membership(deviceInfo, MembershipRequestType.Accept);
    },

    async decline(deviceInfo: DeviceUiElement) {
      await init()
      await this.membership(deviceInfo, MembershipRequestType.Decline);
    },


    async membership(deviceInfo: DeviceUiElement, requestType: MembershipRequestType) {
      let membershipResult = membership(deviceInfo.userSig, requestType);
      console.log("membership operation: ", membershipResult)
      //TODO check the operation status

      router.push({path: '/vault/devices'})
    },
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
      <li v-for="deviceInfo in appState.devices" :key="deviceInfo.userSig.device.deviceId" class="flex flex-row">
        <div class="flex items-center flex-1 p-4 cursor-pointer select-none">
          <div class="flex-1 pl-1 mr-16">
            <div class="font-medium dark:text-white">
              {{ deviceInfo.userSig.device.deviceName }}
            </div>
            <div class="text-sm text-gray-600 dark:text-gray-200 truncate">
              <p class="truncate w-24">
                {{ deviceInfo.userSig.device.deviceId }}
              </p>
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
