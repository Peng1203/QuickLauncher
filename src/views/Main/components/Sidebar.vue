<template>
  <!-- !w-48 -->
  <n-layout-sider
    id="layoutSider"
    ref="siderRef"
    tabindex="-1"
    collapse-mode="width"
    :collapsed-width="120"
    :width="192"
    :show-collapsed-content="true"
    class="w-48 bg-white border-r border-gray-200 flex flex-col py-1.5 px-2 z-10"
    @contextmenu.prevent.stop="handleShowCategoryContextMenu"
    @keydown="handleKeydown"
  >
    <nav class="flex-1 flex flex-col gap-1">
      <button
        tabindex="-1"
        class="text-left px-4 py-2 rounded-lg transition font-medium cursor-pointer"
        :class="[activeCategory === -1 ? 'bg-gray-100 text-blue-600' : 'hover:bg-gray-50 text-gray-700']"
        @click="handleChangeCategory(-1)"
        @contextmenu.prevent.stop
      >
        {{ `默 认` }}
      </button>

      <button
        v-for="item of categoryData"
        :key="item.id"
        :ref="el => (categoryItemRefs[`${item.id}`] = el)"
        :class="[activeCategory === item.id ? 'bg-gray-100 text-blue-600' : 'hover:bg-gray-50 text-gray-700']"
        tabindex="-1"
        class="text-left px-4 py-2 rounded-lg transition font-medium cursor-pointer overflow-hidden"
        @click="handleChangeCategory(item.id)"
        @contextmenu.prevent.stop="handleShowCategoryItemContextMenu($event, item)"
        @dblclick="handleOpenAssDir(item)"
      >
        <!-- class="px-1 w-fit pointer-events-none line-clamp-2 mt-0.5 leading-normal" -->
        <!-- :ref="`nameRef${item.id}`" -->
        <span
          :ref="el => (itemRefs[`${item.id}`] = el)"
          :contenteditable="item.id === renameItemId"
          class="block whitespace-nowrap overflow-x-auto overflow-y-hidden max-w-full outline-none"
          :class="[item.id === renameItemId && 'editable-active']"
        >
          <!-- class="block whitespace-nowrap overflow-x-auto overflow-y-hidden max-w-full outline-none" -->
          {{ item.name }}
        </span>
      </button>
      <!-- <button @click="unregisterAll()" >取消所有快捷键</button> -->
    </nav>
  </n-layout-sider>

  <!-- 新建/编辑 分类菜单 -->
  <CategoryContextMenu
    v-model="contextMenuVisible"
    :position="contextMenuPosition"
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
import { nextTick, ref, shallowRef } from 'vue';
import { openPath, updateCategory } from '@/api';
import CategoryContextMenu from '@/components/CategoryContextMenu.vue';
import CategoryItemContextMenu from '@/components/CategoryItemContextMenu.vue';
import { useCategoryCorrelationDir } from '@/composables/useCategoryCorrelationDir';
import { AppEvent } from '@/constant';
import { useStore } from '@/store/useStore';
import { EventBus } from '@/utils/eventBus';

const store = useStore();
const { categoryData, activeCategory, activeCategoryItem, activeLaunchItem } = storeToRefs(store);
const { registerAllCategoryDirWatch, checkCategoryDirAndLaunchSync } = useCategoryCorrelationDir();

async function getCategorys() {
  await store.getCategoryData();
  registerAllCategoryDirWatch();
}

const contextMenuVisible = ref<boolean>(false);
const contextMenuPosition = ref({ x: 0, y: 0 });
function handleShowCategoryContextMenu(e: MouseEvent) {
  // EventBus.emit(AppEvent.CLOSE_CONTEXT_MENU);

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
  // EventBus.emit(AppEvent.CLOSE_CONTEXT_MENU);
  itemMenuVisible.value = false;

  setTimeout(() => {
    nextTick(() => {
      itemMenuVisible.value = true;
      activeItem.value = item;
      contextMenuPosition.value = { x: e.clientX, y: e.clientY };
    });
  }, 100);
}

async function handleChangeCategory(id: number) {
  // 每次切换分类时 取消选中的重命名元素
  cancelRename();
  // 清空选中的分类
  if (activeCategory.value === id) return;
  activeLaunchItem.value = null;
  store.launchData = [];
  await store.handleChangeCategory(id);
  checkCategoryDirAndLaunchSync();
}

function handleOpenAssDir(item: CategoryItem) {
  if (!item.association_directory) return;
  openPath(item!.association_directory!);
}

// const currentCategory = computed<CategoryItem>(
//   () => categoryData.value.find(item => item.id === activeCategory.value)!
// )

