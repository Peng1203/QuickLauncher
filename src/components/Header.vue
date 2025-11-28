<template>
  <n-layout-header
    :data-tauri-drag-region="!appConfigStore.center"
    class="h-8 bg-white flex items-center justify-between px-2 border-b-1 border-gray-200 z-10"
  >
    <span class="text-gray-700">Quick Launcher</span>
    <!-- 右侧操作 -->
    <div class="flex items-center gap-2">
      <n-icon
        title="设置"
        size="20"
        class="cursor-pointer"
        @click="handleToggleSettingWindowVisible"
      >
        <SettingsOutline />
      </n-icon>

      <n-dropdown
        placement="bottom-start"
        trigger="click"
        size="small"
        :options="options"
      >
        <!-- @select="" -->
        <n-icon size="25" class="cursor-pointer">
          <MenuOutline />
        </n-icon>
      </n-dropdown>

      <n-icon
        title="关闭窗口"
        size="25"
        class="cursor-pointer"
        @click="handleClose"
      >
        <CloseOutline />
      </n-icon>
    </div>
  </n-layout-header>
</template>

<script setup lang="tsx">
import type { DropdownMixedOption } from 'naive-ui/es/dropdown/src/interface';
import { LogicalPosition } from '@tauri-apps/api/dpi';
import { listen } from '@tauri-apps/api/event';
import {
  getCurrentWebviewWindow,
  WebviewWindow,
} from '@tauri-apps/api/webviewWindow';
import { CloseOutline, MenuOutline, SettingsOutline } from '@vicons/ionicons5';
import { useAppConfig } from '@/composables/useAppConfig';
import { useAppConfigActions } from '@/composables/useAppConfigActions';
import { AppEvent } from '@/constant';

const { appConfigStore, settingWindowPositionX, settingWindowPositionY } =
  useAppConfig();
const { setAlwaysOnTop, setMainWindowCenter, setAutoStart } =
  useAppConfigActions();

const cuurrentWindow = getCurrentWebviewWindow();

// TODO 通过个性化配置设置 为隐藏还是关闭
const handleClose = async () => cuurrentWindow?.hide();

const options: DropdownMixedOption[] = [
  {
    key: 'onTop',
    label: '窗口置顶',
    type: 'render',
    render: () =>
      h(
        // v-model:
        // checked={appConfigStore.onTop}
        /* onChange={setAlwaysOnTop} */
        <n-checkbox
          size="small"
          class="mx-2"
          label="窗口置顶"
          default-checked={appConfigStore.onTop}
          v-model:checked={appConfigStore.onTop}
          onUpdate-checked={setAlwaysOnTop}
        />
      ),
  },
  {
    key: 'center',
    label: '居中显示',
    type: 'render',
    render: () =>
      h(
        <n-checkbox
          size="small"
          class="mx-2"
          label="居中显示"
          default-checked={appConfigStore.center}
          v-model:checked={appConfigStore.center}
          onUpdate-checked={setMainWindowCenter}
        />
      ),
  },
  {
    key: 'silentStart',
    label: '静默启动',
    type: 'render',
    render: () =>
      h(
        <n-checkbox
          size="small"
          class="mx-2"
          label="静默启动"
          default-checked={appConfigStore.silentStart}
          v-model:checked={appConfigStore.silentStart}
        />
      ),
  },
  {
    key: 'autoStart',
    label: '开机自启',
    type: 'render',
    render: () =>
      h(
        <n-checkbox
          size="small"
          class="mx-2"
          label="开机自启"
          default-checked={appConfigStore.autoStart}
          v-model:checked={appConfigStore.autoStart}
          onUpdate-checked={setAutoStart}
        />
      ),
  },

  // {
  //   key: 'about',
  //   label: '关于',
  // },
];

async function handleToggleSettingWindowVisible() {
  // 获取 setting 窗口
  const settingWindow = await WebviewWindow.getByLabel('setting');

  const visbile = await settingWindow?.isVisible();
  const focus = await settingWindow?.isFocused();

  if (visbile && !focus) {
    settingWindow?.setFocus();
  } else if (visbile && focus) {
    settingWindow?.hide();
  } else {
    const x =
      settingWindowPositionX.value > 0
        ? settingWindowPositionX.value
        : appConfigStore.mainWindowPositionX + 100;
    const y =
      settingWindowPositionY.value > 0
        ? settingWindowPositionY.value
        : appConfigStore.mainWindowPositionY + 50;

    // 将设置窗口置于 当前主窗口之间展示
    settingWindow?.setPosition(new LogicalPosition(x, y));
    await settingWindow?.setAlwaysOnTop(appConfigStore.onTop);
    settingWindow?.show();
    settingWindow?.setFocus();
  }
}

listen<AppConfigState>(AppEvent.UPDATE_APP_CONFIG_DATA, val => {
  for (const key in val.payload) {
    // @ts-expect-error
    appConfigStore[key] = val.payload[key];
  }
});
</script>
