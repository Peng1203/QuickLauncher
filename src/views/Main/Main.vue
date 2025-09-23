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
          v-if="launchData.length"
        >
          <ListItem
            v-model="activeItem"
            :item="item"
            :key="item.id"
            :icon="item.icon!"
            :name="item.name"
            v-for="item in launchData"
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
    <OperationLaunchModal v-model="launchModalStatus" />

    <OperationCategoryModal v-model="categoryModalStatus" />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import ListItem from '@/components/ListItem.vue'
import { getFileInfo, addLaunch } from '@/api'
import ListContextMenu from '@/components/ListContextMenu.vue'
import OperationLaunchModal from '@/components/OperationLaunchModal.vue'
import Sidebar from './components/Sidebar.vue'
import { EventBus } from '@/utils/eventBus'
import { AppEvent } from '@/constant'
import { useStore } from '@/store/useStore'
import { storeToRefs } from 'pinia'
import { useAppConfig } from '@/composables/useAppConfig'
import { emit, listen } from '@tauri-apps/api/event'
import { useUpdateAppConfig } from '@/composables/useUpdateAppConfig'

useUpdateAppConfig()
const store = useStore()
const { launchData, activeCategory, categoryData } = storeToRefs(store)

const { appConfigStore } = useAppConfig()

const launchModalStatus = ref(false)
const categoryModalStatus = ref(false)

const currentWindow = getCurrentWebviewWindow()

currentWindow.onDragDropEvent(async e => {
  // 当添加对话框打开时不触发后续操作 防止和对话框拖拽事件相互影响
  if (launchModalStatus.value || categoryModalStatus.value) return
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
        category_id: activeCategory.value || null,
        subcategory_id: null,
        extension: fileInfo.extension,
      }
      // // 添加记录
      await addLaunch(item)
    })

    await Promise.all(addLaunchTasks)

    // 刷新列表
    store.getLaunchData()
  }
})

let timer: any
currentWindow.onMoved(({ payload: position }) => {
  clearTimeout(timer)
  timer = setTimeout(() => {
    appConfigStore.mainWindowPositionX = position.x
    appConfigStore.mainWindowPositionY = position.y
  }, 100)
})

store.getLaunchData()

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

EventBus.listen(AppEvent.UPDATE_LAUNCH_LIST, store.getLaunchData)

watch(
  () => categoryData.value,
  val => {
    if (!val.length) return
  },
  { deep: true, immediate: true }
)

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
</script>
