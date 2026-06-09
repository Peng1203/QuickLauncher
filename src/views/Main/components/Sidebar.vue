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
    class="w-48 bg-card border-r border-border flex flex-col py-1.5 px-2 z-10"
    @contextmenu.prevent.stop="handleShowCategoryContextMenu"
    @keydown="handleKeydown"
  >
    <nav class="flex-1 flex flex-col gap-1">
      <button
        v-for="item of categoryData"
        :key="item.id"
        :ref="(el) => (categoryItemRefs[`${item.id}`] = el)"
        :class="[
          activeCategory === item.id
            ? 'bg-muted active-category'
            : 'hover:bg-secondary text-foreground',
          dragOverCategoryId === item.id && 'drag-over',
        ]"
        tabindex="-1"
        class="text-left px-4 py-2 rounded-lg transition font-medium cursor-pointer overflow-hidden flex items-center gap-1.5"
        @click="handleChangeCategory(item.id)"
        @contextmenu.prevent.stop="handleShowCategoryItemContextMenu($event, item)"
        @dblclick="handleOpenAssDir(item)"
        @dragover="handleDragOver($event, item)"
        @dragleave="handleDragLeave($event, item)"
        @drop="handleDrop($event, item)"
      >
        <!-- class="px-1 w-fit pointer-events-none line-clamp-2 mt-0.5 leading-normal" -->
        <!-- :ref="`nameRef${item.id}`" -->
        <img v-if="appConfigStore.showCategoryIcon && item.icon" class="w-5 h-5" :src="item.icon" />
        <span
          :ref="(el) => (itemRefs[`${item.id}`] = el)"
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

  <!-- 分类菜单 -->
  <CategoryContextMenu v-model="contextMenuVisible" :position="contextMenuPosition" />

  <!-- 分类项自定义菜单 -->
  <CategoryItemContextMenu
    v-model="itemMenuVisible"
    :item="activeItem!"
    :position="contextMenuPosition"
  />

  <!--  -->
</template>

<script setup lang="ts">
import { storeToRefs } from "pinia";
import { nextTick, ref, shallowRef } from "vue";
import { openPath, updateCategory, updateLaunch } from "@/api";
import CategoryContextMenu from "@/components/CategoryContextMenu.vue";
import CategoryItemContextMenu from "@/components/CategoryItemContextMenu.vue";
import { useAppConfig, useCategoryCorrelationDir, useLaunchDrag } from "@/composables";
import { AppEvent } from "@/constant";
import { useStore } from "@/store/useStore";
import { EventBus } from "@/utils/eventBus";

const store = useStore();
const { categoryData, activeCategory, activeCategoryItem, activeLaunchItem, enableWindoShortcuts } =
  storeToRefs(store);
const { registerAllCategoryDirWatch, checkCategoryDirAndLaunchSync } = useCategoryCorrelationDir();
const { themeColor, appConfigStore } = useAppConfig();
const { draggedItem } = useLaunchDrag();
const dragOverCategoryId = ref<number | null>(null);

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

// ===== 拖拽到分类 =====

function handleDragOver(e: DragEvent, item: CategoryItem) {
  // 关联目录分类禁止放置
  if (item.association_directory) return;
  // 禁止放置到当前分类自身
  if (!draggedItem.value || draggedItem.value.category_id === item.id) return;
  e.preventDefault();
  e.dataTransfer!.dropEffect = "move";
  dragOverCategoryId.value = item.id;
}

function handleDragLeave(e: DragEvent, item: CategoryItem) {
  if (dragOverCategoryId.value === item.id) {
    dragOverCategoryId.value = null;
  }
}

async function handleDrop(e: DragEvent, item: CategoryItem) {
  e.preventDefault();
  dragOverCategoryId.value = null;

  if (!draggedItem.value) return;
  // 关联目录分类禁止放置
  if (item.association_directory) return;
  // 禁止放置到当前分类自身
  if (draggedItem.value.category_id === item.id) return;

  const targetCategoryId = item.id;
  const movedItem = { ...draggedItem.value, category_id: targetCategoryId };
  draggedItem.value = null;

  await updateLaunch(movedItem);
  // 刷新当前分类列表
  void store.getLaunchData();
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

const oldName = ref("");
function handleRename() {
  const item = activeCategoryItem.value;
  renameStatus.value = true;
  activeRenameItem.value = item;
  renameItemId.value = item.id;
  enableWindoShortcuts.value = false;

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

const siderRef = useTemplateRef("siderRef");
function cancelRename(restore: boolean = true) {
  if (!renameStatus.value) return;
  renameItemId.value = 0;
  renameStatus.value = false;
  enableWindoShortcuts.value = true;
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
  console.log("key ------", key);
  switch (key) {
    case "F2": // 113
      handleRename();
      break;
    case "Enter": // 13
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
    case "Escape": // 27
      cancelRename();
      break;
    case "ArrowUp": // 38
      handleCategorySwitchByKey("up");
      break;
    case "ArrowDown": // 40
      handleCategorySwitchByKey("down");
      break;
    default:
      break;
  }
}

const maxIndex = computed(() => categoryData.value.length - 1);
const currentIndex = computed(() =>
  categoryData.value.findIndex((item) => item.id === activeCategory.value),
);
const categoryItemRefs = shallowRef<any>({});
async function handleCategorySwitchByKey(direction: DirectionType) {
  if (maxIndex.value <= 1) return;
  let newIndex = 0;

  switch (direction) {
    case "up":
      // 边界情况处理
      if (!currentIndex.value) newIndex = maxIndex.value;
      else newIndex = currentIndex.value - 1;
      break;
    case "down":
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
      behavior: "smooth",
      block: "nearest",
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
      behavior: "smooth",
      block: "nearest",
    });
  });
});
</script>

<style lang="scss" scoped>
.editable-active {
  background-color: color-mix(in srgb, var(--secondary) 50%, transparent);
  border-radius: 4px;
  padding: 0px 6px;
  cursor: text;
}
.active-category {
  color: v-bind("themeColor") !important;
}
.drag-over {
  background-color: color-mix(in srgb, v-bind("themeColor") 20%, transparent) !important;
  outline: 2px dashed v-bind("themeColor");
  outline-offset: -2px;
}
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
