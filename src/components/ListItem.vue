<template>
  <div
    class="item flex flex-col items-center min-w-20 w-full h-18 cursor-pointer select-none hover:bg-opacity-20 rounded"
    :title="item.remarks || name"
    :style="
      activeItemIds.includes(item.id) // activeItems?.id === item.id
        ? { backgroundColor: '#f5f5f5 !important' }
        : {}
    "
    tabindex="0"
    @click="handleActive($event)"
    @dblclick="handleRun"
    @keydown="handleKeydown"
    @contextmenu.prevent.stop="handleShowContextMenu"
  >
    <img
      :src="icon"
      alt="icon"
      class="object-contain pointer-events-none w-8 mt-0.5"
    />
    <!-- v-if="!isEdit" -->
    <!-- {{ isEdit }} -->

    <span
      ref="nameRef"
      :contenteditable="isEdit && activeItemIds.includes(item.id)"
      class="text-xs text-center text-black px-1 w-fit pointer-events-none line-clamp-2 mt-0.5 leading-normal max-2-lines"
    >
      {{ name }}

      <!-- -- {{ newName }} -->
    </span>

    <!-- :visible="menuVisible" -->
    <LaunchItemContextMenu
      v-model="menuVisible"
      :item="item"
      :selected-ids="activeItemIds"
      :position="menuPosition"
      :item-path="item.path"
      :item-name="item.name"
      @rename="handleEditName"
    />
  </div>
</template>

<script setup lang="ts">
import { renameLaunch, runLaunch } from '@/api';
import { AppEvent } from '@/constant';
import { EventBus } from '@/utils/eventBus';
import LaunchItemContextMenu from './ListItemContextMenu.vue';

const props = defineProps<{
  icon: string;
  name: string;
  item: LaunchItem;
  activeCategoryItem?: CategoryItem;
}>();

// 鼠标单击选中的项
const activeItems = defineModel<LaunchItem[]>();
const activeItemIds = computed(() => {
  if (!activeItems.value || !activeItems.value.length) return [];
  return activeItems.value?.map(item => item.id);
});

const isEdit = ref<boolean>(false);
const newName = ref<string>(props.name);

function handleRun() {
  runLaunch(props.item.id);
}

function handleKeydown(e: KeyboardEvent) {
  const { keyCode, key } = e;

  console.log(`%c keyCode ----`, 'color: #fff;background-color: #000;font-size: 18px', keyCode, key);
  switch (key) {
    case 'F2': // 113
      handleEditName();
      break;
    case 'Enter': // 13
      handleSaveEditName();
      e.preventDefault();
      break;
    case 'Escape': // 27
      handleCancelEditName();
      break;
    default:
      break;
  }
}

const nameRef = useTemplateRef('nameRef');
function handleEditName() {
  handleActive();
  if (isEdit.value) return;
  newName.value = props.name;
  isEdit.value = true;
  nextTick(() => {
    nameRef.value?.focus();
    const range = document.createRange();
    range.selectNodeContents(nameRef.value as any); // 选择元素内的所有内容
    const selection = window.getSelection();
    selection?.removeAllRanges(); // 清除之前的选区
    selection?.addRange(range);

    // setInputWidth()
  });
}

function handleCancelEditName() {
  isEdit.value = false;
}

async function handleSaveEditName() {
  // @ts-ignore
  await saveEditName(nameRef.value.textContent);
  isEdit.value = false;
  // 更新数据
  EventBus.emit(AppEvent.UPDATE_LAUNCH_LIST);
}

async function saveEditName(newName: string) {
  try {
    await renameLaunch(props.item.id, newName);
  } catch (e) {
    console.log('e', e);
  }
}

const menuVisible = ref(false);
const menuPosition = ref({ x: 0, y: 0 });
const isSelected = computed(() => !!((activeItems.value?.length || 0) > 1));
function handleShowContextMenu(e: MouseEvent) {
  // if (props.activeCategoryItem?.association_directory) return;
  // 先关闭其他菜单 再打开当前菜单
  EventBus.emit(AppEvent.CLOSE_CONTEXT_MENU);

  setTimeout(() => {
    nextTick(() => {
      // 判断是否处于复选状态
      menuVisible.value = true;
      menuPosition.value = { x: e.clientX, y: e.clientY };
      if (isSelected.value) {
        console.log(`%c isSelected ----`, 'color: #fff;background-color: #000;font-size: 18px', isSelected.value);
      } else {
        // 选中当前菜单
        activeItems.value = [props.item];
      }
    });
  }, 100);
}

function handleActive(e?: PointerEvent) {
  // 当用户同时按下ctrl + 点击时为复选操作
  isEdit.value = false;
  if (e?.ctrlKey) {
    // 判断当前按下的启动项是否已经在复选列表中 如果存在则从复选列表中移除
    if (activeItemIds.value.includes(props.item.id)) {
      const index = activeItems.value?.findIndex(item => item.id === props.item.id);

      if (!index || index === -1) return;
      activeItems.value?.splice(index, 1);
    } else {
      activeItems.value?.push(props.item);
    }
  } else {
    activeItems.value = [props.item];
  }
}
</script>

<style scoped>
.item:hover {
  /* background-color: #f5f5f5; */
  background-color: #f5f5f586;
}
span[contenteditable='true'] {
  display: inline-block;
  max-width: 100%;
  width: fit-content;
  border: 1px solid rgba(128, 128, 128, 0.486); /* 蓝色边框 */
  /* padding: 4px; */
  border-radius: 4px;
}
.max-2-lines {
  display: inline-block;
  max-width: 100%;
  width: fit-content;
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: clip; /* 不要省略号 */
}
</style>
