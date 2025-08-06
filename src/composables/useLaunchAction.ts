import { invoke } from '@tauri-apps/api/core'

export const useLaunchAction = () => {
  /**
   * 执行启动项
   * @param id
   */
  async function runLaunch(id: number) {
    return await invoke('run_launch', { id })
  }
  /**
   * 以管理员身份执行启动项
   * @param id
   */
  async function runLaunchAsAdmin(id: number) {
    return await invoke('run_launch_as_admin', { id })
  }

  return {
    runLaunch,
    runLaunchAsAdmin,
  }
}
