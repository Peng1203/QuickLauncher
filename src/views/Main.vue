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

      <ListContextMenu
        v-model="contextMenuVisible"
        :position="contextMenuPosition"
        @add="handleOpenAddLaunch"
        @refresh="getData"
      />

      <OperationLaunchModal
        :editItem="editItem"
        v-model="modalStatus"
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
import OperationLaunchModal from '@/components/OperationLaunchModal.vue'
import { EventBus } from '@/utils/eventBus'
import { AppEvent } from '@/constant'

const modalStatus = ref(false)

const editItem = ref<LaunchItem>()

getCurrentWebviewWindow().onDragDropEvent(async e => {
  // 当添加对话框打开时不触发后续操作 防止和对话框拖拽事件相互影响
  if (modalStatus.value) return
  if (e.payload.type === 'drop') {
    const addLaunchTasks = (e.payload.paths ?? []).map(async path => {
      const fileInfo = await getFileInfo(path)

      const item: NewLaunchItem = {
        name: fileInfo.name,
        path: fileInfo.path,
        type: fileInfo.type,
        icon: fileInfo.icon,
        // category_id: null,
        hotkey: '',
        hotkey_global: 0,
        keywords: '',
        start_dir: '',
        remarks: fileInfo.remarks || '',
        args: fileInfo.args || '',
        run_as_admin: 0,
        order_index: 0,
        enabled: 1,
        category_id: null,
        extension: fileInfo.extension,
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
  EventBus.emit(AppEvent.CLOSE_CONTEXT_MENU)

  setTimeout(() => {
    nextTick(() => {
      contextMenuVisible.value = true
      contextMenuPosition.value = { x: e.clientX, y: e.clientY }
    })
  }, 100)
}

const handleOpenAddLaunch = () => {
  editItem.value = undefined
  modalStatus.value = true
}

const activeItem = ref<LaunchItem>()

EventBus.listen(AppEvent.UPDATE_LAUNCH_LIST, getData)

EventBus.listen<LaunchItem>(AppEvent.EDIT_LAUNCH, item => {
  editItem.value = item
  modalStatus.value = true
})
</script>
