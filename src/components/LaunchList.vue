<template>
  <n-layout-content
    ref="launchListContainerRef"
    tabindex="-1"
    class="overflow-auto m-1"
    @contextmenu.prevent.stop="handleShowListContextMenu"
  >
    <!-- {{ isConrrelationDir }} -->
    <!-- TODO 支持启动项拖拽 控制 order_index 字段 以及拖拽分类 -->
    <VueDraggable
      v-if="launchData.length"
      v-model="launchData"
      :animation="150"
      ghost-class="ghost"
      group="people"
      class="grid grid-cols-6 draggable gap-0.5"
    >
      <ListItem
        v-for="item in launchData"
        :key="item.id"
        v-model="activeItem"
        :item="item"
        :icon="item.icon!"
        :name="item.name"
        :active-category-item="activeCategoryItem!"
      />
    </VueDraggable>

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
import { storeToRefs } from 'pinia';
import { ref } from 'vue';
import { VueDraggable } from 'vue-draggable-plus';
import { addLaunch, getFileInfo } from '@/api';
import ListContextMenu from '@/components/ListContextMenu.vue';
import ListItem from '@/components/ListItem.vue';
import { useCategoryCorrelationDir } from '@/composables/useCategoryCorrelationDir';
import { AppEvent } from '@/constant';
import { useStore } from '@/store/useStore';
import { EventBus } from '@/utils/eventBus';

const store = useStore();
const { launchData, activeCategory } = storeToRefs(store);

const { isConrrelationDir, activeCategoryItem } = useCategoryCorrelationDir();

const launchModalStatus = ref(false);
const categoryModalStatus = ref(false);
const currentWindow = getCurrentWebviewWindow();

currentWindow.onDragDropEvent(async e => {
  // 防止在关联目录分类下手动拖拽添加启动项
  if (isConrrelationDir.value) return;
  // 当添加对话框打开时不触发后续操作 防止和对话框拖拽事件相互影响
  if (launchModalStatus.value || categoryModalStatus.value) return;
  // TODO 分类对话框打开
  if (e.payload.type === 'drop') {
    const addLaunchTasks = (e.payload.paths ?? []).map(async path => {
      const fileInfo = await getFileInfo(path);

      const item: NewLaunchItem = {
        name: fileInfo.name,
        path: fileInfo.path,
        type: fileInfo.type,
        icon: fileInfo.icon,
        // category_id: null,
        hotkey: '',
        hotkey_global: 0,
        keywords: '',
        start_dir: fileInfo.start_dir,
        remarks: fileInfo.remarks || '',
        args: fileInfo.args || '',
        run_as_admin: 0,
        order_index: 0,
        enabled: 1,
        category_id: activeCategory.value === -1 ? null : activeCategory.value,
        subcategory_id: null,
        extension: fileInfo.extension,
      };

      // // 添加记录
      await addLaunch(item);
    });

    await Promise.all(addLaunchTasks);

    // 刷新列表
    store.getLaunchData();
  }
});

store.getLaunchData();

const contextMenuVisible = ref<boolean>(false);
const contextMenuPosition = ref({ x: 0, y: 0 });
const activeItem = ref<LaunchItem[]>([]);

function handleShowListContextMenu(e: MouseEvent) {
  EventBus.emit(AppEvent.CLOSE_CONTEXT_MENU);

  setTimeout(() => {
    nextTick(() => {
      contextMenuVisible.value = true;
      contextMenuPosition.value = { x: e.clientX, y: e.clientY };
    });
  }, 100);
}
</script>

<style scoped lang="scss"></style>
