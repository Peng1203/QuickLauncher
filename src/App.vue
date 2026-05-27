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
import { storeToRefs } from 'pinia';
import { useTheme } from './composables/useTheme';
import { useStore } from './store/useStore';

const { globalSetThemeFlag } = storeToRefs(useStore());

const {
  font,
  fontSize,
  themeModel, //
  naiveTheme,
  themeOverrides,
  mediaQuery,
  initThemeClass,
  setHTMLThemeClass,
  onOsThemeChange,
  handleOnOSThemeChange,
  setFontFamily,
  setFontSize,
} = useTheme();

watch(
  () => themeModel.value,
  val => {
    if (!globalSetThemeFlag.value) return;
    if (val === 'system') {
      const { matches } = window.matchMedia('(prefers-color-scheme: dark)');
      handleOnOSThemeChange(matches);
    } else {
      setHTMLThemeClass();
    }
  },
);

watch(() => font.value, setFontFamily);
watch(() => fontSize.value, setFontSize);

initThemeClass();
setFontFamily();
setFontSize();

onMounted(() => mediaQuery.addEventListener('change', onOsThemeChange));
onUnmounted(() => mediaQuery.removeEventListener('change', onOsThemeChange));
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
