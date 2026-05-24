<template>
  <NConfigProvider :theme="naiveTheme" :theme-overrides="themeOverrides">
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
import { darkTheme } from 'naive-ui';
import { useAppConfig } from '@/composables';

const { appConfigStore } = useAppConfig();

const prefersDark = ref(window.matchMedia('(prefers-color-scheme: dark)').matches);

const isDark = computed(() => {
  if (appConfigStore.appearanceMode === 'dark') return true;
  if (appConfigStore.appearanceMode === 'light') return false;
  return prefersDark.value;
});

const naiveTheme = computed(() => (isDark.value ? darkTheme : undefined));

const themeOverrides = computed(() => ({
  common: {
    primaryColor: appConfigStore.themeColor,
    primaryColorHover: appConfigStore.themeColor,
  },
}));

// Toggle .dark class on root element for Tailwind dark: utilities
watchEffect(() => {
  document.documentElement.classList.toggle('dark', isDark.value);
});

// Listen for OS theme changes (for "follow system" mode)
const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
function onOsThemeChange(e: MediaQueryListEvent) {
  prefersDark.value = e.matches;
}
mediaQuery.addEventListener('change', onOsThemeChange);

onUnmounted(() => {
  mediaQuery.removeEventListener('change', onOsThemeChange);
});
</script>
