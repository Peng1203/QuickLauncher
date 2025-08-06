<template>
  <div
    class="item flex flex-col items-center min-w-20 w-full h-18 cursor-pointer select-none hover:bg-opacity-20 rounded pt-0.5 pb-0.5"
    :title="name"
    @dblclick="handleRun"
    @contextmenu.prevent="handleShowContextMenu"
    @keydown.f2="handleEditName"
  >
    <img
      :src="icon"
      alt="icon"
      class="object-contain pointer-events-none"
    />
    <span
      v-if="!isEdit"
      class="text-xs text-center text-black px-1 w-full pointer-events-none line-clamp-2 mt-0.5 leading-normal"
    >
      {{ name }}
      <!-- -- {{ newName }} -->
    </span>
    <input
      v-else
      ref="inputRef"
      v-model="newName"
      @blur="handleSaveEditName"
      @keydown.enter.prevent="handleSaveEditName"
      @keydown.esc.prevent="isEdit = false"
      class="border rounded"
    />

    <LaunchItemContextMenu
      :visible="menuVisible"
      :position="menuPosition"
      :item="item"
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
  console.log(`%c 112 ----`, 'color: #fff;background-color: #000;font-size: 18px', 112)
  runLaunch(props.item.id)
}

const inputRef = useTemplateRef('inputRef')
const handleEditName = () => {
  newName.value = props.name
  isEdit.value = true
  nextTick(() => inputRef.value?.focus())
}

const handleSaveEditName = () => {
  console.log(`%c 编辑名称 ----`, 'color: #fff;background-color: #000;font-size: 18px')
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
