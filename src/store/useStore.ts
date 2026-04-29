import { defineStore } from 'pinia';
import { getCategory, getLaunchs } from '@/api';
import { ACTIVE_CATEGORY_LOCAL_KEY } from '@/constant';

export const useStore = defineStore('main', {
  state: () => ({
    launchData: <LaunchItem[]>[],
    activeLaunchItem: <LaunchItem | null>{},
    activeCategory: Number(sessionStorage.getItem(ACTIVE_CATEGORY_LOCAL_KEY)) || 0,
    defaultCategory: <CategoryItem>{},
    categoryData: <CategoryItem[]>[],
    categoryOptions: <OptionItem[]>[],
    // 光标选中的启动项 左边
    activeCursorX: 0,
    activeCursorY: 0,
  }),
  actions: {
    async getLaunchData(id?: number) {
      const categoryId = id ?? this.activeCategory;

      const params = [
        categoryId,
        this.activeCategoryItem?.sort_by,
        this.activeCategoryItem?.sort_order,
        !this.activeCategoryItem?.exclude,
        categoryId === this.defaultCategory.id,
      ];

      // @ts-ignore
      const data = await getLaunchs(...params);
      this.launchData = data;
    },

    async getCategoryData(init: boolean = false) {
      const data = await getCategory();
      this.categoryData = data;
      this.categoryOptions = data
        // .filter(item => !item.association_directory)
        .map(item => ({
          // 部分分了关联了 文件目录 在 option中 禁止这些目录作为选项
          disable: !!item.association_directory,
          value: item.id,
          label: item.name,
        }));

      // 找出默认分类
      this.defaultCategory = data.find(item => item.order_index === 9999)!;

      if (init && !sessionStorage.getItem(ACTIVE_CATEGORY_LOCAL_KEY)) this.activeCategory = this.defaultCategory.id;
    },

    async handleChangeCategory(id: number) {
      this.activeCategory = id;
      sessionStorage.setItem(ACTIVE_CATEGORY_LOCAL_KEY, id + '');
      this.activeCursorX = 0;
      this.activeCursorY = 0;
      await this.getLaunchData(id);
    },
  },
  getters: {
    activeCategoryItem: state => state.categoryData.find(item => item.id === state.activeCategory) as CategoryItem,
  },
  tauri: {
    autoStart: true,
    saveInterval: 1000,
    saveStrategy: 'debounce',

    hooks: {
      // 初始化钩子 在rust端之前调用
      beforeBackendSync: (state: any) => state,
    },
    saveOnChange: true,
  },
  persist: true,
});
