import { ref } from "vue";

const draggedItem = ref<LaunchItem | null>(null);

/** 启动项跨分类拖拽状态管理 */
export function useLaunchDrag() {
  return { draggedItem };
}
