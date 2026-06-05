<template>
  <div class="flex flex-col gap-4 p-4">
    <SettingGroup :title="t('general.groupSystem')">
      <SettingSwitchItem
        v-model="appConfigStore.autoStart"
        icon="icon-switch"
        :title="t('general.autoStartTitle')"
        :description="t('general.autoStartDesc')"
        @update:model-value="setAutoStart"
      />

      <SettingSwitchItem
        v-model="appConfigStore.silentStart"
        icon="icon-wurao"
        :title="t('general.silentStartTitle')"
        :description="t('general.silentStartDesc')"
      />
    </SettingGroup>

    <SettingGroup :title="t('general.groupLanguage')">
      <!-- <SettingSelectItem
        v-model="appConfigStore.language"
        title="界面语言"
        description="选择应用显示语言"
        :options="languageOptions"
      /> -->

      <SettingItem :title="t('general.languageTitle')" :description="t('general.languageDesc')">
        <ToggleGroup v-model="appConfigStore.language" size="sm" :options="languageOptions" />
      </SettingItem>
    </SettingGroup>

    <SettingGroup :title="t('general.groupWindow')">
      <SettingSwitchItem
        v-model="appConfigStore.onTop"
        icon="icon-chuangkouzhiding"
        :title="t('general.pinTitle')"
        :description="t('general.pinDesc')"
        @update:model-value="setAlwaysOnTop"
      />

      <SettingSwitchItem
        v-model="appConfigStore.center"
        icon="icon-juzhongxianshi"
        :title="t('general.centerTitle')"
        :description="t('general.centerDesc')"
        @update:model-value="setMainWindowCenter"
      />

      <SettingItem
        expandable
        icon="icon-kuaijiejian-"
        :title="t('general.shortcutKeyTitle')"
        :description="t('general.shortcutKeyDesc')"
      >
        <template #expand>
          <SettingItem v-for="preItem of ['Alt + P', 'Alt + M']" :key="preItem">
            <template #title>
              <n-button type="info" size="tiny" @click="registerShortcutKey(preItem)">
                {{ $t("common.usePreset") }}
              </n-button>
            </template>
            <ShortcutKbd :value="preItem" />
          </SettingItem>
        </template>

        <!-- :presets="['Alt + P', 'Alt + M']" -->
        <ShortcutKeyInput
          v-model="appConfigStore.mainWindowGlobalShortcutKey"
          @clear="handleClear"
          @commit="registerShortcutKey"
        />
      </SettingItem>
    </SettingGroup>

    <SettingGroup :title="t('general.groupOperation')">
      <SettingSwitchItem
        v-model="appConfigStore.confirmBeforeDelete"
        icon="icon-shanchu"
        :title="t('general.confirmDeleteTitle')"
        :description="t('general.confirmDeleteDesc')"
      />
    </SettingGroup>
  </div>
</template>

<script setup lang="ts">
import { useAppConfig, useAppConfigActions } from "@/composables";
import { t } from "@/i18n";
import { unRegisterShortcutKey } from "@/utils/shortcutKey";

const { appConfigStore } = useAppConfig();
const {
  setAlwaysOnTop, //
  setMainWindowCenter,
  setAutoStart,
  registerMainWindowShortcutKey,
} = useAppConfigActions();

const languageOptions: OptionItem<LanguageType>[] = [
  { label: "简体中文", value: "zh-CN" },
  { label: "繁體中文", value: "zh-HK" },
  { label: "English", value: "en" },
  { label: "日本語", value: "ja" },
];

const shortcutKey = ref("");
watch(
  () => appConfigStore.mainWindowGlobalShortcutKey,
  (val) => (shortcutKey.value = val),
  { immediate: true },
);

async function handleUnRegisterShortcutKey() {
  await unRegisterShortcutKey(appConfigStore.mainWindowGlobalShortcutKey);
}

async function registerShortcutKey(key: string) {
  // 取消注册之前在的快捷键 并注册新的快捷键
  await handleUnRegisterShortcutKey();

  // 注册快捷键
  await registerMainWindowShortcutKey(key);
  appConfigStore.mainWindowGlobalShortcutKey = key;
}

function handleClear() {
  handleUnRegisterShortcutKey();
  appConfigStore.mainWindowGlobalShortcutKey = "";
}
</script>
