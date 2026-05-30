<template>
  <NConfigProvider
    :theme="naiveTheme"
    :theme-overrides="themeOverrides"
    :locale="naiveLocale"
    :date-locale="naiveDateLocale"
  >
    <NDialogProvider>
      <NNotificationProvider>
        <NMessageProvider>
          <NModalProvider>
            <router-view />
          </NModalProvider>
        </NMessageProvider>
      </NNotificationProvider>
    </NDialogProvider>
  </NConfigProvider>
</template>

<script setup lang="ts">
import type { NDateLocale, NLocale } from 'naive-ui';

import { getCurrentWindow } from '@tauri-apps/api/window';
import { watchImmediate } from '@vueuse/core';
import { dateEnUS, dateJaJP, dateZhCN, dateZhTW, enUS, jaJP, zhCN, zhTW } from 'naive-ui';

import { useAppConfig } from './composables';
import { useTheme } from './composables/useTheme';
import { AppEvent } from './constant';
import { i18n } from './i18n';
import { EventBus } from './utils/eventBus';

const {
  font, //
  fontSize,
  naiveTheme,
  themeOverrides,
  setThemeClass,
  setHTMLThemeClass,
  setFontFamily,
  setFontSize,
} = useTheme();

const { appConfigStore } = useAppConfig();

const naiveLocaleMap: Record<string, NLocale> = {
  'zh-CN': zhCN,
  'zh-HK': zhTW,
  // prettier-ignore
  'en': enUS,
  // prettier-ignore
  'ja': jaJP,
};

const naiveDateLocaleMap: Record<string, NDateLocale> = {
  'zh-CN': dateZhCN,
  'zh-HK': dateZhTW,
  // prettier-ignore
  'en': dateEnUS,
  // prettier-ignore
  'ja': dateJaJP,
};

const naiveLocale = computed(() => naiveLocaleMap[appConfigStore.language] ?? zhCN);
const naiveDateLocale = computed(() => naiveDateLocaleMap[appConfigStore.language] ?? dateZhCN);

// 语言切换联动
watch(
  () => appConfigStore.language,
  lang => {
    i18n.global.locale.value = lang;
  },
  { immediate: true },
);

// 跨窗口主题同步
EventBus.listen(AppEvent.CHANGE_THEME, async (windowLabel: string) => {
  const currentWindow = await getCurrentWindow();
  if (currentWindow.label === windowLabel) return;

  setHTMLThemeClass();
});

setThemeClass();
watchImmediate(font, setFontFamily);
watchImmediate(fontSize, setFontSize);
</script>

<style>
::view-transition-old(root),
::view-transition-new(root) {
  animation: none !important;
  mix-blend-mode: normal !important;
}

/* 进入dark模式和退出dark模式时，两个图像的位置顺序正好相反 */
[class='dark']::view-transition-old(root) {
  z-index: 10000;
}

[class='dark']::view-transition-new(root) {
  z-index: 99999;
}

::view-transition-old(root) {
  z-index: 99999;
}

::view-transition-new(root) {
  z-index: 10000;
}
</style>
