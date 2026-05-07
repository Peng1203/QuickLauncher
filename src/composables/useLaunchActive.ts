import { useStore } from '@/store/useStore';
import { storeToRefs } from 'pinia';

export function useLaunchActive() {
  const store = useStore();
  const { activeCategoryItem } = storeToRefs(store);
  const gridRowMaxItem = computed(() => (activeCategoryItem.value?.layout === 'list' ? 1 : 6));

  // 通过列表启动项索引查询对应的坐标
  function getPositionByIndex(i: number) {
    if (i === -1) return { x: 0, y: 0 };
    const x = Math.ceil((i + 1) / gridRowMaxItem.value);
    const y = (i + 1) % gridRowMaxItem.value || gridRowMaxItem.value;
    return { x, y };
  }
  return {
    gridRowMaxItem,
    getPositionByIndex,
  };
}
