import {defineStore} from "pinia";
import type {MetaVault} from "@/model/MetaVault";
import init, {get_meta_vault, WasmMetaClient} from "meta-secret-web-cli";
import type {UserSignature} from "@/model/UserSignature";

export interface DeviceUiElement {
  userSig: UserSignature
  status: string
}

export const AppState = defineStore({
  id: "app_state",
  state: () => {
    console.log("App state. Init");
    let emptyDevices: Array<DeviceUiElement> = []

    return {
      metaVault: undefined as MetaVault | undefined,
      joinComponent: false,
      devices: emptyDevices,
      metaClient: undefined as WasmMetaClient | undefined
    }
  },

  actions: {
    async loadMetaVault() {
      console.log("Load meta vault");
      await init();
      let asyncMetaVault = get_meta_vault();
      this.metaVault = await asyncMetaVault;
      this.metaClient = WasmMetaClient.new();
      
      return asyncMetaVault;
    },
  },
});
