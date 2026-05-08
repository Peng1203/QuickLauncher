import { createRouter, createWebHashHistory } from 'vue-router';

import Main from '@/views/Main/Main.vue';
import OperationCategory from '@/views/OperationCategory/OperationCategory.vue';
import OperationLaunch from '@/views/OperationLaunch/OperationLaunch.vue';
import Search from '@/views/Search/Search.vue';
import Setting from '@/views/Setting/Setting.vue';

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: { name: 'Main' } },
    { path: '/main', name: 'Main', component: Main },
    { path: '/search', name: 'Search', component: Search },
    { path: '/setting', name: 'Setting', component: Setting },
    { path: '/operLaunch', name: 'OperLaunch', component: OperationLaunch },
    { path: '/operCategory', name: 'OperCategory', component: OperationCategory },
  ],
});

export default router;
