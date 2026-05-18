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
      <SettingItem
        title="界面语言"
        description="选择应用显示语言"
      >
        <n-select
          v-model:value="appConfigStore.language"
          :consistent-menu-width="false"
          size="small"
          placeholder="Select"
          :options="languageOptions as any"
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
          <!-- type="info" -->
          <n-button
            type="info"
            size="tiny"
            @click="registerPresetShortcutKey('Alt + P')"
          >
            Alt + P
          </n-button>
          <n-button
            type="info"
            size="tiny"
            @click="registerPresetShortcutKey('Alt + M')"
          >
            Alt + M
          </n-button>

          <n-input
            v-model:value="shortcutKey"
            style="width: 45%"
            class="w-1/5"
            size="tiny"
            readonly
            clearable
            type="text"
            placeholder=""
            :status="shortcutKeyInputStatus"
            @keydown="handleKeydown"
            @blur="handleBlur"
            @focus="shortcutKeyInputStatus = 'success'"
            @clear="handleClear"
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
import type { FormValidationStatus } from 'naive-ui';
import { useAppConfig, useAppConfigActions, useNaiveUiApi } from '@/composables';
import { checkShortcutKey, checkShortcutKeyComplete, getShortcutKey, unRegisterShortcutKey } from '@/utils/shortcutKey';

const { message } = useNaiveUiApi();

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

const shortcutKeyInputStatus = ref<FormValidationStatus>('success');
const shortcutKey = ref('');
watch(
  () => appConfigStore.mainWindowGlobalShortcutKey,
  val => (shortcutKey.value = val),
  { immediate: true },
);
function handleKeydown(e: KeyboardEvent) {
  const target = e.target as HTMLInputElement | null;
  if (!target) return;

  // 👉 判断是否有文本被选中
  const hasSelection =
    target.selectionStart !== null && target.selectionEnd !== null && target.selectionStart !== target.selectionEnd;

  if (hasSelection) {
    return;
  }

  e.preventDefault();

  const keyValue = getShortcutKey(e, appConfigStore.mainWindowGlobalShortcutKey);

  shortcutKey.value = keyValue;
}

async function handleBlur() {
  // 清空快捷键值 则进行取消绑定的操作
  if (!shortcutKey.value) return handleUnRegisterShortcutKey();

  const isComplete = checkShortcutKeyComplete(shortcutKey.value);
  if (!isComplete) {
    message.error('快捷键输入不完整');
    shortcutKeyInputStatus.value = 'error';
    return;
  }

  const checkVal = await checkShortcutKey(shortcutKey.value, appConfigStore.mainWindowGlobalShortcutKey);
  if (!checkVal) {
    message.warning('快捷键被占用');
    shortcutKeyInputStatus.value = 'warning';
    return;
  }
  registerShortcutKey(shortcutKey.value);
}

async function handleUnRegisterShortcutKey() {
  await unRegisterShortcutKey(appConfigStore.mainWindowGlobalShortcutKey);
  shortcutKeyInputStatus.value = 'success';
}

async function registerShortcutKey(key: string) {
  // 取消注册之前在的快捷键 并注册新的快捷键
  await handleUnRegisterShortcutKey();

  // 注册快捷键
  await registerMainWindowShortcutKey(key);
  appConfigStore.mainWindowGlobalShortcutKey = key;
}

async function registerPresetShortcutKey(key: string) {
  shortcutKey.value = key;
  await registerShortcutKey(key);
}

function handleClear() {
  handleUnRegisterShortcutKey();
  appConfigStore.mainWindowGlobalShortcutKey = '';
}
</script>
