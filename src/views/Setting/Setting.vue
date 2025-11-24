<template>
  <div class="h-full">
    <header
      data-tauri-drag-region
      class="top-0 left-0 right-0 h-8 bg-white flex items-center justify-between px-2 z-10"
    >
      <span class="text-gray-700">设 置</span>
      <!-- {{ activeTab }} -->
      <!-- {{ appConfigStore.center }} -->

      <n-icon size="25" class="cursor-pointer" @click="handleClose">
        <CloseOutline />
      </n-icon>
    </header>

    <n-tabs
      v-model:value="activeTab"
      animated
      type="line"
      size="medium"
      placement="left"
      :default-value="activeTab"
      :on-update="handleTypeChange"
    >
      <n-tab-pane
        v-for="(tab, i) in settingTabs"
        :key="i"
        :tab="tab.label"
        :name="tab.value"
      >
        <component :is="tab.contentComponent" />
      </n-tab-pane>
    </n-tabs>
  </div>
</template>

<script setup lang="ts">
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { CloseOutline } from '@vicons/ionicons5';
import { ref } from 'vue';
import { useAppConfig } from '@/composables/useAppConfig';
import GeneralPane from './components/General.vue';
import NetworkPane from './components/Network.vue';
import QuickSearchPane from './components/QuickSearch.vue';
import Translation from './components/Translation.vue';
import WebSearchPane from './components/WebSearch.vue';

const { appConfigStore } = useAppConfig();
const currentWindow = getCurrentWebviewWindow();

const settingTabs = [
  { label: '常 规', value: 'general', contentComponent: GeneralPane },
  { label: '快速搜索', value: 'q_search', contentComponent: QuickSearchPane },
  { label: '网络搜索', value: 'n_search', contentComponent: WebSearchPane },
  { label: '翻 译', value: 'translation', contentComponent: Translation },
  { label: '网 络', value: 'network', contentComponent: NetworkPane },
];

const activeTab = ref(settingTabs[3].value);

const handleTypeChange = (val: string) => (activeTab.value = val);

const handleClose = async () => currentWindow?.hide();

// watch(appConfigStore, val => emit(AppEvent.UPDATE_APP_CONFIG_DATA, val), { deep: true })

let timer: any;
currentWindow.onMoved(({ payload: position }) => {
  clearTimeout(timer);
  timer = setTimeout(() => {
    appConfigStore.settingWindowPositionX = position.x;
    appConfigStore.settingWindowPositionY = position.y;
  }, 100);
});
</script>

<style scoped>
.n-tabs {
  height: calc(100% - 32px) !important;
}

.n-tab-pane {
  overflow-y: auto;
  padding-bottom: 10px !important;
}
</style>
