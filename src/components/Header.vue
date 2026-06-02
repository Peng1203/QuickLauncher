<template>
  <!-- position="absolute" -->
  <n-layout-header
    :data-tauri-drag-region="!appConfigStore.center"
    class="h-8 bg-card flex items-center justify-between px-2 border-b-1 border-border z-10"
  >
    <!-- text-foreground -->
    <span class="color-[inherit]">{{ appConfigStore.title }}</span>
    <!-- 右侧操作 -->
    <div class="flex items-center gap-2">
      <ThemeSwitch />

      <n-icon
        :title="t('main.settings')"
        size="20"
        class="cursor-pointer"
        @click="toogleSettingWindowVisible"
      >
        <SettingsOutline />
      </n-icon>

      <n-dropdown placement="bottom-start" trigger="click" size="small" :options="options">
        <!-- @select="" -->
        <n-icon size="25" class="cursor-pointer">
          <MenuOutline />
        </n-icon>
      </n-dropdown>

      <n-icon :title="t('main.closeWindow')" size="25" class="cursor-pointer" @click="handleClose">
        <CloseOutline />
      </n-icon>
    </div>
  </n-layout-header>
</template>

<script setup lang="tsx">
import type { DropdownMixedOption } from 'naive-ui/es/dropdown/src/interface';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { CloseOutline, MenuOutline, SettingsOutline } from '@vicons/ionicons5';
import { useAppConfig, useAppConfigActions, useToggleWindowVisible } from '@/composables';
import { AppEvent } from '@/constant';
import { t } from '@/i18n';

const { appConfigStore } = useAppConfig();
const { setAlwaysOnTop, setMainWindowCenter, setAutoStart } = useAppConfigActions();
const { toogleSettingWindowVisible } = useToggleWindowVisible();
const cuurrentWindow = getCurrentWebviewWindow();

// TODO 通过个性化配置设置 为隐藏还是关闭
const handleClose = async () => cuurrentWindow?.hide();

const options: DropdownMixedOption[] = [
  {
    key: 'onTop',
    label: t('common.windowPin'),
    type: 'render',
    render: () =>
      h(
        // v-model:
        // checked={appConfigStore.onTop}
        /* onChange={setAlwaysOnTop} */
        <n-checkbox
          size="small"
          class="mx-2"
          label={t('common.windowPin')}
          default-checked={appConfigStore.onTop}
          v-model:checked={appConfigStore.onTop}
          onUpdate-checked={setAlwaysOnTop}
        />,
      ),
  },
  {
    key: 'center',
    label: t('common.centerDisplay'),
    type: 'render',
    render: () =>
      h(
        <n-checkbox
          size="small"
          class="mx-2"
          label={t('common.centerDisplay')}
          default-checked={appConfigStore.center}
          v-model:checked={appConfigStore.center}
          onUpdate-checked={setMainWindowCenter}
        />,
      ),
  },
  {
    key: 'silentStart',
    label: t('common.silentStart'),
    type: 'render',
    render: () =>
      h(
        <n-checkbox
          size="small"
          class="mx-2"
          label={t('common.silentStart')}
          default-checked={appConfigStore.silentStart}
          v-model:checked={appConfigStore.silentStart}
        />,
      ),
  },
  {
    key: 'autoStart',
    label: t('common.autoStart'),
    type: 'render',
    render: () =>
      h(
        <n-checkbox
          size="small"
          class="mx-2"
          label={t('common.autoStart')}
          default-checked={appConfigStore.autoStart}
          v-model:checked={appConfigStore.autoStart}
          onUpdate-checked={setAutoStart}
        />,
      ),
  },

  // {
  //   key: 'about',
  //   label: '关于',
  // },
];

listen<AppConfigState>(AppEvent.UPDATE_APP_CONFIG_DATA, val => {
  for (const key in val.payload) {
    // @ts-expect-error
    appConfigStore[key] = val.payload[key];
  }
});
</script>
