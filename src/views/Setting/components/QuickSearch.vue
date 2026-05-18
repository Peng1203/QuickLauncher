<template>
  <div class="flex flex-col gap-4 p-4">
    <SettingGroup title="启用">
      <SettingSwitchItem
        v-model="appConfigStore.enableSearch"
        icon="icon-switch"
        title="启用快速搜索"
        description="开启后可以使用快捷键快速唤起搜索窗口"
      />
    </SettingGroup>

    <SettingGroup title="窗口">
      <SettingSwitchItem
        v-model="appConfigStore.searchLostFocusHide"
        icon="icon-chuangkouzhiding"
        title="失去焦点隐藏"
        description="窗口失去焦点时自动隐藏"
      />

      <SettingSwitchItem
        v-model="appConfigStore.searchHideAfterOpen"
        icon="icon-yanjing_yincang_o"
        title="启动后隐藏"
        description="执行后自动隐藏窗口"
      />

      <SettingSwitchItem
        v-model="appConfigStore.doNotDisturbMode"
        icon="icon-wurao"
        title="勿扰模式"
        description="前台窗口处于全屏模式下不会弹出搜索窗口"
      />

      <SettingSwitchItem
        v-model="appConfigStore.searchOpenOnMouseDisplay"
        icon="icon-lcd"
        title="跟随鼠标显示"
        description="在多显示器环境下，搜索窗口跟随鼠标所在显示器弹出"
      />

      <SettingItem
        icon="icon-kuaijiejian-"
        title="全局快捷键"
        description="唤起或隐藏搜索窗口"
      >
        <div class="flex gap-1 justify-end">
          <!-- type="info" -->
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

          <n-input
            v-model:value="shortcutKey"
            style="width: 35%"
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

    <SettingGroup title="分类">
      <SettingSwitchItem
        v-model="appConfigStore.showCategory"
        icon="icon-fenlei"
        title="展示分类"
        description="在搜索结果中展示分类标签"
        @update:model-value="handleShowCategory"
      />

      <SettingSwitchItem
        v-model="appConfigStore.showSubCategory"
        icon="icon-tianjiazifenlei"
        title="展示子分类"
        description="在搜索结果中展示子分类标签"
      />
    </SettingGroup>

    <SettingGroup title="自动补全">
      <SettingSwitchItem
        v-model="appConfigStore.enableAutocomplete"
        icon="icon-zidongbuquanshurukuang"
        title="启用自动补全"
        description="输入时显示智能补全建议"
      />

      <SettingSwitchItem
        v-model="appConfigStore.enableAutocompleteFrequencyFilter"
        icon="icon-hashjinghao"
        title="高频优先"
        description="仅使用输入次数 ≥3 的记录"
      />
    </SettingGroup>

    <SettingGroup title="历史记录">
      <SettingSwitchItem
        v-model="appConfigStore.enableHistory"
        icon="icon-lishijilu_o"
        title="保存历史"
        description="记录搜索和保存历史，关闭后将不再记录"
      />

      <SettingSwitchItem
        v-model="appConfigStore.showHistory"
        icon="icon-switch"
        title="历史导航"
        description="↑ ↓ 切换历史输入"
      />
    </SettingGroup>
  </div>
</template>

<script setup lang="ts">
import type { FormValidationStatus } from 'naive-ui';
import { ref } from 'vue';
import { useAppConfig, useAppConfigActions, useNaiveUiApi } from '@/composables';

import { checkShortcutKey, checkShortcutKeyComplete, getShortcutKey, unRegisterShortcutKey } from '@/utils/shortcutKey';

const { message } = useNaiveUiApi();
const { appConfigStore } = useAppConfig();
const { registerSearchShortcutKey } = useAppConfigActions();

const shortcutKeyInputStatus = ref<FormValidationStatus>('success');
const shortcutKey = ref('');
watch(
  () => appConfigStore.searchGlobalShortcutKey,
  val => (shortcutKey.value = val),
  { immediate: true },
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

  const checkVal = await checkShortcutKey(shortcutKey.value, appConfigStore.searchGlobalShortcutKey);
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
  await unRegisterShortcutKey(appConfigStore.searchGlobalShortcutKey);
  // 注册快捷键
  await registerSearchShortcutKey(key);
  appConfigStore.searchGlobalShortcutKey = key;
}

async function registerPresetShortcutKey(key: string) {
  shortcutKey.value = key;
  await registerShortcutKey(key);
}

function handleClear() {
  handleUnRegisterShortcutKey();
  appConfigStore.searchGlobalShortcutKey = '';
}

function handleShowCategory(val: boolean) {
  console.log(`%c val ----`, 'color: #fff;background-color: #000;font-size: 18px', val);

  if (!val) appConfigStore.showSubCategory = false;
}
</script>

<style scoped>
.n-form-item {
  width: 90%;
  padding-left: 8px;
}
</style>
