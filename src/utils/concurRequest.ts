/**
 * 批量执行异步接口请求函数，带并发控制
 * @param tasks - 一个个返回 Promise 的函数数组
 * @param concurrency - 最大并发数
 * @returns Promise<结果数组>
 */
export async function batchRequest(tasks: (() => Promise<any>)[], concurrency = 5) {
  const results: any[] = new Array(tasks.length);
  let index = 0;

  // 内部 worker 函数：负责执行队列中的任务
  async function worker() {
    while (index < tasks.length) {
      const currentIndex = index++;
      try {
        const res = await tasks[currentIndex]();
        results[currentIndex] = { status: 'fulfilled', value: res };
      } catch (err) {
        results[currentIndex] = { status: 'rejected', reason: err };
      }
    }
  }

  // 启动 N 个并发 worker
  await Promise.all(new Array(Math.min(concurrency, tasks.length)).fill(null).map(() => worker()));

  return results;
}
