import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import install from "./components";

createApp(App).use(router).use(install).mount("#app");
