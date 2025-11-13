import { createPlugin } from '@tauri-store/pinia';
import { createPinia } from 'pinia';

const store = createPinia();
store.use(createPlugin());

export default store;
