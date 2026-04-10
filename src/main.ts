import { createApp } from 'vue';
import App from './App.vue';
// import { setupApp } from './app/setupApp';
import router from './router';
import store from './store';
import './styles/global.css';
import './styles/tailwind.css';
import { transparent } from './directives';
// 放入Main.ts中的js代码 每个窗口都会单独执行
// setupApp();

// prettier-ignore
createApp(App)
  .directive('transparent', transparent)
  .use(router)
  .use(store)
  .mount('#app');
