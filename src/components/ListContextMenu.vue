<template>
  <div
    v-if="visible"
    :style="{ top: `${position.y}px`, left: `${position.x}px` }"
    class="fixed z-50 w-48 rounded-lg shadow-lg bg-white border border-gray-200"
    @click.stop
  >
    <ul class="text-sm text-gray-700">
      <li
        :key="item.label"
        v-for="item in menuItems"
        @click="handleClick(item)"
        class="px-4 py-2 hover:bg-gray-100 cursor-pointer"
      >
        {{ item.label }}
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { addLaunch, getFileInfo } from '@/api'

export interface MenuAction {
  label: string
  onClick: () => void
}

const visible = defineModel<boolean>()

const props = defineProps<{
  position: { x: number; y: number }
}>()

const emit = defineEmits(['refresh'])

const handleClose = () => {
  visible.value = false
}

const handleClick = (item: MenuAction) => {
  item.onClick()
  handleClose()
}

// 默认菜单项
const menuItems = ref<MenuAction[]>([
  {
    label: '新建',
    onClick: async () => {
      const path = await open({
        multiple: false,
        directory: false,
      })
      if (!path) return
      console.log(`%c path ----`, 'color: #fff;background-color: #000;font-size: 18px', path)
      const fileInfo = await getFileInfo(path!)
      const item: NewLaunchItem = {
        name: fileInfo.name,
        path: fileInfo.path,
        type: fileInfo.type,
        icon: fileInfo.icon,
        // category_id: null,
      }
      // // 添加记录
      await addLaunch(item)
      // 通知父组件刷新列表
      emit('refresh')
    },
  },
])

// 自动监听点击窗口其他地方关闭菜单
const handleOutsideClick = (e: MouseEvent) => {
  if (!(e.target as HTMLElement).closest('.context-menu')) {
    handleClose()
  }
}

onMounted(() => {
  window.addEventListener('click', handleOutsideClick)
})
onUnmounted(() => {
  window.removeEventListener('click', handleOutsideClick)
})
</script>
