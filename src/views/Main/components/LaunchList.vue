<template>
  <main
    tabindex="-1"
    class="pl-48 pt-8 h-screen overflow-auto"
  >
    <div
      class="w-full h-full max-w-5xl mx-auto p-1"
      @contextmenu.prevent.stop="handleShowListContextMenu"
    >
      <!-- TODO 支持启动项拖拽 控制 order_index 字段 以及拖拽分类 -->
      <div
        class="grid grid-cols-6"
        v-if="dataList.length"
      >
        <ListItem
          v-model="activeItem"
          :item="item"
          :key="item.id"
          :icon="item.icon!"
          :name="item.name"
          v-for="item in dataList"
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
import { EventBus } from '@/utils/eventBus'
import { AppEvent } from '@/constant'

const activeItem = ref<LaunchItem>()

const contextMenuVisible = ref<boolean>(false)
const contextMenuPosition = ref({ x: 0, y: 0 })

const handleShowListContextMenu = (e: MouseEvent) => {
  EventBus.emit(AppEvent.CLOSE_CONTEXT_MENU)

  setTimeout(() => {
    nextTick(() => {
      contextMenuVisible.value = true
      contextMenuPosition.value = { x: e.clientX, y: e.clientY }
    })
  }, 100)
}
</script>
