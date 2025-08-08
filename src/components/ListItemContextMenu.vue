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
import { openPath } from '@/api'
import { ref, onMounted, onUnmounted } from 'vue'

export interface MenuAction {
  label: string
  onClick: () => void
}

const props = defineProps<{
  visible: boolean
  position: { x: number; y: number }
  onClose: () => void
  item: LaunchItem
  itemPath: string
  itemName: string
  extraItems?: MenuAction[]
}>()

const emit = defineEmits(['close'])

const handleClick = (item: MenuAction) => {
  item.onClick()
  props.onClose()
}

// 默认菜单项
const menuItems = ref<MenuAction[]>([
  {
    label: '以管理员身份运行',
    onClick: () => openPath(props.item.path),
  },
  {
    label: '打开所在位置',
    onClick: () => openPath(props.item.path),
  },
  {
    label: '复制路径',
    onClick: async () => {
      // await window.__TAURI__.clipboard.writeText(props.itemPath);
    },
  },
  {
    label: '重命名',
    onClick: () => {
      console.log('重命名')
    },
  },
  {
    label: '删除',
    onClick: () => {
      console.log('删除')
    },
  },
  {
    label: '设置开机启动',
    onClick: () => {
      console.log('设置开机启动')
    },
  },
  {
    label: '移除启动项',
    onClick: () => {
      console.log('移除启动项')
    },
  },
])

// 自动监听点击窗口其他地方关闭菜单
const handleOutsideClick = (e: MouseEvent) => {
  if (!(e.target as HTMLElement).closest('.context-menu')) {
    props.onClose()
  }
}

onMounted(() => {
  window.addEventListener('click', handleOutsideClick)
})
onUnmounted(() => {
  window.removeEventListener('click', handleOutsideClick)
})
</script>

<style scoped>
/* 可选：添加动画、图标等美化 */
</style>
