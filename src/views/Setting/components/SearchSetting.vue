<template>
  <div class="flex flex-col gap-4 p-4">
    <SettingGroup :title="t('quickSearch.groupEnable')">
      <SettingSwitchItem
        v-model="appConfigStore.enableSearch"
        icon="icon-sousuo"
        title="启用搜索"
        description="开启后可以使用快捷键快速唤起搜索窗口"
      />
    </SettingGroup>

    <SettingGroup :title="t('quickSearch.groupWindow')">
      <SettingSwitchItem
        v-model="appConfigStore.searchLostFocusHide"
        icon="icon-chuangkouzhiding"
        :title="t('quickSearch.lostFocusTitle')"
        :description="t('quickSearch.lostFocusDesc')"
      />

      <!-- <SettingSwitchItem
        v-model="appConfigStore.searchHideAfterOpen"
        icon="icon-yanjing_yincang_o"
        :title="t('quickSearch.hideAfterOpenTitle')"
        :description="t('quickSearch.hideAfterOpenDesc')"
      /> -->

      <SettingSwitchItem
        v-model="appConfigStore.doNotDisturbMode"
        icon="icon-wurao"
        :title="t('quickSearch.dndTitle')"
        :description="t('quickSearch.dndDesc')"
      />

      <SettingSwitchItem
        v-model="appConfigStore.searchOpenOnMouseDisplay"
        icon="icon-lcd"
        :title="t('quickSearch.followMouseTitle')"
        :description="t('quickSearch.followMouseDesc')"
      />

      <SettingItem
        expandable
        icon="icon-kuaijiejian-"
        :title="t('general.shortcutKeyTitle')"
        :description="t('quickSearch.shortcutKeyDesc')"
      >
        <template #expand>
          <SettingItem v-for="preItem of ['Alt + Space', 'Ctrl + Space']" :key="preItem">
            <template #title>
              <n-button type="info" size="tiny" @click="registerShortcutKey(preItem)">
                {{ $t("common.usePreset") }}
              </n-button>
            </template>
            <ShortcutKbd :value="preItem" />
          </SettingItem>
        </template>
        <div class="flex gap-1 justify-end">
          <!-- type="info" -->
          <!-- <n-button
            type="info"
            size="tiny"
            @click="registerPresetShortcutKey('Alt + Space')"
          >
            Alt + Space
          </n-button>
          <n-button
            type="info"
            size="tiny"
            @click="registerPresetShortcutKey('Ctrl + Space')"
          >
            Ctrl + Space
          </n-button> -->

          <!-- :presets="['Alt + Space', 'Ctrl + Space']" -->
          <ShortcutKeyInput
            v-model="appConfigStore.searchGlobalShortcutKey"
            @clear="handleClear"
            @commit="registerShortcutKey"
          />
        </div>
      </SettingItem>
    </SettingGroup>

    <SettingGroup :title="t('searchSetting.groupMode')">
      <SettingSwitchItem
        v-model="appConfigStore.showSearchModeTabs"
        icon="icon-buju"
        :title="t('searchSetting.showModeTabsTitle')"
        :description="t('searchSetting.showModeTabsDesc')"
      >
        <template #body>
          <!-- {{ appConfigStore.showModes }} -->
          <SearchModeTabs
            class="p-1 rounded-lg border mt-2"
            v-model="appConfigStore.showModes"
            type="multiple"
            @change="handleToggleModl"
          />
        </template>
      </SettingSwitchItem>

      <SettingSelectItem
        v-model="appConfigStore.defaultMode"
        :options="modeOptions"
        icon="icon-buju"
        title="默认模式"
        description="呼出搜索窗口默认选中的模式"
      />

      <SettingItem
        expandable
        icon="icon-kuaijiejian-"
        title="快捷键"
        description="使用快捷键快速切换模式"
      >
        <ShortcutKeyInput
          v-model="appConfigStore.switchModeShortcutKey"
          @clear="appConfigStore.switchModeShortcutKey = ''"
          @commit="(val) => (appConfigStore.switchModeShortcutKey = val)"
        />

        <template #expand>
          <SettingItem v-for="preItem of ['Ctrl + Tab', 'Shift + Tab']" :key="preItem">
            <template #title>
              <n-button
                type="info"
                size="tiny"
                @click="appConfigStore.switchModeShortcutKey = preItem"
              >
                {{ $t("common.usePreset") }}
              </n-button>
            </template>
            <ShortcutKbd :value="preItem" />
          </SettingItem>
        </template>
      </SettingItem>
    </SettingGroup>
  </div>
</template>

<script setup lang="ts">
import { useAppConfig, useAppConfigActions } from "@/composables";
import { unRegisterShortcutKey } from "@/utils/shortcutKey";
import { t } from "@/i18n";
import { MODE_TABS } from "@/constant";

const { registerSearchShortcutKey } = useAppConfigActions();

const { appConfigStore } = useAppConfig();

const modeOptions = computed(() =>
  MODE_TABS.filter((item) => appConfigStore.showModes.includes(item.value)),
);

async function registerShortcutKey(key: string) {
  // 取消注册之前在的快捷键 并注册新的快捷键
  await unRegisterShortcutKey(appConfigStore.searchGlobalShortcutKey);
  // 注册快捷键
  await registerSearchShortcutKey(key);
  appConfigStore.searchGlobalShortcutKey = key;
}

function handleClear() {
  handleUnRegisterShortcutKey();
  appConfigStore.searchGlobalShortcutKey = "";
}

async function handleUnRegisterShortcutKey() {
  await unRegisterShortcutKey(appConfigStore.searchGlobalShortcutKey);
}

function handleToggleModl(val: SearchMode[]) {
  if (val.includes(appConfigStore.defaultMode)) return;
  appConfigStore.defaultMode = modeOptions.value[0].value;
}
</script>

<style scoped></style>
