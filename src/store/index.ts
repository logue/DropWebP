import { createPinia, type Pinia } from 'pinia';

// Pinia Stores
import useConfigStore from '@/store/ConfigStore';
import useGlobalStore from '@/store/GlobalStore';

/** Pinia Store */
const pinia: Pinia = createPinia();
export default pinia;

export { useConfigStore, useGlobalStore };
