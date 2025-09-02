<template>
  <div
    v-if="visible"
    ref="menuRef"
    class="fixed z-50 rounded-lg shadow-lg bg-white border border-gray-200"
    :style="{ top: `${calcPosition.y}px`, left: `${calcPosition.x}px`, width: MENU_WIDTH + 'px' }"
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
import { AppEvent, MENU_WIDTH } from '@/constant'
import { EventBus } from '@/utils/eventBus'
import { ref, onMounted, onUnmounted } from 'vue'

export interface MenuAction {
  label: string
  onClick: () => void
}

const visible = defineModel<boolean>()

const props = defineProps<{ position: { x: number; y: number } }>()

const emit = defineEmits(['add'])

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
    label: '新建分类',
    onClick: async () => emit('add'),
  },
])

const menuRef = useTemplateRef('menuRef')
// 自动监听点击窗口其他地方关闭菜单
const handleOutsideClick = (e: MouseEvent) => {
  if (menuRef.value && !menuRef.value.contains(e.target as Node)) {
    handleClose()
  }

  // if (!(e.target as HTMLElement).closest('.context-menu')) {
  //   handleClose()
  // }
}

// 菜单距离窗口边的距离
const VIEWPORT_MARGIN = 5

// 计算出菜单出现的 x y 位置
const calcPosition = computed(() => {
  let x = props.position.x
  let y = props.position.y

  if (props.position.x + MENU_WIDTH > window.innerWidth) {
    x = window.innerWidth - MENU_WIDTH - VIEWPORT_MARGIN
  }

  if (menuRef.value) {
    if (props.position.y + menuRef.value.offsetHeight > window.innerHeight) {
      y = window.innerHeight - menuRef.value.offsetHeight - VIEWPORT_MARGIN
    }
  }

  return { x, y }
})

onMounted(() => {
  EventBus.listen(AppEvent.CLOSE_CONTEXT_MENU, handleClose)

  window.addEventListener('click', handleOutsideClick)
  window.addEventListener('contextmenu', handleOutsideClick)
  window.addEventListener('scroll', handleClose, true)
})
onUnmounted(() => {
  window.removeEventListener('click', handleOutsideClick)
  window.removeEventListener('contextmenu', handleOutsideClick)
  window.removeEventListener('scroll', handleClose, true)
})
</script>
