<template>
  <n-layout-content
    id="list-container"
    ref="launchListContainerRef"
    tabindex="-1"
    class="overflow-auto m-1"
    @contextmenu.prevent.stop="handleShowListContextMenu"
  >
    <transition-group
      v-if="launchData.length"
      name="list"
      tag="div"
      class="relative"
      :class="
        isListMode
          ? 'flex flex-col divide-y divide-gray-100 mb-6'
          : 'grid grid-cols-2 sm:grid-cols-4 md:grid-cols-6 draggable gap-0.5'
      "
    >
      <template
        v-for="item in launchData"
        :key="item.id"
      >
        <ListItem
          :ref="el => (itemRefs[`${item.id}`] = el)"
          :item="item"
          :icon="item.icon!"
          :name="item.name"
          :active-category-item="activeCategoryItem!"
        />
      </template>
    </transition-group>

    <div
      v-else
      class="w-full h-full flex items-center justify-center text-gray-500 text-lg"
    >
      {{ isConrrelationDir ? '空文件夹' : '拖动文件到该区域' }}
    </div>
  </n-layout-content>

  <!-- 启动项列表 空白处右键菜单 -->
  <ListContextMenu
    v-model="contextMenuVisible"
    :position="contextMenuPosition"
  />
</template>

<script setup lang="ts">
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { isEmpty } from 'lodash-es';
import { storeToRefs } from 'pinia';
import { nextTick, ref } from 'vue';
// import { VueDraggable } from 'vue-draggable-plus';
import { addLaunch, getFileInfo } from '@/api';
import ListContextMenu from '@/components/ListContextMenu.vue';
import ListItem from '@/components/ListItem.vue';
import { useCategoryCorrelationDir } from '@/composables/useCategoryCorrelationDir';
import { AppEvent } from '@/constant';
import { useStore } from '@/store/useStore';
import { EventBus } from '@/utils/eventBus';

const store = useStore();
const { launchData, activeCategory, activeCategoryItem, activeLaunchItem } = storeToRefs(store);
const { isConrrelationDir } = useCategoryCorrelationDir();
const currentWindow = getCurrentWebviewWindow();
const isListMode = computed(() => activeCategoryItem.value.layout === 'list');

currentWindow.onDragDropEvent(async e => {
  // 防止在关联目录分类下手动拖拽添加启动项
  if (isConrrelationDir.value) return;

  // TODO 分类对话框打开
  if (e.payload.type === 'drop') {
    const addLaunchTasks = (e.payload.paths ?? []).map(async path => {
      const fileInfo = await getFileInfo(path);

      const item: NewLaunchItem = {
        name: fileInfo.name,
        lnk_name: fileInfo.lnk_name,
        path: fileInfo.path,
        type: fileInfo.type,
        icon: fileInfo.icon,
        // category_id: null,
        hotkey: '',
        hotkey_global: false,
        keywords: '',
        start_dir: fileInfo.start_dir,
        remarks: fileInfo.remarks || '',
        args: fileInfo.args || '',
        run_as_admin: false,
        order_index: 0,
        enabled: true,
        category_id: activeCategory.value === -1 ? null : activeCategory.value,
        subcategory_id: null,
        extension: fileInfo.extension,
      };
      await addLaunch(item);
    });

    await Promise.all(addLaunchTasks);
    // 刷新列表
    store.getLaunchData();
  }
});

const contextMenuVisible = ref<boolean>(false);
const contextMenuPosition = ref({ x: 0, y: 0 });

function handleShowListContextMenu(e: MouseEvent) {
  // EventBus.emit(AppEvent.CLOSE_CONTEXT_MENU);

  setTimeout(() => {
    nextTick(() => {
      contextMenuVisible.value = true;
      contextMenuPosition.value = { x: e.clientX, y: e.clientY };
    });
  }, 100);
}

const itemRefs = shallowRef<any>({});
EventBus.listen(AppEvent.LAUNCH_RENAME, () => {
  if (isEmpty(activeLaunchItem.value)) return;
  const itemRef = itemRefs.value[activeLaunchItem.value.id];
  itemRef?.handleEditName();
});

// 处理窗口内部 Delete 快捷键
EventBus.listen(AppEvent.DELETE_LAUNCH, () => {
  if (isEmpty(activeLaunchItem.value)) return;
  const itemRef = itemRefs.value[activeLaunchItem.value.id];
  itemRef?.handleDelete();
});

watch(
  () => activeLaunchItem.value,
  val => {
    console.log(`%c val ----`, 'color: #fff;background-color: #000;font-size: 18px', val);
    if (val) {
      const itemRef = itemRefs.value[val.id];
      itemRef?.scrollItemIntoView();
    }
  },
);
</script>

<style scoped lang="scss">
.list-move {
  transition: all 0.25s ease;
}

.list-enter-active,
.list-leave-active {
  transition: all 0.2s ease;
}

.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: scale(0.95);
}
</style>
