<template>
  <div class="flex flex-col gap-4 p-4">
    <SettingGroup title="系统">
      <SettingSwitchItem
        v-model="appConfigStore.autoStart"
        icon="icon-switch"
        title="开机自启"
        description="系统启动时自动运行应用"
        @update:model-value="setAutoStart"
      />

      <SettingSwitchItem
        v-model="appConfigStore.silentStart"
        icon="icon-wurao"
        title="静默启动"
        description="启动时最小化到系统托盘"
      />
    </SettingGroup>

    <SettingGroup title="语言">
      <!-- <SettingSelectItem
        v-model="appConfigStore.language"
        title="界面语言"
        description="选择应用显示语言"
        :options="languageOptions"
      /> -->

      <SettingItem
        title="界面语言"
        description="选择应用显示语言"
      >
        <ToggleGroup
          v-model="appConfigStore.language"
          size="sm"
          :options="languageOptions"
        />
      </SettingItem>
    </SettingGroup>

    <SettingGroup title="窗口">
      <SettingSwitchItem
        v-model="appConfigStore.onTop"
        icon="icon-chuangkouzhiding"
        title="窗口置顶"
        description="是否将应用窗口置顶显示"
        @update:model-value="setAlwaysOnTop"
      />

      <SettingSwitchItem
        v-model="appConfigStore.center"
        icon="icon-juzhongxianshi"
        title="居中显示"
        description="启动时窗口居中显示"
        @update:model-value="setMainWindowCenter"
      />

      <SettingItem
        icon="icon-kuaijiejian-"
        title="全局快捷键"
        description="快速唤起或隐藏主窗口"
      >
        <div class="flex gap-1 justify-end">
          <ShortcutKeyInput
            v-model="appConfigStore.mainWindowGlobalShortcutKey"
            :presets="['Alt + P', 'Alt + M']"
            @clear="handleClear"
            @commit="registerShortcutKey"
          />
        </div>
      </SettingItem>
    </SettingGroup>

    <SettingGroup title="操作">
      <SettingSwitchItem
        v-model="appConfigStore.confirmBeforeDelete"
        icon="icon-shanchufenlei"
        title="删除二次确认"
        description="删除启动项前显示确认对话框"
      />
    </SettingGroup>
  </div>
</template>

<script setup lang="ts">
import { useAppConfig, useAppConfigActions } from '@/composables';
import { unRegisterShortcutKey } from '@/utils/shortcutKey';

const { appConfigStore } = useAppConfig();
const {
  setAlwaysOnTop, //
  setMainWindowCenter,
  setAutoStart,
  registerMainWindowShortcutKey,
} = useAppConfigActions();

const languageOptions: OptionItem<LanguageType>[] = [
  { label: '简体中文', value: 'zh-CN' },
  { label: '繁體中文', value: 'zh-HK' },
  { label: 'English', value: 'en' },
  { label: '日本語', value: 'ja' },
];

const shortcutKey = ref('');
watch(
  () => appConfigStore.mainWindowGlobalShortcutKey,
  val => (shortcutKey.value = val),
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
  appConfigStore.mainWindowGlobalShortcutKey = '';
}
</script>
