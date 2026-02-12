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
      try {
        const data = await getLaunchs(id || this.activeCategory);
        this.launchData = data;
      } catch (e) {
        console.log('e', e);
      }
    },

    async getCategoryData() {
      try {
        const data = await getCategory();

        this.categoryData = data;
        this.categoryOptions = data.map(item => ({
          value: item.id,
          label: item.name,
        }));
      } catch (e) {
        console.log('e', e);
      }
    },

    async handleChangeCategory(id: number) {
      this.activeCategory = id;
      await this.getLaunchData(id);
    },
  },
});
