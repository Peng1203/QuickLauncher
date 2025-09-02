<template>
  <aside
    tabindex="-1"
    class="fixed top-8 left-0 bottom-0 w-48 bg-white border-r border-gray-200 flex flex-col py-3 px-2 z-10"
    @contextmenu.prevent.stop="handleShowCategoryContextMenu"
  >
    <nav class="flex-1 flex flex-col gap-1">
      <button
        tabindex="-1"
        :class="[
          'text-left px-4 py-2 rounded-lg transition font-medium cursor-pointer',
          1 ? 'bg-gray-100 text-blue-600' : 'hover:bg-gray-50 text-gray-700',
        ]"
      >
        {{ `默 认` }}
      </button>

      <!-- <button @click="unregisterAll()" >取消所有快捷键</button> -->
    </nav>
  </aside>

  <!-- 新建/编辑 分类菜单 -->
  <CategoryContextMenu
    v-model="contextMenuVisible"
    :position="contextMenuPosition"
    @add="handleOpenAddCategory"
  />

  <!--  -->
</template>

<script setup lang="ts">
import CategoryContextMenu from '@/components/CategoryContextMenu.vue'
import { EventBus } from '@/utils/eventBus'
import { AppEvent } from '@/constant'

const contextMenuVisible = ref<boolean>(false)
const contextMenuPosition = ref({ x: 0, y: 0 })

const handleShowCategoryContextMenu = (e: MouseEvent) => {
  EventBus.emit(AppEvent.CLOSE_CONTEXT_MENU)

  setTimeout(() => {
    nextTick(() => {
      contextMenuVisible.value = true
      contextMenuPosition.value = { x: e.clientX, y: e.clientY }
    })
  }, 100)
}

const handleOpenAddCategory = () => {}
</script>
