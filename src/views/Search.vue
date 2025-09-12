<template>
  <!-- border border-gray-100 -->
  <n-input
    tabindex="-1"
    type="text"
    size="medium"
    ref="searchInputRef"
    class="w-full h-full max-h-[45px] resize-none text-sm hover:outline-0 focus-visible:outline-0 border-none bg-white shadow-none rounded-[10px]"
    :class="hasResult ? '!border-b-0 !rounded-b-none' : ''"
    :placeholder="placeholder"
    @keydown="handleKeydown"
    v-model:value="keyword"
  >
    <!-- @keydown.up -->
    <template #prefix>
      <!-- {{ selectedIndex }} -->
      <!-- {{ selectedIndex }}--- {{ resultList.length }}--{{ hasResult }} -->
      <n-icon
        :component="SearchOutline"
        size="22"
      />
    </template>
  </n-input>

  <ul
    tabindex="-1"
    v-if="hasResult"
    class="search-container absolute z-50 w-full overflow-y-scroll bg-white border-none rounded-b-[10px] !border-t-gray-200 max-h-[300px]"
    :style="{ maxHeight: `calc(${searchWindowHeight}px - ${SEARCH_INPUT_HEIGHT}px)` }"
  >
    <template
      :key="item.id"
      v-for="(item, index) of resultList"
    >
      <!-- <LaunchItem
        :icon="item.icon!"
        :name="item.name"
      /> -->
      <li
        class="flex items-center px-2 h-[48px]"
        :ref="el => (itemRefs[index] = el as any)"
        :class="[
          'px-4 py-2 cursor-pointer',
          index === selectedIndex ? 'bg-[#f5f5f5]' : 'hover:bg-gray-100',
        ]"
        @click="
          () => {
            selectedIndex = index
            handleEnter()
          }
        "
      >
        <!-- @mouseenter="selectedIndex = index" -->
        <img
          :src="item.icon || ''"
          alt="icon"
          class="!m-2 object-contain pointer-events-none w-8"
        />

        <span class="!ml-0.5">{{ item.name }}</span>

        <!-- TODO 个性化控制 分类是否显示 -->
        <span
          v-if="item.category_name"
          class="!ml-3"
        >
          （{{ item.category_name }}）
        </span>
      </li>
    </template>
  </ul>

  <!-- </div> -->
</template>

<script setup lang="ts">
import { ref } from 'vue'
import {
  LogicalPosition,
  LogicalSize,
  cursorPosition,
  getCurrentWindow,
} from '@tauri-apps/api/window'
import { SearchOutline } from '@vicons/ionicons5'
import { useLaunchAction } from '@/composables/useLaunchAction'
import { isRegistered, register, unregister } from '@tauri-apps/plugin-global-shortcut'
import { useAppConfig } from '@/composables/useAppConfig'

import { SEARCH_WINDOW_WIDTH, SEARCH_INPUT_HEIGHT, SEARCH_RESULT_ITEM_HEIGHT } from '@/constant'
import { exeCommand, searchLaunch } from '@/api'

const { runLaunch } = useLaunchAction()

const { appConfigStore } = useAppConfig()

const placeholder = ref('名称 / 拼音 / 关键字 / 文件(目录)地址 / URL / Win内置命令 (mstsc)')

const keyword = ref('')
const resultList = ref<SearchLauncItem[]>([])
const current = getCurrentWindow()

const itemRefs = ref<HTMLElement[]>([])
// 选中启动光标
const selectedIndex = ref(0)

const handleKeydown = (e: KeyboardEvent) => {
  const reultCount = resultList.value.length
  const minIndex = 0
  const maxIndex = reultCount - 1
  const { keyCode } = e
  // code
  // Enter keyCode=13 code=Enter
  // Esc   keyCode=27 code=Escape
  // 上箭头 keyCode=38 code=ArrowUp
  // 下箭头 keyCode=40 code=ArrowDown
  switch (keyCode) {
    case 13:
      handleEnter()
      break
    case 27:
      // Esc 键 关闭搜索窗口
      handleClose()
      break
    case 38:
      if (selectedIndex.value === minIndex && reultCount) selectedIndex.value = reultCount - 1
      else selectedIndex.value > 0 && selectedIndex.value--
      e.preventDefault()
      break
    case 40:
      if (selectedIndex.value === maxIndex && reultCount) selectedIndex.value = minIndex
      else selectedIndex.value < maxIndex && selectedIndex.value++
      e.preventDefault()
      break
  }
}

