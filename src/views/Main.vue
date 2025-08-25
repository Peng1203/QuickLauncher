<template>
  <div class="h-screen bg-white">
    <!-- 顶部固定自定义拖拽导航栏 -->
    <Header />
    <!-- 左侧分类栏 -->
    <aside
      tabindex="-1"
      class="fixed top-8 left-0 bottom-0 w-48 bg-white border-r border-gray-200 flex flex-col py-3 px-2 z-10"
    >
      <nav class="flex-1 flex flex-col gap-1">
        <button
          tabindex="-1"
          :class="[
            'text-left px-4 py-2 rounded-lg transition font-medium',
            1 ? 'bg-gray-100 text-blue-600' : 'hover:bg-gray-50 text-gray-700',
          ]"
        >
          {{ `默认` }}
        </button>

        <!-- <button @click="unregisterAll()" >取消所有快捷键</button> -->
      </nav>
    </aside>

    <!-- 右侧启动项区域 -->
    <main
      tabindex="-1"
      class="pl-48 pt-8 h-screen overflow-auto"
    >
      <div
        class="w-full h-full max-w-5xl mx-auto p-1"
        @contextmenu.prevent="handleShowListContextMenu"
      >
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

      <ListContextMenu
        v-model="contextMenuVisible"
        :position="contextMenuPosition"
        @refresh="getData"
      />
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import ListItem from '@/components/ListItem.vue'
import { getFileInfo, addLaunch, getLaunchs } from '@/api'
import ListContextMenu from '@/components/ListContextMenu.vue'

getCurrentWebviewWindow().onDragDropEvent(async e => {
  if (e.payload.type === 'drop') {
    const addLaunchTasks = (e.payload.paths ?? []).map(async path => {
      const fileInfo = await getFileInfo(path)

      const item: NewLaunchItem = {
        name: fileInfo.name,
        path: fileInfo.path,
        type: fileInfo.type,
        icon: fileInfo.icon,
        // category_id: null,
      }
      // // 添加记录
      await addLaunch(item)
    })

    await Promise.all(addLaunchTasks)

    // 刷新列表
    getData()
  }
})

const dataList = ref<LaunchItem[]>([])

const getData = async () => {
  const data = await getLaunchs()
  dataList.value = data
}

getData()

const contextMenuVisible = ref<boolean>(false)
const contextMenuPosition = ref({ x: 0, y: 0 })

const handleShowListContextMenu = (e: MouseEvent) => {
  contextMenuVisible.value = true
  contextMenuPosition.value = { x: e.clientX, y: e.clientY }
}

const activeItem = ref<LaunchItem>()
</script>
