<template>
  <n-layout
    class="w-full h-full flex flex-col"
    position="absolute"
    :native-scrollbar="false"
  >
    <Header />

    <n-layout
      has-sider
      class="container-layout flex-1 overflow-hidden"
    >
      <Sidebar />

      <LaunchList />
      {{ store.categoryOptions.length }}
    </n-layout>

    <Footer />
  </n-layout>
</template>

<script setup lang="ts">
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { ref } from 'vue';
import { ensureDefaultCategory } from '@/api';
import { useAppConfig } from '@/composables/useAppConfig';
import { useAppConfigActions } from '@/composables/useAppConfigActions';
import { useCategoryCorrelationDir } from '@/composables/useCategoryCorrelationDir';
import { useLoadConfig } from '@/composables/useLoadConfig';
import { AppEvent } from '@/constant';
import { useStore } from '@/store/useStore';
import { EventBus } from '@/utils/eventBus';
import LaunchList from './components/LaunchList.vue';
import Sidebar from './components/Sidebar.vue';

const store = useStore();

const { appConfigStore } = useAppConfig();

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
EventBus.listen(AppEvent.UPDATE_CATEGORY_LIST, store.getCategoryData);

useLoadConfig();

useAppConfigActions().initMainWindowShortcutKey();

const { registerAllCategoryDirWatch } = useCategoryCorrelationDir();

function delay(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function initCategoryData() {
  try {
    await store.getCategoryData(true);
    registerAllCategoryDirWatch();
  } catch (e) {
    console.log('initCategoryData', e);
  }
}

async function initLaunchData() {
  try {
    await store.getLaunchData();
  } catch (e) {
    console.log('initLaunchData', e);
  }
}

const retryCounter = ref(0);
async function initData() {
  try {
    if (retryCounter.value >= 20) return;
    await delay(50);
    await ensureDefaultCategory().then(initCategoryData).then(initLaunchData);
  } catch (e) {
    console.log('initData', e);
    retryCounter.value++;
    await initData();
  }
}

initData();
</script>

<style scoped lang="scss">
.container-layout {
  height: calc(600px - 24px - 32px);
  ::v-deep(.n-layout-scroll-container:has(> aside)) {
    overflow-y: hidden !important;
  }
}
</style>