const activeRenameItem = ref<CategoryItem>();
const activeRenameItemRef = ref(null);
const itemRefs = shallowRef<any>({});
const renameItemId = ref<number>(0);
const renameStatus = ref<boolean>(true);

const oldName = ref('');
function handleRename() {
  const item = activeCategoryItem.value;
  renameStatus.value = true;
  activeRenameItem.value = item;
  renameItemId.value = item.id;
  nextTick(() => {
    const nameRef = itemRefs.value[item.id];
    if (!nameRef) return;
    activeRenameItemRef.value = nameRef;
    // console.log(`%c nameRef ----`, 'color: #fff;background-color: #000;font-size: 18px', nameRef.textContent);
    oldName.value = item.name;
    nameRef?.focus();
    const range = document.createRange();
    range.selectNodeContents(nameRef as any); // 选择元素内的所有内容
    const selection = window.getSelection();
    selection?.removeAllRanges(); // 清除之前的选区
    selection?.addRange(range);
  });
}
EventBus.listen(AppEvent.CATEGORY_RENAME, handleRename);

const siderRef = useTemplateRef('siderRef');
function cancelRename(restore: boolean = true) {
  if (!renameStatus.value) return;
  renameItemId.value = 0;
  renameStatus.value = false;
  // 清除之前的选区
  const selection = window.getSelection();
  selection?.removeAllRanges();
  // 记录修改前的内容 取消时复原
  if (activeRenameItemRef.value && restore) {
    // @ts-ignore
    activeRenameItemRef.value.textContent = oldName.value;
  }

  nextTick(() => {
    // removeAllRanges()会使 activeElement 元素变为body 导致快捷键失效
    siderRef?.value?.$el.focus();
  });
}

async function handleKeydown(e: KeyboardEvent) {
  const { key } = e;
  // console.log('keyCode ------', keyCode);
  console.log('key ------', key);
  switch (key) {
    case 'F2': // 113
      handleRename();
      break;
    case 'Enter': // 13
      if (!renameStatus.value) return;
      nextTick(async () => {
        const params = {
          ...activeCategoryItem.value,
          // @ts-ignore
          name: activeRenameItemRef.value.textContent,
        };
        await updateCategory(params);
        activeCategoryItem.value.name = params.name;
        cancelRename(false);
      });
      e.preventDefault();
      break;
    case 'Escape': // 27
      cancelRename();
      break;
    case 'ArrowUp': // 38
      handleCategorySwitchByKey('up');
      break;
    case 'ArrowDown': // 40
      handleCategorySwitchByKey('down');
      break;
    default:
      break;
  }
}

const maxIndex = computed(() => categoryData.value.length - 1);
const currentIndex = computed(() => categoryData.value.findIndex(item => item.id === activeCategory.value));
const categoryItemRefs = shallowRef<any>({});
async function handleCategorySwitchByKey(direction: DirectionType) {
  if (maxIndex.value <= 1 || currentIndex.value === -1) return;
  let newIndex = 0;

  switch (direction) {
    case 'up':
      // 边界情况处理
      if (!currentIndex.value) newIndex = maxIndex.value;
      else newIndex = currentIndex.value - 1;
      break;
    case 'down':
      if (currentIndex.value === maxIndex.value) newIndex = 0;
      else newIndex = currentIndex.value + 1;
      break;
  }

  const item = categoryData.value[newIndex];
  await handleChangeCategory(item.id);
  // 分类滚动到可视区域

  nextTick(() => {
    const el = categoryItemRefs.value[item.id];
    if (!el) return;
    el?.scrollIntoView({
      behavior: 'smooth',
      block: 'nearest',
    });
  });
}

EventBus.listen(AppEvent.ACTIVE_CATEGORY, async (item: CategoryItem) => {
  await getCategorys();
  await store.handleChangeCategory(item.id);
  nextTick(() => {
    const el = categoryItemRefs.value[item.id];

    if (!el) return;
    el?.scrollIntoView({
      behavior: 'smooth',
      block: 'nearest',
    });
  });
});
</script>

<style lang="scss" scoped>
.editable-active {
  background-color: #dfdfdf81; /* 淡黄色背景 */
  // border: 1px solid #ffc107; /* 黄色边框 */
  border-radius: 4px;
  padding: 0px 6px;
  // box-shadow: 0 0 0 3px rgba(255, 193, 7, 0.1); /* 外发光 */
  cursor: text;
}

// /* 也可以添加动画 */
// .editable-active {
//   animation: editablePulse 0.3s ease-in-out;
// }

// @keyframes editablePulse {
//   0% {
//     transform: scale(1);
//   }
//   50% {
//     transform: scale(1.02);
//   }
//   100% {
//     transform: scale(1);
//   }
// }
</style>
