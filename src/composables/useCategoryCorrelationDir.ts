import { useStore } from '@/store/useStore';
import { storeToRefs } from 'pinia';
import { watch } from 'vue';
import { watch as watchDir } from '@tauri-apps/plugin-fs';

/** 监听分类关联目录 */
export const useCategoryCorrelationDir = () => {
  const store = useStore();
  const { launchData, activeCategory, categoryData } = storeToRefs(store);
  const activeCategoryItem = computed(() => {
    if (activeCategory.value === -1) return null;
    return categoryData.value.find(item => item.id === activeCategory.value);
  });

  const watchMap = new Map();

  watch(
    () => categoryData.value,
    async (val, oldVal) => {
      // eslint-disable-next-line no-useless-return
      if (!val.length) return;
      console.log(
        `%c val ----`,
        'color: #fff;background-color: #000;font-size: 18px',
        val,
        oldVal,
      );

      for await (const item of val) {
        if (!item.association_directory) continue;
        if (watchMap.has(item.id)) continue;

        const unWatch = await watchDir(
          item.association_directory,
          event => {
            console.log('event ------', event);
            event.type;
            console.log(
              `%c 目录有变化了!!! ----`,
              'color: #fff;background-color: #000;font-size: 18px',
              item.association_directory,
              item,
            );
          },
          { delayMs: 300 },
        );

        watchMap.set(item.id, unWatch);
      }
    },
    { deep: true, immediate: true },
  );

  const isConrrelationDir = computed(
    () => !!activeCategoryItem.value?.association_directory,
  );

  // watchEffect(() => {
  //   console.log('activeCategory ------', activeCategory.value);
  //   console.log('activeCategoryItem ------', activeCategoryItem.value);
  // });
  // onMounted(() => {
  //   console.log(
  //     `%c categoryData.value ----`,
  //     'color: #fff;background-color: #000;font-size: 18px',
  //     categoryData.value,
  //   );
  // });
  return {
    activeCategoryItem,
    isConrrelationDir,
  };
};
