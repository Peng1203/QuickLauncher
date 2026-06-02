<template>
  <div class="flex flex-col gap-4 p-4">
    <SettingGroup :title="t('quickSearch.groupEnable')">
      <SettingSwitchItem
        v-model="appConfigStore.enableSearch"
        icon="icon-icon-sousuofenlei"
        :title="t('quickSearch.enableTitle')"
        :description="t('quickSearch.enableDesc')"
      />
    </SettingGroup>

    <SettingGroup :title="t('quickSearch.groupWindow')">
      <SettingSwitchItem
        v-model="appConfigStore.searchLostFocusHide"
        icon="icon-chuangkouzhiding"
        :title="t('quickSearch.lostFocusTitle')"
        :description="t('quickSearch.lostFocusDesc')"
      />

      <SettingSwitchItem
        v-model="appConfigStore.searchHideAfterOpen"
        icon="icon-yanjing_yincang_o"
        :title="t('quickSearch.hideAfterOpenTitle')"
        :description="t('quickSearch.hideAfterOpenDesc')"
      />

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

      <SettingSwitchItem
        v-model="appConfigStore.showSearchModeTabs"
        icon="icon-buju"
        title="展示模式标签"
        description="控制搜索窗口顶部的模式切换栏是否显示"
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

    <SettingGroup :title="t('quickSearch.groupCategory')">
      <SettingSwitchItem
        v-model="appConfigStore.showCategory"
        icon="icon-fenlei"
        :title="t('quickSearch.showCategoryTitle')"
        :description="t('quickSearch.showCategoryDesc')"
        @update:model-value="handleShowCategory"
      />

      <SettingSwitchItem
        v-model="appConfigStore.showSubCategory"
        icon="icon-tianjiazifenlei"
        :title="t('quickSearch.showSubCategoryTitle')"
        :description="t('quickSearch.showSubCategoryDesc')"
      />
    </SettingGroup>

    <SettingGroup :title="t('quickSearch.groupAutocomplete')">
      <SettingSwitchItem
        v-model="appConfigStore.enableAutocomplete"
        icon="icon-zidongbuquanshurukuang"
        :title="t('quickSearch.autocompleteTitle')"
        :description="t('quickSearch.autocompleteDesc')"
      />

      <SettingSwitchItem
        v-model="appConfigStore.enableAutocompleteFrequencyFilter"
        icon="icon-hashjinghao"
        :title="t('quickSearch.frequencyTitle')"
        :description="t('quickSearch.frequencyDesc')"
      />
    </SettingGroup>

    <SettingGroup :title="t('quickSearch.groupHistory')">
      <SettingSwitchItem
        v-model="appConfigStore.enableHistory"
        icon="icon-lishijilu_o"
        :title="t('quickSearch.historyTitle')"
        :description="t('quickSearch.historyDesc')"
      />

      <SettingSwitchItem
        v-model="appConfigStore.showHistory"
        icon="icon-switch"
        :title="t('quickSearch.historyNavTitle')"
        :description="t('quickSearch.historyNavDesc')"
      />
    </SettingGroup>
  </div>
</template>

<script setup lang="ts">
import { useAppConfig, useAppConfigActions } from "@/composables";
import { t } from "@/i18n";
import { unRegisterShortcutKey } from "@/utils/shortcutKey";

const { appConfigStore } = useAppConfig();
const { registerSearchShortcutKey } = useAppConfigActions();

const shortcutKey = ref("");
watch(
  () => appConfigStore.searchGlobalShortcutKey,
  (val) => (shortcutKey.value = val),
  { immediate: true },
);

async function handleUnRegisterShortcutKey() {
  await unRegisterShortcutKey(appConfigStore.searchGlobalShortcutKey);
}

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

function handleShowCategory(val: boolean) {
  console.log(`%c val ----`, "color: #fff;background-color: #000;font-size: 18px", val);

  if (!val) appConfigStore.showSubCategory = false;
}
</script>

<style scoped>
.n-form-item {
  width: 90%;
  padding-left: 8px;
}
</style>
