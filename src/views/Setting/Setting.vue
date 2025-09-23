<template>
  <div class="h-full">
    <header
      data-tauri-drag-region
      class="top-0 left-0 right-0 h-8 bg-white flex items-center justify-between px-2 z-10"
    >
      <span class="text-gray-700">设 置</span>

      <!-- {{ appConfigStore.center }} -->

      <n-icon
        size="25"
        class="cursor-pointer"
        @click="handleClose"
      >
        <CloseOutline />
      </n-icon>
    </header>

    <n-tabs
      animated
      type="line"
      placement="left"
      size="medium"
      v-model:value="activeTab"
      :default-value="activeTab"
      :on-update:value="handleTypeChange"
    >
      <n-tab-pane
        :tab="tab.label"
        :name="tab.value"
        v-for="tab in settingTabs"
      >
        <component :is="tab.contentComponent" />
      </n-tab-pane>
    </n-tabs>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { CloseOutline } from '@vicons/ionicons5'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { useAppConfig } from '@/composables/useAppConfig'
import GeneralPane from './components/General.vue'
import { useUpdateAppConfig } from '@/composables/useUpdateAppConfig'

useUpdateAppConfig()
const { appConfigStore } = useAppConfig()

const cuurrentWindow = getCurrentWebviewWindow()

const settingTabs = [
  { label: '常 规', value: 'general', contentComponent: GeneralPane },
  { label: '快速搜索', value: 'q_search', contentComponent: GeneralPane },
  { label: '网络搜索', value: 'n_search', contentComponent: GeneralPane },
  { label: '网 络', value: 'network', contentComponent: GeneralPane },
]

const activeTab = ref(settingTabs[0].value)

const handleTypeChange = (val: string) => (activeTab.value = val)

const handleClose = async () => cuurrentWindow?.hide()

// watch(appConfigStore, val => emit(AppEvent.UPDATE_APP_CONFIG_DATA, val), { deep: true })
</script>

<style scoped>
.n-tabs {
  height: calc(100% - 32px) !important;
}
</style>
