<template>
  <div class="h-full">
    <header
      data-tauri-drag-region
      class="top-0 left-0 right-0 h-8 bg-card flex items-center justify-between px-2 z-10"
    >
      <span class="text-foreground">设 置</span>
      <!-- {{ activeTab }} -->
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
        display-directive="if"
      >
        <!-- display-directive="show:lazy" -->
        <template #tab>
          <div class="flex items-center gap-2">
            <n-icon :class="`iconfont ${tab.icon}`" />
            <span>{{ tab.label }}</span>
          </div>
        </template>
        <component :is="tab.contentComponent" />
      </n-tab-pane>
    </n-tabs>
  </div>
</template>

<script setup lang="ts">
import { CloseOutline } from '@vicons/ionicons5';
import { ref } from 'vue';
import { useAppConfig, useToggleWindowVisible } from '@/composables';
import { AppEvent } from '@/constant';
import { EventBus } from '@/utils/eventBus';
import AboutPane from './components/About.vue';
import CommandAliasPane from './components/CommandAlias.vue';
import DataPane from './components/Data.vue';
import GeneralPane from './components/General.vue';
import PortalPane from './components/Portal.vue';
import ProxyPane from './components/Proxy.vue';
import QuickSearchPane from './components/QuickSearch.vue';
import ThemePane from './components/Theme.vue';
import Translation from './components/Translation.vue';
import WebSearchPane from './components/WebSearch.vue';

const { toogleSettingWindowVisible } = useToggleWindowVisible();
const { themeColor } = useAppConfig();

const settingTabs = [
  { label: '常 规', value: 'general', icon: 'icon-changguishezhi-moren', contentComponent: GeneralPane },
  { label: '主 题', value: 'theme', icon: 'icon-zhuti_tiaosepan', contentComponent: ThemePane },
  { label: '快速搜索', value: 'q_search', icon: 'icon-icon-sousuofenlei', contentComponent: QuickSearchPane },
  { label: '网络搜索', value: 'n_search', icon: 'icon-wangluosousuo', contentComponent: WebSearchPane },
  { label: '命令别名', value: 'command_alias', icon: 'icon-minglinghangchaxun', contentComponent: CommandAliasPane },
  { label: '翻 译', value: 'translation', icon: 'icon-fanyi', contentComponent: Translation },
  { label: '代 理', value: 'network', icon: 'icon-wangluodaili', contentComponent: ProxyPane },
  { label: '数 据', value: 'data', icon: 'icon-shuju', contentComponent: DataPane },
  { label: '传送门', value: 'portal', icon: 'icon-chuansongmen', contentComponent: PortalPane },
  { label: '关 于', value: 'about', icon: 'icon-guanyu1', contentComponent: AboutPane },
];

const activeTab = ref(settingTabs[0].value);
// const activeTab = ref(settingTabs[8].value);

const handleTypeChange = (val: string) => (activeTab.value = val);

const handleClose = async () => toogleSettingWindowVisible();

// watch(appConfigStore, val => emit(AppEvent.UPDATE_APP_CONFIG_DATA, val), { deep: true })

EventBus.listen(AppEvent.OPEN_SETTING, (tabName?: string) => {
  if (tabName) activeTab.value = tabName;
  toogleSettingWindowVisible(true);
});
</script>

<style scoped lang="scss">
.n-tabs {
  height: calc(100% - 32px) !important;
  background-color: var(--background) !important;
}

.n-tab-pane {
  padding: 0 !important;
  padding-bottom: 0 !important;
  overflow-y: auto;
  padding-bottom: 10px !important;
  background-color: var(--background);
}

::v-deep(.n-tabs .n-tabs-tab.n-tabs-tab--active) {
  color: v-bind('themeColor') !important;
  background-color: var(--muted) !important;
}
</style>