const handleEnter = async () => {
  if (!keyword.value.trim()) return

  /**
   * 查询结果列表中是否有内容
   *  有 则执行选中启动项
   *  没有 则判断输入框内容执行对应的操作
   *   1.如果是文件路径则打开对应文件
   *   2.如果是目录则打开对应目录
   *   3.如果是 URL 则打开对应的网页
   *   4.如果不是则将输入内容传入命令执行
   *
   * 根据命令行执行结果 执行后续
   * 如果为有效命令 执行并关闭搜索框
   * 如果为无效命令 不做响应 (无法实现)
   */
  if (!resultList.value.length) {
    await exeCommand(keyword.value)
  } else {
    const item = resultList.value[selectedIndex.value]
    if (!item) return
    // 执行启动
    await runLaunch(item.id)
    // TODO 根据返回结果进行统计 对应次数
  }
  handleClose()
}

const inputRef = useTemplateRef('searchInputRef')

const handleClose = () => {
  // 清空输入框
  keyword.value = ''
  resultList.value = []
  selectedIndex.value = 0
  // 隐藏搜索窗口
  // current.setSize(new LogicalSize(600, 45))
  current.hide()
}

const handleShow = async () => {
  // 当存在多个显示器时 将搜索窗口显示在鼠标停留的显示器上
  const { x, y } = await cursorPosition()

  // TODO 可作为个性化设置 搜索框呼出位置跟随鼠标 需要适配搜索结果显示位置 朝上或者朝下
  // await current.setPosition(new LogicalPosition(x, y))
  const { width } = await current.innerSize()
  // 存在多个显示器时 鼠标边缘呼出适配
  await current.setPosition(new LogicalPosition(x - width / 2, y))
  await current.center()

  // 显示搜索窗口
  await current.show()
  // 窗口聚焦
  await current.setFocus()
  // 输入框聚焦
  inputRef.value?.focus()
}

// const searchWindowHeight = ref<number>(300)
// console.log('searchWindowHeight ------', searchWindowHeight.value)
// 当使用箭头控制选项是 自动将预选项滚到可视窗口内
watch(selectedIndex, async newIndex => {
  await nextTick()
  const el = itemRefs.value[newIndex]
  el?.scrollIntoView({ behavior: 'smooth', block: 'nearest' })
})

const searchWindowHeight = computed(() => {
  if (!resultList.value.length) return SEARCH_INPUT_HEIGHT

  // 结果列表总高度 + 1像素的的顶部边框高度
  const resultsHeight = resultList.value.length * SEARCH_RESULT_ITEM_HEIGHT + 1

  return resultsHeight + SEARCH_INPUT_HEIGHT > appConfigStore.searchWindowMaxHeight
    ? appConfigStore.searchWindowMaxHeight
    : resultsHeight + SEARCH_INPUT_HEIGHT + 1
})

watch(
  () => keyword.value,
  async keyword => {
    if (!keyword.trim()) {
      selectedIndex.value = 0
      resultList.value = []
    } else {
      const launchs = await searchLaunch(keyword)
      resultList.value = launchs
    }
    current.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value))
  }
)

const hasResult = computed(() => !!resultList.value.length)

const handleRegisterSearchHotkey = async () => {
  const key = appConfigStore.searchGlobalShortcutKey
  if (!key) return
  const isReg = await isRegistered(key)
  isReg && (await unregister(key))

  register(key, async e => {
    if (e.state === 'Released') {
      const windowVisible = await current.isVisible()

      windowVisible ? handleClose() : handleShow()
    }
  })
}

handleRegisterSearchHotkey()
</script>

<style scoped>
/* ::v-deep(.n-input) { */
.n-input {
  /* 移除移入移出是边框变化 */
  --n-border-hover: 0px !important;
  --n-border-focus: 0px !important;
  --n-border: 0px !important;
  /* 输入框光标颜色 */
  --n-caret-color: gray !important;
  /* 输入框高度 */
  --n-height: 100% !important;
  /* 输入框字体大小 */
  --n-font-size: 20px !important;

  --n-border-focus: 0px !important;

  border-radius: 10px;
  border: none !important;
  /* border-color: !important; */
}

::v-deep(.n-input__placeholder) {
  font-size: 14px !important;
}

.search-container {
  /* max-height: calc(v-bind(searchWindowHeight + 'px') - v-bind(SEARCH_INPUT_HEIGHT + 'px')); */
  border-top: 0.5px solid;
  box-sizing: border-box;
}

ul:focus-visible {
  outline: none !important; /* 例如，取消焦点时的轮廓 */
}
</style>
