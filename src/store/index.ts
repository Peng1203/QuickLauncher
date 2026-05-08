import { createPlugin } from '@tauri-store/pinia';
import { createPinia } from 'pinia';
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate';

const store = createPinia();
// 持久化存储插件 初始化时直接使用存储数据
store.use(piniaPluginPersistedstate);
// 多窗口状态同步插件
store.use(createPlugin());

export default store;
