<template>
  <main tabindex="-1" class="pl-48 pt-8 h-screen overflow-auto">
    <div
      class="w-full h-full max-w-5xl mx-auto p-1"
      @contextmenu.prevent.stop="handleShowListContextMenu"
    >
      <!-- TODO 支持启动项拖拽 控制 order_index 字段 以及拖拽分类 -->
      <div v-if="dataList.length" class="grid grid-cols-6">
        <ListItem
          v-for="item in dataList"
          :key="item.id"
          v-model="activeItem"
          :item="item"
          :icon="item.icon!"
          :name="item.name"
        />
      </div>

      <div
        v-else
        class="w-full h-full flex items-center justify-center text-gray-500 text-lg"
      >
        拖动文件到该区域
      </div>
    </div>
  </main>
</template>

<script setup lang="ts">
import { AppEvent } from '@/constant';
import { EventBus } from '@/utils/eventBus';

const activeItem = ref<LaunchItem>();

const dataList = ref();

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
</script>
