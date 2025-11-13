import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import store from './store';
import { useWindowGuards } from './composables/useWindowGuards';
import './styles/global.css';
import './styles/tailwind.css';

// 放入Main.ts 中的js 代码都会单独执行
// 加载应用配置
useWindowGuards();

createApp(App).use(router).use(store).mount('#app');
