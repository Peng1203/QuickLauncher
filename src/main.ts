import { createApp } from 'vue';
import { setupApp } from './app/setupApp';
import App from './App.vue';
import router from './router';
import store from './store';
import './styles/global.css';
import './styles/tailwind.css';

// 放入Main.ts中的js代码 每个窗口都会单独执行
setupApp();

createApp(App).use(router).use(store).mount('#app');
