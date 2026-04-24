import { defineStore } from 'pinia';
import { getCategory, getLaunchs } from '@/api';
import { ACTIVE_CATEGORY_LOCAL_KEY } from '@/constant';

export const useStore = defineStore('main', {
  state: () => ({
    launchData: <LaunchItem[]>[],
    activeLaunchItem: <LaunchItem | null>{},
    activeCategory: Number(localStorage.getItem(ACTIVE_CATEGORY_LOCAL_KEY)) || -1,
    categoryData: <CategoryItem[]>[],
    categoryOptions: <OptionItem[]>[],
  }),
  actions: {
    async getLaunchData(id?: number) {
      const categoryId = id ?? this.activeCategory;

      const params =
        categoryId === -1
          ? [categoryId]
          : [categoryId, this.activeCategoryItem?.sort_by, this.activeCategoryItem?.sort_order];

      // @ts-ignore
      const data = await getLaunchs(...params);
      this.launchData = data;
    },

    async getCategoryData() {
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
    },

    async handleChangeCategory(id: number) {
      this.activeCategory = id;
      localStorage.setItem(ACTIVE_CATEGORY_LOCAL_KEY, id + '');
      await this.getLaunchData(id);
    },
  },
  getters: {
    activeCategoryItem: state => state.categoryData.find(item => item.id === state.activeCategory) as CategoryItem,
  },
});
