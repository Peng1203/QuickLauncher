import { createRouter, createWebHashHistory } from 'vue-router'

import Main from '@/views/Main.vue'
import Search from '@/views/Search.vue'

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: { name: 'Main' } },
    { path: '/main', name: 'Main', component: Main },
    { path: '/search', name: 'Search', component: Search },
  ],
})

export default router
