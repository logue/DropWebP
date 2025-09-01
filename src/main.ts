import store from '@/store';
import { createApp } from 'vue';

import '@/styles/settings.scss';
import App from '@/App.vue';
import vuetify from '@/plugins/vuetify';
const app = createApp(App);

app.use(vuetify);
app.use(store);
app.mount('#app');
