import { defineStore } from 'pinia';
import { getCategory, getLaunchs } from '@/api';

export const useStore = defineStore('main', {
  state: () => ({
    launchData: <LaunchItem[]>[],

    activeCategory: -1,
    categoryData: <CategoryItem[]>[],
    categoryOptions: <OptionItem[]>[],
  }),
  actions: {
    async getLaunchData(id?: number) {
      const data = await getLaunchs(id || this.activeCategory);
      this.launchData = data;
    },

    async getCategoryData() {
      const data = await getCategory();
      this.categoryData = data;
      this.categoryOptions = data.map(item => ({
        value: item.id,
        label: item.name,
      }));
    },

    async handleChangeCategory(id: number) {
      this.activeCategory = id;
      await this.getLaunchData(id);
    },
  },
  getters: {
    activeCategoryItem: state => state.categoryData.find(item => item.id === state.activeCategory) as CategoryItem,
  },
});
