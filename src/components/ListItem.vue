<template>
  <div
    class="item flex flex-col items-center min-w-20 w-full h-18 cursor-pointer select-none hover:bg-opacity-20 rounded pt-0.5 pb-0.5"
    :title="item.remarks || name"
    :style="
      activeItem?.id === item.id
        ? { backgroundColor: '#f5f5f5 !important' }
        : {}
    "
    tabindex="0"
    @click="handleActive"
    @dblclick="handleRun"
    @keydown="handleKeydown"
    @contextmenu.prevent.stop="handleShowContextMenu"
  >
    <img
      :src="icon"
      alt="icon"
      class="object-contain pointer-events-none w-8"
    />
    <!-- v-if="!isEdit" -->
    <!-- {{ isEdit }} -->

    <span
      ref="nameRef"
      :contenteditable="isEdit && activeItem?.id === item.id"
      class="text-xs text-center text-black px-1 w-fit pointer-events-none line-clamp-2 mt-0.5 leading-normal max-2-lines"
    >
      {{ name }}
      <!-- -- {{ newName }} -->
    </span>

    <!-- :visible="menuVisible" -->
    <LaunchItemContextMenu
      v-model="menuVisible"
      :item="item"
      :position="menuPosition"
      :item-path="item.path"
      :item-name="item.name"
      @rename="handleEditName"
    />
  </div>
</template>

<script setup lang="ts">
import { renameLaunch } from '@/api';
import { useLaunchAction } from '@/composables/useLaunchAction';
import { AppEvent } from '@/constant';
import { EventBus } from '@/utils/eventBus';
import LaunchItemContextMenu from './ListItemContextMenu.vue';

const props = defineProps<{
  icon: string;
  name: string;
  item: LaunchItem;
}>();

const { runLaunch } = useLaunchAction();

// 鼠标单击选中的项
const activeItem = defineModel<LaunchItem>();

const isEdit = ref<boolean>(false);
const newName = ref<string>(props.name);

function handleRun() {
  runLaunch(props.item.id);
}

function handleKeydown(e: KeyboardEvent) {
  const { keyCode, key } = e;

  console.log(
    `%c keyCode ----`,
    'color: #fff;background-color: #000;font-size: 18px',
    keyCode,
    key,
  );
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

function handleShowContextMenu(e: MouseEvent) {
  // 先关闭其他菜单 再打开当前菜单
  EventBus.emit(AppEvent.CLOSE_CONTEXT_MENU);

  setTimeout(() => {
    nextTick(() => {
      menuVisible.value = true;
      menuPosition.value = { x: e.clientX, y: e.clientY };

      // 选中当前菜单
      activeItem.value = props.item;
    });
  }, 100);
}

function handleActive() {
  isEdit.value = false;
  activeItem.value = props.item;
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
