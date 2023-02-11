import {defineStore} from "pinia";
import type {MetaVault} from "@/model/MetaVault";
import init, {get_meta_vault} from "meta-secret-web-cli";

export const AppState = defineStore({
  id: "app_state",
  state: () => {
    return {
      metaVault: undefined as MetaVault | undefined
    }
  },

  actions: {
    async loadMetaVault() {
      await init();
      this.metaVault = await get_meta_vault();
    },
  },
});
