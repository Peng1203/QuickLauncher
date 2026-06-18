<template>
  <label class="input-container max-h-11.25">
    <n-input
      ref="searchInputRef"
      v-model:value="keyword"
      tabindex="-1"
      type="text"
      size="medium"
      class="w-full h-full max-h-11.25 resize-none text-sm hover:outline-0 focus-visible:outline-0 border-none bg-card shadow-none rounded-[10px]"
      :class="[hasResult ? 'border-b-0! rounded-b-none!' : '', esConfigured ? 'border-t-0!' : '']"
      :disabled="!esConfigured"
      :placeholder="
        esConfigured ? t('fileSearch.searchPlaceholder') : t('fileSearch.notConfiguredDesc')
      "
    >
      <template #prefix>
        <Icon name="icon-wenjian" size="22" :color="esConfigured ? '#155dfc' : '#999'" />
      </template>
      <template #suffix>
        <div class="suggestion-con">
          <span class="suggestion-text"></span>
          <div class="shortcut-list mr-3">
            <span v-if="searching" class="shortcut-item">
              <span class="text-xs">{{ t("fileSearch.searching") }}</span>
            </span>
            <template v-else-if="resultList.length">
              <span class="shortcut-item">
                <Kbd>↑↓</Kbd>
                <span class="text-xs ml-1">{{ t("todo.switch") }}</span>
              </span>
              <span class="shortcut-item">
                <Kbd>↵</Kbd>
                <span class="text-xs ml-1">{{ t("fileSearch.open") }}</span>
              </span>
              <span class="shortcut-item">
                <Kbd>Ctrl + ↵</Kbd>
                <span class="text-xs ml-1">{{ t("fileSearch.runAsAdmin") }}</span>
              </span>
            </template>
          </div>
        </div>
      </template>
    </n-input>
  </label>

  <transition-group
    name="list"
    tag="ul"
    tabindex="-1"
    class="search-container absolute z-50 w-full overflow-y-scroll bg-card border-none rounded-b-[10px] border-t-border! max-h-75"
    :style="{
      maxHeight: `calc(${searchWindowHeight}px - ${chromeHeight}px - ${SEARCH_INPUT_HEIGHT}px)`,
    }"
  >
    <template v-for="(item, index) in resultList" :key="item.path">
      <li
        ref="itemRefs"
        class="flex items-center gap-3 px-4 box-border cursor-pointer border-l-4 transition-all duration-150"
        :title="item.path"
        :class="[
          selectedIndex === index
            ? 'bg-muted border-l-blue-500'
            : 'border-l-transparent hover:bg-muted/50',
        ]"
        :style="{ height: `${SEARCH_RESULT_ITEM_HEIGHT}px` }"
        @click="
          () => {
            selectedIndex = index;
            handleEnter();
          }
        "
      >
        <!-- 文件图标 -->
        <div
          class="flex-shrink-0 w-9 h-9 flex items-center justify-center rounded-lg bg-gray-100 dark:bg-neutral-800"
        >
          <n-avatar class="bg-transparent!" :size="22" :src="item.icon" />
        </div>

        <!-- 文件信息 -->
        <div class="flex-1 min-w-0">
          <div class="text-[13px] font-medium truncate">{{ item.name }}</div>
          <div
            class="text-[11px] text-gray-400 truncate mt-0.5"
            v-html="highlightPath(item.path)"
          ></div>
        </div>
      </li>
    </template>
  </transition-group>
</template>

<script setup lang="ts">
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import { nextTick, ref, watch, onMounted, computed, useTemplateRef } from "vue";
import { searchFiles, openPath } from "@/api";
import { SEARCH_INPUT_HEIGHT, SEARCH_RESULT_ITEM_HEIGHT, SEARCH_WINDOW_WIDTH } from "@/constant";
import { t } from "@/i18n";
import { useAppConfig } from "@/composables";
import { useDebounceFn } from "@vueuse/core";

const props = withDefaults(
  defineProps<{
    keyword?: string;
    chromeHeight?: number;
  }>(),
  {
    keyword: "",
    chromeHeight: 0,
  },
);

const emit = defineEmits<{
  closeWindow: [];
  switchMode: [];
}>();

const { appConfigStore } = useAppConfig();
const searchWindow = getCurrentWindow();

const searchInputRef = useTemplateRef("searchInputRef");
const keyword = ref(props.keyword || "");
const resultList = ref<FileSearchResult[]>([]);
const selectedIndex = ref(0);
const searching = ref(false);
let searchRequestId = 0;

const hasResult = computed(() => !!resultList.value.length);
const chromeHeight = computed(() => props.chromeHeight);
const esConfigured = computed(() => !!appConfigStore.esFilePath);

const searchWindowHeight = computed(() => {
  if (!resultList.value.length) return chromeHeight.value + SEARCH_INPUT_HEIGHT;

  const resultsHeight = resultList.value.length * SEARCH_RESULT_ITEM_HEIGHT;
  const contentHeight = resultsHeight + SEARCH_INPUT_HEIGHT;
  return (
    chromeHeight.value +
    (contentHeight > appConfigStore.searchWindowMaxHeight
      ? appConfigStore.searchWindowMaxHeight
      : contentHeight + 1)
  );
});

