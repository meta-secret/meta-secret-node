import {defineStore} from "pinia";
import init, {ApplicationStateManager} from "meta-secret-web-cli";
import {ApplicationState} from "@/model/ApplicationState";

class JsAppStateManager {
  appState: any
  
  constructor(appState) {
    this.appState = appState;
  }
  
  async updateJsState() {
    this.appState.internalState = await this.appState.stateManager.get_state();
  }
}

export const AppState = defineStore("app_state", {
  state: () => {
    console.log("App state. Init");
    
    let internalState: ApplicationState = {
      joinComponent: false,
      metaVault: undefined,
      vault: undefined,
      metaPasswords: []
    };

    return {
      internalState: internalState,
      stateManager: undefined as ApplicationStateManager | undefined,
    };
  },

  actions: {
    async appStateInit() {
      console.log("Js: App state init");
      await init();
      
      let jsAppStateManager = new JsAppStateManager(this);
      
      let stateManager = ApplicationStateManager.new(jsAppStateManager);
      this.stateManager = stateManager;
      
      await stateManager.init();
    }
  },
});
