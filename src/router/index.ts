import { createRouter, createWebHashHistory } from 'vue-router'

import Main from '@/views/Main/Main.vue'
import Search from '@/views/Search.vue'
import Setting from '@/views/Setting/Setting.vue'

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: { name: 'Main' } },
    { path: '/main', name: 'Main', component: Main },
    { path: '/search', name: 'Search', component: Search },
    { path: '/setting', name: 'Setting', component: Setting },
  ],
})

export default router
