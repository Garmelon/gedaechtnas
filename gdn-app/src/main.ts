import { createPinia } from "pinia";
import { createApp } from "vue";
import App from "./App.vue";
import { useNotesStore } from "./stores/notes";

const app = createApp(App).use(createPinia());

await useNotesStore().initialize();

app.mount("#app");
