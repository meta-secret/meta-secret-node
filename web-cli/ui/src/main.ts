import {createApp} from "vue";
import {createPinia} from "pinia";

import AppManager from "./AppManager.vue";
import router from "./router";

import "./index.css"

const app = createApp(AppManager);

app.use(createPinia());
app.use(router);

app.mount("#app");
