<template>
  <div>
    <n-form
      ref="formRef"
      size="small"
      label-placement="left"
      :model="appConfigStore"
      :label-width="160"
      :show-feedback="false"
    >
      <h3>启动</h3>
      <n-form-item>
        <n-checkbox v-model:checked="appConfigStore.enableSearch" size="small">
          启动快速搜索
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
        />
        <!-- @input="handleChange" -->
      </n-form-item>
      <div class="mt-1 flex gap-1">
        <n-button
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
        </n-button>
      </div>

      <h3>窗口</h3>
      <n-form-item>
        <n-checkbox
          v-model:checked="appConfigStore.searchLostFocusHide"
          size="small"
        >
          失去焦点隐藏
        </n-checkbox>
      </n-form-item>

      <n-form-item>
        <n-checkbox
          v-model:checked="appConfigStore.searchHideAfterOpen"
          size="small"
        >
          Enter后隐藏
        </n-checkbox>
      </n-form-item>

      <!-- TODO -->
      <h3>历史记录</h3>
      <n-form-item>
        <n-checkbox
          v-model:checked="appConfigStore.searchLostFocusHide"
          size="small"
        >
          显示
        </n-checkbox>
        <!-- 排序条件 -->
      </n-form-item>

      <!-- TODO -->
      <h3>自动补全</h3>
      <n-form-item>
        <n-checkbox
          v-model:checked="appConfigStore.searchLostFocusHide"
          size="small"
        >
          启动
        </n-checkbox>
      </n-form-item>
    </n-form>
  </div>
</template>

<script setup lang="ts">
import type { FormValidationStatus } from 'naive-ui';
import { ref } from 'vue';
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
const { registerSearchShortcutKey } = useAppConfigActions();

const shortcutKeyInputStatus = ref<FormValidationStatus>('success');
const shortcutKey = ref('');
watch(
  () => appConfigStore.searchGlobalShortcutKey,
  val => (shortcutKey.value = val),
  { immediate: true }
);

function handleKeydown(e: KeyboardEvent) {
  const keyValue = getShortcutKey(e, appConfigStore.searchGlobalShortcutKey);

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
    appConfigStore.searchGlobalShortcutKey
  );
  if (!checkVal) {
    message.warning('快捷键被占用');
    shortcutKeyInputStatus.value = 'warning';
    return;
  }
  registerShortcutKey(shortcutKey.value);
}

async function handleUnRegisterShortcutKey() {
  await unRegisterShortcutKey(appConfigStore.searchGlobalShortcutKey);
  shortcutKeyInputStatus.value = 'success';
}

async function registerShortcutKey(key: string) {
  // 取消注册之前在的快捷键 并注册新的快捷键
  await handleUnRegisterShortcutKey();

  // 注册快捷键
  await registerSearchShortcutKey(key);
  appConfigStore.searchGlobalShortcutKey = key;
}

async function registerPresetShortcutKey(key: string) {
  shortcutKey.value = key;
  await registerShortcutKey(key);
}
</script>

<style scoped>
.n-form-item {
  width: 90%;
  padding-left: 8px;
}
</style>
