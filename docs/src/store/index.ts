import { createPinia, type Pinia } from 'pinia';

// Pinia Stores
import useConfigStore from './ConfigStore';

/** Pinia Store */
const pinia: Pinia = createPinia();
export default pinia;

export { useConfigStore };
