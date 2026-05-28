<template>
  <div class="flex flex-col gap-4 p-4">
    <SettingGroup title="启用">
      <SettingSwitchItem
        v-model="appConfigStore.enableSearch"
        icon="icon-icon-sousuofenlei"
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

          <ShortcutKeyInput
            v-model="appConfigStore.searchGlobalShortcutKey"
            :presets="['Alt + Space', 'Ctrl + Space']"
            @clear="handleClear"
            @commit="registerShortcutKey"
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
import { useAppConfig, useAppConfigActions } from '@/composables';
import { unRegisterShortcutKey } from '@/utils/shortcutKey';

const { appConfigStore } = useAppConfig();
const { registerSearchShortcutKey } = useAppConfigActions();

const shortcutKey = ref('');
watch(
  () => appConfigStore.searchGlobalShortcutKey,
  val => (shortcutKey.value = val),
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
