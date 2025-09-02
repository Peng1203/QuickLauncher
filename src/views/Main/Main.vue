<template>
  <div class="h-screen bg-white">
    <!-- 顶部固定自定义拖拽导航栏 -->
    <Header />
    <!-- 左侧分类栏 -->
    <Sidebar />

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
    </main>

    <!-- 启动项列表 空白处右键菜单 -->
    <ListContextMenu
      v-model="contextMenuVisible"
      :position="contextMenuPosition"
    />

    <!-- 新增/编辑 启动项 -->
    <OperationLaunchModal
      v-model="modalStatus"
      @refresh="getData"
    />

    <OperationCategoryModal />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import ListItem from '@/components/ListItem.vue'
import { getFileInfo, addLaunch, getLaunchs } from '@/api'
import ListContextMenu from '@/components/ListContextMenu.vue'
import OperationLaunchModal from '@/components/OperationLaunchModal.vue'

import Sidebar from './components/Sidebar.vue'
import { EventBus } from '@/utils/eventBus'
import { AppEvent } from '@/constant'

const modalStatus = ref(false)

getCurrentWebviewWindow().onDragDropEvent(async e => {
  // 当添加对话框打开时不触发后续操作 防止和对话框拖拽事件相互影响
  if (modalStatus.value) return
  // TODO 分类对话框打开
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

const activeItem = ref<LaunchItem>()

EventBus.listen(AppEvent.UPDATE_LAUNCH_LIST, getData)
</script>
