import {
  runLaunch as handleRunLaunch,
  runLaunchAsAdmin as handleRunLaunchAsAdmin,
} from '@/api';

export function useLaunchAction() {
  /**
   * 执行启动项
   * @param id
   */
  async function runLaunch(id: number) {
    return await handleRunLaunch(id);
  }
  /**
   * 以管理员身份执行启动项
   * @param id
   */
  async function runLaunchAsAdmin(id: number) {
    return await handleRunLaunchAsAdmin(id);
  }

  return {
    runLaunch,
    runLaunchAsAdmin,
  };
}
