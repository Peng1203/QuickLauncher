<template>
  <label class="input-container max-h-11.25">
    <n-input
      ref="searchInputRef"
      v-model:value="keyword"
      :disabled="isEmpty"
      tabindex="-1"
      type="text"
      size="medium"
      class="w-full h-full max-h-11.25 resize-none text-sm hover:outline-0 focus-visible:outline-0 border-none bg-card shadow-none rounded-[10px]"
      :class="[
        showDropdown ? 'border-b-0! rounded-b-none!' : '',
        isEmpty ? ' border-0! rounded-b-none!' : '',
      ]"
      :placeholder="
        isEmpty
          ? '暂无网络搜索源，请在设置中添加'
          : selectedSource?.desc || selectedSource?.name || ''
      "
    >
      <template #prefix>
        <n-avatar
          v-if="selectedSource?.icon"
          class="!bg-transparent"
          :size="22"
          :src="selectedSource.icon"
        />
        <n-icon v-else :component="GlobeOutline" size="22" />
      </template>
    </n-input>
  </label>

  <transition-group
    v-if="showDropdown"
    name="list"
    tag="ul"
    tabindex="-1"
    class="search-container absolute z-50 w-full overflow-y-scroll bg-card border-none rounded-b-[10px] !border-t-border max-h-[300px]"
    :style="{
      maxHeight: `calc(${searchWindowHeight}px - ${chromeHeight}px - ${SEARCH_INPUT_HEIGHT}px)`,
    }"
  >
    <template v-for="(item, index) of engineList" :key="item.id">
      <li
        :ref="(el) => (itemRefs[index] = el as any)"
        class="flex items-center justify-between h-[48px] px-4 py-2 cursor-pointer"
        :class="[index === selectedIndex ? 'bg-muted' : 'hover:bg-muted']"
        @click="handleSelectEngine(index)"
      >
        <div class="flex items-center gap-2">
          <n-avatar v-if="item.icon" class="!bg-transparent" :size="18" :src="item.icon" />
          <n-icon v-else :component="GlobeOutline" size="18" />
          <span>{{ item.name }}</span>
        </div>
        <span v-if="item.desc" class="text-xs text-muted-foreground">{{ item.desc }}</span>
      </li>
    </template>
  </transition-group>
</template>

<script setup lang="ts">
import type { SearchModelType } from "../searchModes";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import { GlobeOutline } from "@vicons/ionicons5";
import { ref, watch } from "vue";
import { exeCommand } from "@/api";
import { useAppConfig, useNaiveUiApi } from "@/composables";
import { SEARCH_INPUT_HEIGHT, SEARCH_RESULT_ITEM_HEIGHT, SEARCH_WINDOW_WIDTH } from "@/constant";
import { t } from "@/i18n";

const props = defineProps<{
  keyword?: string;
  source?: WebSearchSource;
  chromeHeight?: number;
}>();
const emit = defineEmits<{
  closeWindow: [isEscClose?: boolean];
  switchMode: [payload: { mode: SearchModelType; keyword?: string; source?: WebSearchSource }];
}>();

const { appConfigStore } = useAppConfig();
const { notification } = useNaiveUiApi();
const searchWindow = getCurrentWindow();
const inputRef = useTemplateRef("searchInputRef");
const keyword = ref(props.keyword || "");
const itemRefs = ref<HTMLElement[]>([]);
const chromeHeight = computed(() => props.chromeHeight || 0);

// 搜索引擎列表
const engineList = computed(() => appConfigStore.webSearchSourceList);
const isEmpty = computed(() => engineList.value.length === 0);

const selectedIndex = ref(0);

// 当前选中的搜索引擎
const selectedSource = computed(() => engineList.value[selectedIndex.value]);

// 输入框为空时展示下拉
const showDropdown = computed(() => !keyword.value.trim() && engineList.value.length > 0);

