import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'
import { useLoadConfig } from './composables/useLoadConfig'
import { useWindowGuards } from './composables/useWindowGuards'
import './styles/global.css'
import './styles/tailwind.css'

// 加载应用配置
useLoadConfig(store)
useWindowGuards()

createApp(App).use(store).use(router).mount('#app')
