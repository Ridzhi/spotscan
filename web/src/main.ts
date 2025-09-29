import Vant from 'vant';
import 'vant/lib/index.css';
import './assets/index.css';
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { plugin as VueInputAutowidth } from 'vue-input-autowidth'
import App from './App.vue';
import router from './router';
import { errorHandler } from './errorHandler';


const pinia = createPinia();
const app = createApp(App);
app.config.errorHandler = errorHandler;
app.use(pinia);
app.use(router);
app.use(VueInputAutowidth);

app.use(Vant);

import { retrieveLaunchParams } from '@telegram-apps/sdk-vue';
import { init } from './init';
// Mock the environment in case, we are outside Telegram.
import './mockEnv'

init(retrieveLaunchParams().startParam === 'debug' || import.meta.env.DEV);

app.mount('#app');