function focus() {
  searchInputRef.value?.focus();
}

function handleClose() {
  keyword.value = "";
  selectedIndex.value = 0;
  resultList.value = [];
  searching.value = false;
  resizeWindow();
}

function resizeWindow() {
  searchWindow.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value));
}

function getDefaultHeight() {
  return chromeHeight.value + SEARCH_INPUT_HEIGHT;
}

function highlightPath(path: string): string {
  if (!keyword.value.trim()) return path;

  const escapedKeyword = keyword.value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const regex = new RegExp(`(${escapedKeyword})`, "gi");
  return path.replace(regex, '<span class="text-blue-500 font-medium font-weight-bold">$1</span>');
}

async function handleSearch() {
  const currentId = ++searchRequestId;
  selectedIndex.value = 0;

  if (!keyword.value.trim()) {
    resultList.value = [];
    resizeWindow();
    return;
  }

  if (!appConfigStore.esFilePath) {
    resultList.value = [];
    resizeWindow();
    return;
  }

  searching.value = true;
  try {
    const results = await searchFiles(keyword.value);
    if (currentId === searchRequestId) {
      resultList.value = results;
      resizeWindow();
    }
  } catch (e) {
    console.error("文件搜索失败:", e);
    if (currentId === searchRequestId) {
      resultList.value = [];
      resizeWindow();
    }
  } finally {
    if (currentId === searchRequestId) {
      searching.value = false;
    }
  }
}

const debouncedSearch = useDebounceFn(handleSearch, 300);

async function handleEnter() {
  if (!resultList.value.length) return;

  const item = resultList.value[selectedIndex.value];
  if (!item) return;

  try {
    await openPath(item.path);
    emit("closeWindow");
  } catch (e) {
    console.error("打开文件失败:", e);
  }
}

async function handleRunAsAdmin() {
  if (!resultList.value.length) return;

  const item = resultList.value[selectedIndex.value];
  if (!item) return;

  try {
    const { exeCommand } = await import("@/api");
    await exeCommand(`powershell -Command "Start-Process '${item.path}' -Verb RunAs"`);
    emit("closeWindow");
  } catch (e) {
    console.error("以管理员身份运行失败:", e);
  }
}

function handleKeydown(e: KeyboardEvent) {
  const { keyCode, ctrlKey, key } = e;

  // Ctrl+W 清空结果
  if (ctrlKey && (key === "w" || key === "W")) {
    selectedIndex.value = 0;
    resultList.value = [];
    resizeWindow();
    e.preventDefault();
    return;
  }

  switch (keyCode) {
    case 9: // TAB
      e.preventDefault();
      break;
    case 13: // ENTER
      if (ctrlKey) {
        handleRunAsAdmin();
      } else {
        handleEnter();
      }
      break;
    case 27: // ESC
      emit("closeWindow");
      break;
    case 38: // 上
      handleSelectIndexKeydownChange("up");
      e.preventDefault();
      break;
    case 40: // 下
      handleSelectIndexKeydownChange("down");
      e.preventDefault();
      break;
  }
}

function handleSelectIndexKeydownChange(type: "up" | "down") {
  if (!resultList.value.length) return;

  const count = resultList.value.length;
  if (type === "up") {
    selectedIndex.value = selectedIndex.value === 0 ? count - 1 : selectedIndex.value - 1;
  } else {
    selectedIndex.value = selectedIndex.value === count - 1 ? 0 : selectedIndex.value + 1;
  }
}

watch(keyword, debouncedSearch);

watch(chromeHeight, resizeWindow);

watch(selectedIndex, async (newIndex) => {
  await nextTick();
  const items = document.querySelectorAll(".search-container li");
  items[newIndex]?.scrollIntoView({ behavior: "smooth", block: "nearest" });
});

onMounted(() => {
  nextTick(() => {
    focus();
    resizeWindow();
  });
});

defineExpose({
  focus,
  handleClose,
  handleKeydown,
  getDefaultHeight,
});
</script>

<style scoped lang="scss">
.n-input {
  --n-border-hover: 0px !important;
  --n-border-focus: 0px !important;
  --n-border: 0px !important;
  --n-caret-color: gray !important;
  --n-height: 100% !important;
  --n-font-size: 16px !important;

  border-radius: 5px;
}

.input-container {
  width: 100%;
  height: 45px;
  position: relative;
  display: block;
}

.suggestion-con {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex: 1;
  min-width: 0;
}

.suggestion-text {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: #ccc;
  font-size: 14px;
  margin-left: 4px;
}

.shortcut-list {
  display: flex;
  align-items: center;
  gap: 6px;
}

.shortcut-item {
  display: flex;
  align-items: center;
  gap: 2px;
  font-size: 12px;
  color: #999;
  white-space: nowrap;
}

.list-move,
.list-enter-active,
.list-leave-active {
  transition: all 0.25s ease;
}

.list-enter-from {
  opacity: 0;
  transform: translateY(-8px);
}

.list-leave-to {
  opacity: 0;
  transform: translateY(8px);
}

.list-leave-active {
  position: absolute;
  width: 100%;
}

::v-deep(.n-input__border) {
  border: none !important;
}

::v-deep(.n-input__placeholder) {
  margin-left: 5px;
}
</style>
