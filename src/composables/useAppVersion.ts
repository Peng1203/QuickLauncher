import { getVersion } from '@tauri-apps/api/app';
import { check } from '@tauri-apps/plugin-updater';
import { useNaiveUiApi } from './useNaiveUiApi';

let cachedVersion: string | null = null;

export function useAppVersion() {
  const { notification } = useNaiveUiApi();
  const version = ref<string | null>(cachedVersion);
  const isChecking = ref(false);
  const updateInfo = ref<{ version: string; date?: string; body?: string } | null>(null);
  const hasUpdate = computed(() => updateInfo.value !== null);

  async function fetchVersion() {
    if (cachedVersion) {
      version.value = cachedVersion;
      return;
    }
    try {
      const v = await getVersion();
      cachedVersion = v;
      version.value = v;
    } catch {
      // getVersion 失败时静默，版本号展示为空
    }
  }

  async function checkUpdate() {
    isChecking.value = true;
    updateInfo.value = null;
    try {
      const result = await check();
      if (!result) {
        notification.success({
          title: '已是最新版本',
          description: '当前版本为最新版本，无需更新',
          duration: 3000,
        });
        return;
      }
      updateInfo.value = {
        version: result.version,
        date: result.date,
        body: result.body,
      };
    } catch (e) {
      notification.error({
        title: '检查更新失败',
        description: `${e}`,
        duration: 5000,
      });
    } finally {
      isChecking.value = false;
    }
  }

  async function downloadAndInstall() {
    try {
      const result = await check();
      if (!result) {
        notification.info({
          title: '没有可用的更新',
          duration: 3000,
        });
        return;
      }
      await result.downloadAndInstall();
    } catch (e) {
      notification.error({
        title: '下载更新失败',
        description: `${e}`,
        duration: 5000,
      });
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
