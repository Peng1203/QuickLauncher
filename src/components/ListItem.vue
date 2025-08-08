<template>
  <div
    class="item flex flex-col items-center min-w-20 w-full h-18 cursor-pointer select-none hover:bg-opacity-20 rounded pt-0.5 pb-0.5"
    :title="name"
    tabindex="0"
    @dblclick="handleRun"
    @keydown="handleKeydown"
    @contextmenu.prevent="handleShowContextMenu"
  >
    <img
      :src="icon"
      alt="icon"
      class="object-contain pointer-events-none"
    />
    <!-- v-if="!isEdit" -->
    <!-- {{ isEdit }} -->

    <span
      ref="nameRef"
      :contenteditable="isEdit"
      class="text-xs text-center text-black px-1 w-full pointer-events-none line-clamp-2 mt-0.5 leading-normal"
    >
      {{ name }}
      <!-- -- {{ newName }} -->
    </span>

    <LaunchItemContextMenu
      :item="item"
      :visible="menuVisible"
      :position="menuPosition"
      :item-path="item.path"
      :item-name="item.name"
      :on-close="() => (menuVisible = false)"
    />
  </div>
</template>

<script setup lang="ts">
import { useLaunchAction } from '@/composables/useLaunchAction'
import LaunchItemContextMenu from './ListItemContextMenu.vue'

const { runLaunch } = useLaunchAction()

const props = defineProps<{
  icon: string
  name: string
  item: LaunchItem
}>()

const isEdit = ref<boolean>(false)
const newName = ref<string>(props.name)

const handleRun = () => {
  runLaunch(props.item.id)
}

const handleKeydown = (e: KeyboardEvent) => {
  const { keyCode, key } = e

  console.log(`%c keyCode ----`, 'color: #fff;background-color: #000;font-size: 18px', keyCode, key)
  switch (key) {
    case 'F2': // 113
      handleEditName()
      break
    case 'Enter': // 13
      handleSaveEditName()
      e.preventDefault()
      break
    case 'Escape': // 27
      handleCancelEditName()
      break
    default:
      break
  }
}

const nameRef = useTemplateRef('nameRef')
const handleEditName = () => {
  if (isEdit.value) return
  newName.value = props.name
  isEdit.value = true
  nextTick(() => {
    nameRef.value?.focus()
    const range = document.createRange()
    range.selectNodeContents(nameRef.value as any) // 选择元素内的所有内容
    const selection = window.getSelection()
    selection?.removeAllRanges() // 清除之前的选区
    selection?.addRange(range)

    // setInputWidth()
  })
}

const handleCancelEditName = () => {
  isEdit.value = false
}

const handleSaveEditName = async () => {
  console.log(
    `%c 保存编辑名称 ----`,
    'color: #fff;background-color: #000;font-size: 18px',
    nameRef.value?.textContent
  )
  await saveEditName()

  isEdit.value = false
}

const saveEditName = async () => {
  try {
  } catch (e) {
    console.log('e', e)
  }
}

const menuVisible = ref(false)
const menuPosition = ref({ x: 0, y: 0 })

const handleShowContextMenu = (e: MouseEvent) => {
  menuVisible.value = true
  menuPosition.value = { x: e.clientX, y: e.clientY }
}
</script>

<style scoped>
.item:hover {
  background-color: #f5f5f5;
}
</style>
