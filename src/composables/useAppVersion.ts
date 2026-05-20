import { getVersion } from '@tauri-apps/api/app';
import { check } from '@tauri-apps/plugin-updater';

let cachedVersion: string | null = null;

export function useAppVersion() {
  const version = ref<string | null>(cachedVersion);
  const isChecking = ref(false);
  const updateInfo = ref<{ version: string; date?: string; body?: string } | null>(null);
  const hasUpdate = computed(() => updateInfo.value !== null);

  async function fetchVersion() {
    if (cachedVersion) {
      version.value = cachedVersion;
      return;
    }
    const v = await getVersion();
    cachedVersion = v;
    version.value = v;
  }

  async function checkUpdate() {
    isChecking.value = true;
    updateInfo.value = null;
    try {
      const result = await check();
      if (result) {
        updateInfo.value = {
          version: result.version,
          date: result.date,
          body: result.body,
        };
      }
    } finally {
      isChecking.value = false;
    }
  }

  async function downloadAndInstall() {
    if (!updateInfo.value) return;
    const result = await check();
    if (result) {
      await result.downloadAndInstall();
    }
  }

  return {
    version,
    isChecking,
    updateInfo,
    hasUpdate,
    fetchVersion,
    checkUpdate,
    downloadAndInstall,
  };
}
