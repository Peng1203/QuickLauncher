import { updateCategory } from '@/api';
import { useStore } from '@/store/useStore';

export function useCategorySort(item: Ref<CategoryItem>) {
  const store = useStore();
  const isCurrentSelected = computed(() => store.activeCategory === item.value.id);

  async function handleLayoutOrderSortChange<T extends LayoutType | SortByType | SortOrderType>(
    val: T,
    upKey: keyof CategoryItem,
    updateLaunchList: boolean = false,
  ) {
    const params = {
      ...item.value,
      [upKey]: val,
    };
    await updateCategory(params);

    const upCategory = store.categoryData.find(i => i.id === item.value.id);
    if (!upCategory) return;
    // @ts-ignore
    upCategory[upKey] = val;

    if (updateLaunchList && isCurrentSelected.value) store.getLaunchData();
  }

  return { handleLayoutOrderSortChange };
}
