<template>
  <div
    ref="itemRef"
    class="item cursor-pointer select-none rounded transition-colors"
    :class="[
      isGridMode
        ? 'flex flex-col items-center min-w-20 h-18 hover:bg-gray-100/60'
        : 'flex items-center px-3 py-2 hover:bg-gray-100',
    ]"
    :style="isActive ? { backgroundColor: '#f5f5f5 !important' } : {}"
    tabindex="0"
    @click="handleActive"
    @dblclick="handleRun"
    @keydown="handleKeydown"
    @contextmenu.prevent.stop="handleShowContextMenu"
  >
    <!-- :title="item.remarks || name" -->
    <img
      :src="icon"
      alt="icon"
      :class="isGridMode ? 'w-8 mt-0.5' : 'w-6 h-6 mr-3 shrink-0'"
      class="object-contain pointer-events-none"
    />

    <span
      ref="nameRef"
      :contenteditable="isEdit && isActive"
      class="text-black pointer-events-none px-1 w-fit line-clamp-2 mt-0.5 leading-normal max-2-lines"
      :class="[isGridMode ? 'text-xs text-center px-1 mt-0.5 line-clamp-2' : 'text-sm flex-1 truncate']"
    >
      {{ name }}
    </span>

    <!-- 右侧扩展（list模式才有） -->
    <template v-if="isListMode">
      <div class="text-xs text-gray-400 w-24">
        {{ dateFormat(item.created_at, 'YYYY/M/D H:m') || '' }}
      </div>

      <div class="text-xs text-gray-400 w-12">
        {{ item.extension || formatLaunchType(item.type) }}
      </div>
    </template>

    <LaunchItemContextMenu
      v-model="menuVisible"
      :item="item"
      :position="menuPosition"
      :item-path="item.path"
      :item-name="item.name"
      @rename="handleEditName"
      @delete="handleDelete"
    />
  </div>
</template>

<script setup lang="ts">
import { isEmpty } from 'lodash-es';
import { storeToRefs } from 'pinia';
import { deleteLaunch, renameLaunch, runLaunch } from '@/api';
import { formatLaunchType } from '@/common/formatLaunchType';
import { useNaiveUiApi } from '@/composables/useNaiveUiApi';
import { AppEvent } from '@/constant';
import { useStore } from '@/store/useStore';
import { dateFormat } from '@/utils/date';
import { EventBus } from '@/utils/eventBus';
import LaunchItemContextMenu from './ListItemContextMenu.vue';

const props = defineProps<{
  icon: string;
  name: string;
  item: LaunchItem;
}>();

const store = useStore();
const { activeCategoryItem, activeLaunchItem, launchData, activeCursorX, activeCursorY } = storeToRefs(store);

// 鼠标单击选中的项
const activeItems = defineModel<LaunchItem[]>();

const isGridMode = computed(() => (activeCategoryItem.value?.layout || 'grid') === 'grid');
const isListMode = computed(() => (activeCategoryItem.value?.layout || 'grid') === 'list');
const isActive = computed(() => activeLaunchItem.value?.id === props.item.id);

const isEdit = ref<boolean>(false);
const newName = ref<string>(props.name);

function handleRun() {
  runLaunch(props.item.id);
}

function handleKeydown(e: KeyboardEvent) {
  const { keyCode, key } = e;

  console.log(`%c keyCode ----`, 'color: #fff;background-color: #000;font-size: 18px', keyCode, key);
  switch (key) {
    // case 'F2': // 113
    //   handleEditName();
    //   break;
    case 'Enter': // 13
      if (isEdit.value) {
        handleSaveEditName();
      } else {
        isActive.value && handleRun();
      }
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
  // 判断是否有选中启动项
  const isEmp = isEmpty(activeLaunchItem.value);
  if (isEmp) return;

  if (activeCategoryItem.value?.association_directory) return;
  // handleActive();
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
  if (activeCategoryItem.value?.association_directory) return;
  // 先关闭其他菜单 再打开当前菜单
  // EventBus.emit(AppEvent.CLOSE_CONTEXT_MENU);

  setTimeout(() => {
    nextTick(() => {
      // 判断是否处于复选状态
      menuVisible.value = true;
      menuPosition.value = { x: e.clientX, y: e.clientY };
      if (isSelected.value) {
        console.log(`%c isSelected ----`, 'color: #fff;background-color: #000;font-size: 18px', isSelected.value);
      } else {
        // 选中当前菜单
        activeLaunchItem.value = props.item;
      }
    });
  }, 100);
}
const gridRowMaxItem = computed(() => (activeCategoryItem.value?.layout === 'list' ? 1 : 6));

// e?: PointerEvent
function handleActive() {
  isEdit.value = false;
  // 手动点击选择启动项时 更新选中的坐标
  const i = launchData.value.findIndex(item => item.id === props.item.id);
  if (i === -1) return;
  const posX = Math.ceil((i + 1) / gridRowMaxItem.value);
  const posY = (i + 1) % gridRowMaxItem.value || gridRowMaxItem.value;
  activeCursorX.value = posX;
  activeCursorY.value = posY;
  activeLaunchItem.value = props.item;

  // 当用户同时按下ctrl + 点击时为复选操作
  // if (e?.ctrlKey) {
  //   // 判断当前按下的启动项是否已经在复选列表中 如果存在则从复选列表中移除
  //   if (activeItemIds.value.includes(props.item.id)) {
  //     const index = activeItems.value?.findIndex(item => item.id === props.item.id);

  //     if (!index || index === -1) return;
  //     activeItems.value?.splice(index, 1);
  //   } else {
  //     activeItems.value?.push(props.item);
  //   }
  // } else {
  //   activeItems.value = [props.item];
  // }
}

const { dialog } = useNaiveUiApi();
async function handleDelete() {
  // 当分类 所属分类关联了目录时 禁止删除操作
  // if (appConfigStore.confirmBeforeDelete) {
  //   const tip = `是否删除 ${props.item.name} ?`;
  //   const answer = await ask(tip, {
  //     title: '删 除',
  //     kind: 'warning',
  //   });
  //   console.log(`%c answer ----`, 'color: #fff;background-color: #000;font-size: 18px', answer);
  //   if (!answer) return;
  // }

  const answer = await new Promise(resolve => {
    dialog.warning({
      title: '提示',
      content: `是否删除 ${props.item.name} ?`,
      positiveText: '确 定',
      negativeText: '取 消',
      draggable: true,
      onPositiveClick: async () => resolve(true),
      onNegativeClick: () => resolve(false),
    });
  });
  if (!answer) return;

  await deleteLaunch(props.item.id);
  EventBus.emit(AppEvent.UPDATE_LAUNCH_LIST);
  if (props.item.id === activeLaunchItem.value?.id) activeLaunchItem.value = null;
}

const el = useTemplateRef('itemRef');
function scrollItemIntoView() {
  if (!el.value) return;

  el.value.scrollIntoView({
    behavior: 'smooth', // 改成 auto 可立即滚动
    block: 'nearest', // 只在超出可视区时滚动
    inline: 'nearest',
  });
  el.value?.focus();
}

defineExpose({ handleEditName, handleDelete, scrollItemIntoView });
</script>

<style scoped lang="scss">
.item {
  border-bottom: none;
}
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
