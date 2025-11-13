import {
  isRegistered,
  register,
  unregister,
  unregisterAll,
} from '@tauri-apps/plugin-global-shortcut';

export function useGlobalHotkeys() {
  /**
   * 注册全局快捷键
   * @param key 快捷键组合
   * @param callback 回调函数
   */
  const registerHotkey = async (
    key: string,
    callback: () => void,
  ): Promise<boolean | undefined> => {
    if (await isRegistered(key)) return false;
    await register(key, callback);
  };

  return {
    registerHotkey,
    unregister,
    unregisterAll,
  };
}
