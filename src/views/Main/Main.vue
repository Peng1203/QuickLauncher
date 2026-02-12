<template>
  <n-layout class="w-full h-full">
    <Header />

    <n-layout
      has-sider
      style="height: calc(600px - 32px)"
      class="container-layout"
    >
      <Sidebar />

      <LaunchList :category-modal-status="categoryModalStatus" />
    </n-layout>

    <OperationCategoryModal v-model="categoryModalStatus" />
  </n-layout>
</template>

<script setup lang="ts">
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { ref } from 'vue';
import { useAppConfig } from '@/composables/useAppConfig';
import { useAppConfigActions } from '@/composables/useAppConfigActions';

import { useLoadConfig } from '@/composables/useLoadConfig';
import { AppEvent } from '@/constant';
import { useStore } from '@/store/useStore';
import { EventBus } from '@/utils/eventBus';
import LaunchList from './components/LaunchList.vue';
import Sidebar from './components/Sidebar.vue';

const store = useStore();

const { appConfigStore } = useAppConfig();

const categoryModalStatus = ref(false);

const currentWindow = getCurrentWebviewWindow();

let timer: any;
currentWindow.onMoved(({ payload: position }) => {
  clearTimeout(timer);
  timer = setTimeout(() => {
    appConfigStore.mainWindowPositionX = position.x;
    appConfigStore.mainWindowPositionY = position.y;
  }, 100);
});

EventBus.listen(AppEvent.UPDATE_LAUNCH_LIST, store.getLaunchData);

useLoadConfig();

useAppConfigActions().initMainWindowShortcutKey();

// watchImmediate(
//   'C:\\Users\\Mayn\\Desktop\\FTTH APP截图',
//   event => {
//     console.log(`%c event ----`, 'color: #fff;background-color: #000;font-size: 18px', event)
//   },
//   { baseDir: BaseDirectory.Desktop }
// )

// listen<AppConfigState>(AppEvent.UPDATE_APP_CONFIG_DATA, val => {
//   for (const key in val.payload) {
//     // @ts-ignore
//     appConfigStore[key] = val.payload[key]
//   }
// })

// const transparentDragWindow = ref<WebviewWindow | null>(null);
// const launchListContainerRef = useTemplateRef('launchListContainerRef');
// function showTransparentDragWindow() {
//   transparentDragWindow.value?.show();
//   transparentDragWindow.value?.setPosition(
//     new LogicalPosition(
//       appConfigStore.mainWindowPositionX + 192 + 12,
//       appConfigStore.mainWindowPositionY + 32 + 4
//     )
//   );
// }
</script>

<style scoped lang="scss">
.container-layout {
  ::v-deep(.n-layout-scroll-container:has(> aside)) {
    overflow-y: hidden !important;
  }
}
</style>
