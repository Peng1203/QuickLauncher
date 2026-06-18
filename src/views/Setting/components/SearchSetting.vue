<template>
  <div class="flex flex-col gap-4 p-4">
    <SettingGroup :title="t('quickSearch.groupEnable')">
      <SettingSwitchItem
        v-model="appConfigStore.enableSearch"
        icon="icon-sousuo"
        :title="t('quickSearch.enableSearch')"
        :description="t('quickSearch.enableSearchDesc')"
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
        icon="icon-tabqiehuan1"
        :title="t('searchSetting.showModeTabsTitle')"
        :description="t('searchSetting.showModeTabsDesc')"
      >
        <template #body>
          <SearchModeTabs
            drag
            :size="modeTabSizeMap[appConfigStore.language]"
            :options="appConfigStore.modeOptions"
            class="p-1 rounded-lg border mt-2"
            v-model="appConfigStore.showModes"
            type="multiple"
            @change="handleToggleModl"
            @drag="handleDrag"
          />

          <div class="mt-2 flex items-center justify-end">
            <n-button type="default" size="small" @click="handleReset" class="flex-1">
              <template #icon>
                <Icon size="14" name="icon-shuaxin" class="cursor-pointer" title="重置" />
              </template>
              {{ $t("common.reset") }}
            </n-button>
          </div>
        </template>
      </SettingSwitchItem>

      <SettingSelectItem
        v-model="appConfigStore.defaultMode"
        icon="icon-buju"
        :options="modeOptions"
        :title="t('quickSearch.defaultModeTitle')"
        :description="t('quickSearch.defaultModeDesc')"
      />

      <SettingItem
        expandable
        icon="icon-kuaijiejian-"
        :title="t('quickSearch.shortcutTitle')"
        :description="t('quickSearch.shortcutDesc')"
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

const modeTabSizeMap: Record<LanguageType, "large" | "default" | "small" | "mini"> = {
  "zh-CN": "mini",
  "zh-HK": "mini",
  en: "mini",
  ja: "mini",
};

const modeOptions = computed(() =>
  MODE_TABS.filter((item) => appConfigStore.showModes.includes(item.value)).map((item) => ({
    ...item,
    label: t(`searchSetting.${item.label}` as any),
  })),
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
  const newVal = appConfigStore.modeOptions
    .map((item) => (val.includes(item.value) ? item.value : null))
    .filter((i) => i !== null);
  appConfigStore.showModes = newVal;
  if (val.includes(appConfigStore.defaultMode)) return;
  appConfigStore.defaultMode = modeOptions.value[0].value;
}

function handleDrag(val: any[]) {
  appConfigStore.modeOptions = val;
  const newVal = val.map((item) => item.value).filter((i) => appConfigStore.showModes.includes(i));
  appConfigStore.showModes = newVal;
}

function handleReset() {
  appConfigStore.modeOptions = MODE_TABS;
  appConfigStore.showModes = MODE_TABS.map((item) => item.value);
}
</script>
