<template>
  <n-layout-content
    id="list-container"
    ref="launchListContainerRef"
    tabindex="-1"
    class="overflow-auto m-1"
    @contextmenu.prevent.stop="handleShowListContextMenu"
  >
    <!-- <transition-group name="list" tag="div" class="relative"> </transition-group> -->
    <VueDraggable
      v-if="launchData.length"
      v-model="launchData"
      ghost-class="opacity-50"
      :animation="200"
      :group="{ name: 'launch', pull: 'clone', put: false }"
      :disabled="isConrrelationDir"
      :class="
        isListMode
          ? 'flex flex-col divide-y divide-border mb-6'
          : 'grid grid-cols-2 sm:grid-cols-4 md:grid-cols-6 draggable gap-0.5'
      "
      @start="handleDragStart"
      @end="handleDragEnd"
    >
      <ListItem
        v-for="item in launchData"
        :key="item.id"
        :ref="(el) => (itemRefs[`${item.id}`] = el)"
        :item="item"
        :icon="item.icon!"
        :name="item.name"
      />
    </VueDraggable>

    <div
      v-else
      class="w-full h-full flex items-center justify-center text-muted-foreground text-lg"
    >
      {{ isConrrelationDir ? t("main.emptyFolder") : t("main.dragHere") }}
    </div>
  </n-layout-content>

  <!-- 启动项列表 空白处右键菜单 -->
  <ListContextMenu v-model="contextMenuVisible" :position="contextMenuPosition" />
</template>

<script setup lang="ts">
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { isEmpty } from "lodash-es";
import { storeToRefs } from "pinia";
import { nextTick, ref } from "vue";
import { VueDraggable } from "vue-draggable-plus";
import { addLaunch, getFileInfo, updateLaunch } from "@/api";
import ListContextMenu from "@/components/ListContextMenu.vue";
import ListItem from "@/components/ListItem.vue";
import { useCategoryCorrelationDir, useLaunchDrag } from "@/composables";
import { AppEvent } from "@/constant";
import { t } from "@/i18n";
import { useStore } from "@/store/useStore";
import { EventBus } from "@/utils/eventBus";

const store = useStore();
const { launchData, activeCategory, activeCategoryItem, activeLaunchItem } = storeToRefs(store);
const { isConrrelationDir } = useCategoryCorrelationDir();
const { draggedItem } = useLaunchDrag();
const currentWindow = getCurrentWebviewWindow();
const isListMode = computed(() => activeCategoryItem.value.layout === "list");

currentWindow.onDragDropEvent(async (e) => {
  // 防止在关联目录分类下手动拖拽添加启动项
  if (isConrrelationDir.value) return;

  // TODO 分类对话框打开
  if (e.payload.type === "drop") {
    const addLaunchTasks = (e.payload.paths ?? []).map(async (path) => {
      const fileInfo = await getFileInfo(path);

      const item: NewLaunchItem = {
        name: fileInfo.name,
        lnk_name: fileInfo.lnk_name,
        path: fileInfo.path,
        type: fileInfo.type,
        icon: fileInfo.icon,
        // category_id: null,
        hotkey: "",
        hotkey_global: false,
        keywords: "",
        start_dir: fileInfo.start_dir,
        remarks: fileInfo.remarks || "",
        args: fileInfo.args || "",
        run_as_admin: false,
        order_index: 0,
        enabled: true,
        category_id: activeCategory.value,
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

/** 拖拽开始：记录当前拖拽的启动项 */
function handleDragStart(evt: any) {
  const index = evt.oldIndex;
  draggedItem.value = launchData.value[index] ?? null;
}

/** 同列表内拖拽排序结束 */
async function handleDragEnd(_evt: any) {
  // 如果 draggedItem 已被 Sidebar 的 drop handler 消费，跳过排序更新
  if (!draggedItem.value) return;

  // 同一列表内排序：更新 order_index
  const tasks = launchData.value.map((item, index) =>
    updateLaunch({ ...item, order_index: index }),
  );
  await Promise.all(tasks);
  draggedItem.value = null;
}

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
  (val) => {
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
