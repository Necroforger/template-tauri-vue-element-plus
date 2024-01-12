import { createApp } from "vue";
import App from "./App.vue";
import './styles.css'

import ElementPlus from 'element-plus'
import 'element-plus/theme-chalk/index.css'
import 'element-plus/theme-chalk/dark/css-vars.css'
import * as ElementPlusIconsVue from '@element-plus/icons-vue';
import { createPinia } from "pinia";

const pinia = createPinia()

const app = createApp(App)
              .use(ElementPlus)
              .use(pinia)

for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component)
  }
app.mount("#app");
