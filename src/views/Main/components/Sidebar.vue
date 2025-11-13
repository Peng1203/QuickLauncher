<template>
  <aside
    tabindex="-1"
    class="fixed top-8 left-0 bottom-0 w-48 bg-white border-r border-gray-200 flex flex-col py-3 px-2 z-10"
    @contextmenu.prevent.stop="handleShowCategoryContextMenu"
  >
    <nav class="flex-1 flex flex-col gap-1">
      <button
        tabindex="-1"
        class="text-left px-4 py-2 rounded-lg transition font-medium cursor-pointer"
        :class="[
          activeCategory === -1
            ? 'bg-gray-100 text-blue-600'
            : 'hover:bg-gray-50 text-gray-700',
        ]"
        @click="handleChangeCategory(-1)"
        @contextmenu.prevent.stop
      >
        {{ `默 认` }}
      </button>

      <button
        v-for="item of categoryData"
        :key="item.id"
        :class="[
          activeCategory === item.id
            ? 'bg-gray-100 text-blue-600'
            : 'hover:bg-gray-50 text-gray-700',
        ]"
        tabindex="-1"
        class="text-left px-4 py-2 rounded-lg transition font-medium cursor-pointer"
        @click="handleChangeCategory(item.id)"
        @contextmenu.prevent.stop="
          handleShowCategoryItemContextMenu($event, item)
        "
      >
        {{ item.name }}
      </button>
      <!-- <button @click="unregisterAll()" >取消所有快捷键</button> -->
    </nav>
  </aside>

  <!-- 新建/编辑 分类菜单 -->
  <CategoryContextMenu
    v-model="contextMenuVisible"
    :position="contextMenuPosition"
    @add="handleOpenAddCategory"
  />

  <!-- 分类自定义菜单 -->
  <CategoryItemContextMenu
    v-model="itemMenuVisible"
    :item="activeItem!"
    :position="contextMenuPosition"
  />

  <!--  -->
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia';
import CategoryContextMenu from '@/components/CategoryContextMenu.vue';
import CategoryItemContextMenu from '@/components/CategoryItemContextMenu.vue';
import { AppEvent } from '@/constant';
import { useStore } from '@/store/useStore';
import { EventBus } from '@/utils/eventBus';

const store = useStore();
const { categoryData, activeCategory } = storeToRefs(store);

function handleChangeCategory(id: number) {
  activeCategory.value = id;
  nextTick(() => {
    store.getLaunchData();
  });
}

function handleOpenAddCategory() {
  console.log(
    `%c 121 ----`,
    'color: #fff;background-color: #000;font-size: 18px',
    121
  );
}

store.getCategoryData();

const contextMenuVisible = ref<boolean>(false);
const contextMenuPosition = ref({ x: 0, y: 0 });
function handleShowCategoryContextMenu(e: MouseEvent) {
  EventBus.emit(AppEvent.CLOSE_CONTEXT_MENU);

  setTimeout(() => {
    nextTick(() => {
      contextMenuVisible.value = true;
      contextMenuPosition.value = { x: e.clientX, y: e.clientY };
    });
  }, 100);
}

const itemMenuVisible = ref<boolean>(false);
const activeItem = ref<CategoryItem>();
function handleShowCategoryItemContextMenu(e: MouseEvent, item: CategoryItem) {
  EventBus.emit(AppEvent.CLOSE_CONTEXT_MENU);

  setTimeout(() => {
    nextTick(() => {
      itemMenuVisible.value = true;
      activeItem.value = item;
      contextMenuPosition.value = { x: e.clientX, y: e.clientY };
    });
  }, 100);
}

// const currentCategory = computed<CategoryItem>(
//   () => categoryData.value.find(item => item.id === activeCategory.value)!
// )

EventBus.listen(AppEvent.UPDATE_CATEGORY_LIST, store.getCategoryData);
</script>
