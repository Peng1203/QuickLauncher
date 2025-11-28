<template>
  <n-layout class="w-full h-full">
    <Header />

    <n-layout
      has-sider
      style="height: calc(600px - 32px)"
      class="container-layout"
    >
      <Sidebar />

      <n-layout-content
        ref="launchListContainerRef"
        tabindex="-1"
        class="overflow-auto m-1"
      >
        <!-- TODO 支持启动项拖拽 控制 order_index 字段 以及拖拽分类 -->
        <VueDraggable
          v-if="launchData.length"
          v-model="launchData"
          :animation="150"
          ghost-class="ghost"
          group="people"
          class="grid grid-cols-6 draggable"
          @contextmenu.prevent.stop="handleShowListContextMenu"
        >
          <ListItem
            v-for="item in launchData"
            :key="item.id"
            v-model="activeItem"
            :item="item"
            :icon="item.icon!"
            :name="item.name"
          />
        </VueDraggable>

        <div
          v-else
          class="w-full h-full flex items-center justify-center text-gray-500 text-lg"
        >
          拖动文件到该区域
        </div>
      </n-layout-content>
    </n-layout>

    <!-- 启动项列表 空白处右键菜单 -->
    <ListContextMenu
      v-model="contextMenuVisible"
      :position="contextMenuPosition"
    />

    <!-- 新增/编辑 启动项 -->
    <OperationLaunchModal v-model="launchModalStatus" />

    <OperationCategoryModal v-model="categoryModalStatus" />
  </n-layout>
</template>

<script setup lang="ts">
import { LogicalPosition } from '@tauri-apps/api/dpi';
import {
  getCurrentWebviewWindow,
  WebviewWindow,
} from '@tauri-apps/api/webviewWindow';
import { storeToRefs } from 'pinia';
import { ref } from 'vue';
import { VueDraggable } from 'vue-draggable-plus';
import { addLaunch, getFileInfo } from '@/api';
import ListContextMenu from '@/components/ListContextMenu.vue';
import ListItem from '@/components/ListItem.vue';
import OperationLaunchModal from '@/components/OperationLaunchModal.vue';
import { useAppConfig } from '@/composables/useAppConfig';
import { useAppConfigActions } from '@/composables/useAppConfigActions';
import { useLoadConfig } from '@/composables/useLoadConfig';
import { AppEvent } from '@/constant';
import { useStore } from '@/store/useStore';
import { EventBus } from '@/utils/eventBus';
import Sidebar from './components/Sidebar.vue';

const store = useStore();
const { launchData, activeCategory, categoryData } = storeToRefs(store);

const { appConfigStore } = useAppConfig();

const launchModalStatus = ref(false);
const categoryModalStatus = ref(false);

const currentWindow = getCurrentWebviewWindow();
const transparentDragWindow = ref<WebviewWindow | null>(null);

currentWindow.onDragDropEvent(async e => {
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

let timer: any;
currentWindow.onMoved(({ payload: position }) => {
  clearTimeout(timer);
  timer = setTimeout(() => {
    appConfigStore.mainWindowPositionX = position.x;
    appConfigStore.mainWindowPositionY = position.y;
  }, 100);
});

store.getLaunchData();

const contextMenuVisible = ref<boolean>(false);
const contextMenuPosition = ref({ x: 0, y: 0 });

function handleShowListContextMenu(e: MouseEvent) {
  EventBus.emit(AppEvent.CLOSE_CONTEXT_MENU);

  setTimeout(() => {
    nextTick(() => {
      contextMenuVisible.value = true;
      contextMenuPosition.value = { x: e.clientX, y: e.clientY };
    });
  }, 100);
}

const activeItem = ref<LaunchItem>();

EventBus.listen(AppEvent.UPDATE_LAUNCH_LIST, store.getLaunchData);

watch(
  () => categoryData.value,
  val => {
    // eslint-disable-next-line no-useless-return
    if (!val.length) return;
  },
  { deep: true, immediate: true }
);

useLoadConfig();

useAppConfigActions().initMainWindowShortcutKey();
// watchImmediate(
//   'C:\\Users\\Mayn\\Desktop\\FTTH APP截图',
//   event => {
//     console.log(`%c event ----`, 'color: #fff;background-color: #000;font-size: 18px', event)
//   },
//   { baseDir: BaseDirectory.Desktop }
// )

// listen<AppConfigState>(AppEvent.UPDATE_APP_CONFIG_DATA, val => {
//   for (const key in val.payload) {
//     // @ts-ignore
//     appConfigStore[key] = val.payload[key]
//   }
// })

const launchListContainerRef = useTemplateRef('launchListContainerRef');
function showTransparentDragWindow() {
  transparentDragWindow.value?.show();
  transparentDragWindow.value?.setPosition(
    new LogicalPosition(
      appConfigStore.mainWindowPositionX + 192 + 12,
      appConfigStore.mainWindowPositionY + 32 + 4
    )
  );
}

onMounted(async () => {
  transparentDragWindow.value = await WebviewWindow.getByLabel(
    'transparentDrag'
  );

  // launchListContainerRef.value?.$el?.addEventListener(
  //   'dragenter',
  //   showTransparentDragWindow
  // );

  window.addEventListener('dragover', e => {
    e.preventDefault();
  });

  window.addEventListener('drop', e => {
    e.preventDefault();
  });
});

onUnmounted(() => {
  launchListContainerRef.value?.$el?.removeEventListener(
    'dragenter',
    showTransparentDragWindow
  );
});
</script>

<style scoped lang="scss">
.container-layout {
  ::v-deep(.n-layout-scroll-container:has(> aside)) {
    overflow-y: hidden !important;
  }
}
</style>
