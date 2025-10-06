import { createPinia, type Pinia } from 'pinia';

import piniaPluginPersistedstate from 'pinia-plugin-persistedstate';

// Pinia Stores
import useConfigStore from './ConfigStore';
import useGlobalStore from './GlobalStore';

/** Pinia Store */
const pinia: Pinia = createPinia();
pinia.use(piniaPluginPersistedstate);
export default pinia;

export { useConfigStore, useGlobalStore };
