<template>
  <NConfigProvider
    :theme="naiveTheme"
    :theme-overrides="themeOverrides"
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
import { getCurrentWindow } from '@tauri-apps/api/window';
import { watchImmediate } from '@vueuse/core';
import { useTheme } from './composables/useTheme';
import { AppEvent } from './constant';
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
