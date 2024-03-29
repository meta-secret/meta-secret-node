import {defineStore} from "pinia";
import type {MetaVault} from "@/model/MetaVault";
import init, {get_meta_vault} from "meta-secret-web-cli";
import type {UserSignature} from "@/model/UserSignature";

export interface DeviceUiElement {
  userSig: UserSignature
  status: string
}

export const AppState = defineStore({
  id: "app_state",
  state: () => {
    let emptyDevices: Array<DeviceUiElement> = []

    return {
      metaVault: undefined as MetaVault | undefined,
      joinComponent: false,
      devices: emptyDevices
    }
  },

  actions: {
    async loadMetaVault() {
      await init();
      this.metaVault = await get_meta_vault();
    },
  },
});