const searchWindowHeight = computed(() => {
  if (!showDropdown.value) return chromeHeight.value + SEARCH_INPUT_HEIGHT;

  const resultsHeight = engineList.value.length * SEARCH_RESULT_ITEM_HEIGHT;
  const contentHeight = resultsHeight + SEARCH_INPUT_HEIGHT;
  return (
    chromeHeight.value +
    (contentHeight > appConfigStore.searchWindowMaxHeight
      ? appConfigStore.searchWindowMaxHeight
      : contentHeight + 1)
  );
});

function focus() {
  inputRef.value?.focus();
}

function resizeWindow() {
  searchWindow.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value));
}

function handleClose() {
  keyword.value = "";
  selectedIndex.value = 0;
  resizeWindow();
}

function handleSelectEngine(index: number) {
  selectedIndex.value = index;
  focus();
}

function handleKeydown(e: KeyboardEvent) {
  const engineCount = engineList.value.length;
  const maxIndex = engineCount - 1;
  const { keyCode, ctrlKey, key } = e;

  if (ctrlKey && (key === "w" || key === "W")) {
    handleClose();
    e.preventDefault();
    return;
  }

  switch (keyCode) {
    case 13:
      handleEnter();
      break;
    case 27:
      // emit("switchMode", { mode: SEARCH_MODEL.DEFAULT_MODEL });
      emit("closeWindow");
      break;
    case 38:
      if (showDropdown.value) {
        if (selectedIndex.value === 0 && engineCount) selectedIndex.value = maxIndex;
        else if (selectedIndex.value > 0) selectedIndex.value--;
        e.preventDefault();
      }
      break;
    case 40:
      if (showDropdown.value) {
        if (selectedIndex.value === maxIndex && engineCount) selectedIndex.value = 0;
        else if (selectedIndex.value < maxIndex) selectedIndex.value++;
        e.preventDefault();
      }
      break;
  }
}

async function handleEnter() {
  try {
    if (!keyword.value.trim()) return;

    const source = selectedSource.value;
    if (!source?.searchApi) return;

    const searchUrl = source.searchApi.replace("{w}", encodeURI(keyword.value));
    if (!searchUrl) return;

    await exeCommand(searchUrl);
    emit("closeWindow");
  } catch (e) {
    notification.error({
      content: t("search.launchFailed"),
      meta: e as string,
      duration: 3000,
      keepAliveOnHover: true,
    });
  }
}

// 当从默认模式带 source 进入时，定位到对应搜索引擎
watch(
  () => props.source,
  (newSource) => {
    if (!newSource) return;
    const idx = engineList.value.findIndex((s) => s.id === newSource.id);
    if (idx !== -1) selectedIndex.value = idx;
  },
  { immediate: true },
);

watch(
  () => keyword.value,
  () => {
    resizeWindow();
  },
);

watch(
  () => engineList.value,
  () => {
    resizeWindow();
  },
);

watch(selectedIndex, async (newIndex) => {
  await nextTick();
  itemRefs.value[newIndex]?.scrollIntoView({ behavior: "smooth", block: "nearest" });
});

watch(chromeHeight, resizeWindow);

onMounted(() => {
  nextTick(focus);
});

defineExpose({
  focus,
  handleClose,
  handleKeydown,
  getDefaultHeight: () => searchWindowHeight.value,
});
</script>

<style scoped lang="scss">
.n-input {
  --n-border-hover: 0px !important;
  --n-border-focus: 0px !important;
  --n-border: 0px !important;
  --n-caret-color: gray !important;
  --n-height: 100% !important;
  --n-font-size: 20px !important;
  --n-border-disabled: none !important;

  border-radius: 5px;
  border: none !important;
}

::v-deep(.n-input__placeholder) {
  font-size: 14px !important;
  margin-left: 5px;
}

.input-container {
  width: 100%;
  height: 45px;
  position: relative;
  display: block;
}

.search-container {
  box-sizing: border-box;
  border-top: 0.5px solid;
  border-radius: 0 0 5px 5px;
}

ul:focus-visible {
  outline: none !important;
}

.list-move {
  transition: transform 0.25s ease;
}

.list-enter-active,
.list-leave-active {
  transition: all 0.2s ease;
}

.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateY(8px);
}
</style>
