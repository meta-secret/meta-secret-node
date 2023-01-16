<script lang="ts">
import init, {get_vault} from "meta-secret-web-cli";
import type {UserSignature} from "@/model/UserSignature";
import type {VaultInfoData} from "@/model/VaultInfoData";
import type {DeviceInfo} from "@/model/DeviceInfo";

interface DeviceUiElement {
  device: DeviceInfo
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

      let activeDevices = getDevices(vault.vault?.signatures, "active");
      let pendingDevices = getDevices(vault.vault?.pendingJoins, "pending");

      this.devices = activeDevices.concat(pendingDevices);
    });

    function getDevices(signatures: Array<UserSignature>, status: string) {
      return signatures.map(memberSig => {
        let el: DeviceUiElement = {
          device: memberSig.device,
          status: status
        }

        return el;
      });
    }
  }
}
</script>

<template>
    <div class="py-4"/>

    <!-- https://www.tailwind-kit.com/components/list -->
    <div :class="$style.devices">
      <div class="w-full px-4 py-5 border-b sm:px-6">
        <h3 class="text-lg font-medium leading-6 text-gray-900 dark:text-white">
          Devices
        </h3>
        <p class="max-w-2xl mt-1 text-sm text-gray-500 dark:text-gray-200">
          Detailed information about user devices
        </p>
      </div>
      <ul class="w-full flex flex-col divide-y divide p-2">
        <li v-for="device in devices" :key="device.deviceId" class="flex flex-row">
          <div class="flex items-center flex-1 p-4 cursor-pointer select-none">
            <div class="flex-1 pl-1 mr-16">
              <div class="font-medium dark:text-white">
                {{ device.device.deviceName }}
              </div>
              <div class="text-sm text-gray-600 dark:text-gray-200">
                {{ device.device.deviceId }}
              </div>
            </div>
            <div class="text-xs text-gray-600 dark:text-gray-200">
              {{ device.status }}
            </div>
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
</style>
