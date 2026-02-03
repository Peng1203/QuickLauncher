<template>
  <n-form
    ref="formRef"
    size="small"
    label-placement="left"
    :model="appConfigStore"
    :label-width="160"
    :show-feedback="false"
  >
    <h3 class="!mt-[0]">系统</h3>
    <n-form-item>
      <n-checkbox
        v-model:checked="appConfigStore.autoStart"
        size="small"
        @update-checked="setAutoStart"
      >
        开机自启
      </n-checkbox>
    </n-form-item>

    <n-form-item>
      <n-checkbox v-model:checked="appConfigStore.silentStart">
        静默启动
      </n-checkbox>
    </n-form-item>

    <!-- <n-form-item>
      <n-checkbox v-model:checked="appConfigStore.autoStart"> 开机自启 </n-checkbox>
    </n-form-item> -->

    <h3>语言</h3>
    <n-form-item class="mt-1">
      <n-select
        v-model:value="appConfigStore.language"
        size="small"
        placeholder="Select"
        :options="languageOptions as any"
      />
    </n-form-item>

    <h3>窗口</h3>
    <n-form-item>
      <n-checkbox
        v-model:checked="appConfigStore.onTop"
        @update-checked="setAlwaysOnTop"
      >
        窗口置顶
      </n-checkbox>
    </n-form-item>
    <n-form-item>
      <n-checkbox
        v-model:checked="appConfigStore.center"
        @update-checked="setMainWindowCenter"
      >
        居中显示
      </n-checkbox>
    </n-form-item>

    <h3>显示/隐藏</h3>
    <n-form-item label="快捷键" label-width="auto">
      <n-input
        v-model:value="shortcutKey"
        readonly
        clearable
        type="text"
        placeholder=""
        :status="shortcutKeyInputStatus"
        @keydown.prevent="handleKeydown"
        @blur="handleBlur"
        @focus="shortcutKeyInputStatus = 'success'"
        @clear="handleClear"
      />
      <!-- @input="handleChange" -->
    </n-form-item>

    <div class="mt-1 flex gap-1">
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
    </div>

    <h3>操作</h3>
    <n-form-item>
      <n-checkbox
        v-model:checked="appConfigStore.confirmBeforeDelete"
        size="small"
      >
        删除启动项二次确认
      </n-checkbox>
    </n-form-item>

    <!-- TODO 勿扰模式 -->
  </n-form>
</template>

<script setup lang="ts">
import type { FormValidationStatus } from 'naive-ui';
import { useAppConfig } from '@/composables/useAppConfig';
import { useAppConfigActions } from '@/composables/useAppConfigActions';
import { useNaiveUiApi } from '@/composables/useNaiveUiApi';
import {
  checkShortcutKey,
  checkShortcutKeyComplete,
  getShortcutKey,
  unRegisterShortcutKey,
} from '@/utils/shortcutKey';

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
  const keyValue = getShortcutKey(
    e,
    appConfigStore.mainWindowGlobalShortcutKey,
  );

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

  const checkVal = await checkShortcutKey(
    shortcutKey.value,
    appConfigStore.mainWindowGlobalShortcutKey,
  );
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

<style scoped>
.n-form-item {
  width: 90%;
  padding-left: 8px;
}
</style>
