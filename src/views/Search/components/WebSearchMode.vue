<template>
  <label class="input-container max-h-11.25">
    <n-input
      ref="searchInputRef"
      v-model:value="keyword"
      tabindex="-1"
      type="text"
      size="medium"
      class="w-full h-full max-h-11.25 resize-none text-sm hover:outline-0 focus-visible:outline-0 border-none bg-card shadow-none rounded-[10px]"
      :class="hasResult ? 'border-b-0! rounded-b-none!' : ''"
      :placeholder="source?.desc || source?.name || ''"
    >
      <template #prefix>
        <n-avatar
          v-if="source?.icon"
          class="!bg-transparent"
          :size="22"
          :src="source.icon"
        />
        <n-icon
          v-else
          :component="GlobeOutline"
          size="22"
        />
      </template>
    </n-input>
  </label>

  <transition-group
    name="list"
    tag="ul"
    tabindex="-1"
    class="search-container absolute z-50 w-full overflow-y-scroll bg-card border-none rounded-b-[10px] !border-t-border max-h-[300px]"
    :style="{
      maxHeight: `calc(${searchWindowHeight}px - ${chromeHeight}px - ${SEARCH_INPUT_HEIGHT}px)`,
    }"
  >
    <template
      v-for="(item, index) of resultList"
      :key="item.id"
    >
      <li
        :ref="el => (itemRefs[index] = el as any)"
        class="flex items-center justify-between h-[48px] px-4 py-2 cursor-pointer"
        :class="[index === selectedIndex ? 'bg-muted' : 'hover:bg-muted']"
        @click="
          () => {
            selectedIndex = index;
            handleEnter();
          }
        "
      >
        <div class="flex items-center">
          <span class="!ml-0.5">{{ item.name }}</span>
        </div>
      </li>
    </template>
  </transition-group>
</template>

<script setup lang="ts">
import type { SearchModelType } from '../searchModes';
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';
import { fetch } from '@tauri-apps/plugin-http';
import { GlobeOutline } from '@vicons/ionicons5';
import { ref, watch } from 'vue';
import { exeCommand } from '@/api';
import { useAppConfig, useNaiveUiApi } from '@/composables';
import { SEARCH_INPUT_HEIGHT, SEARCH_RESULT_ITEM_HEIGHT, SEARCH_WINDOW_WIDTH } from '@/constant';
import { t } from '@/i18n';
import { SEARCH_MODEL } from '../searchModes';

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
const inputRef = useTemplateRef('searchInputRef');
const keyword = ref(props.keyword || '');
const resultList = ref<SearchLauncItem[]>([]);
const itemRefs = ref<HTMLElement[]>([]);
const selectedIndex = ref(0);
const hasResult = computed(() => !!resultList.value.length);
const chromeHeight = computed(() => props.chromeHeight || 0);

const searchWindowHeight = computed(() => {
  if (!resultList.value.length) return chromeHeight.value + SEARCH_INPUT_HEIGHT;

  const resultsHeight = resultList.value.length * SEARCH_RESULT_ITEM_HEIGHT;
  const contentHeight = resultsHeight + SEARCH_INPUT_HEIGHT;
  return (
    chromeHeight.value +
    (contentHeight > appConfigStore.searchWindowMaxHeight ? appConfigStore.searchWindowMaxHeight : contentHeight + 1)
  );
});

let searchRequestId = 0;

function focus() {
  inputRef.value?.focus();
}

function resizeWindow() {
  searchWindow.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value));
}

function handleClose() {
  keyword.value = '';
  selectedIndex.value = 0;
  resultList.value = [];
  resizeWindow();
}

function handleKeydown(e: KeyboardEvent) {
  const resultCount = resultList.value.length;
  const maxIndex = resultCount - 1;
  const { keyCode, ctrlKey, key } = e;

  if (ctrlKey && (key === 'w' || key === 'W')) {
    handleClose();
    e.preventDefault();
    return;
  }

  switch (keyCode) {
    case 13:
      handleEnter();
      break;
    case 27:
      emit('switchMode', { mode: SEARCH_MODEL.DEFAULT_MODEL });
      break;
    case 38:
      if (selectedIndex.value === 0 && resultCount) selectedIndex.value = maxIndex;
      else if (selectedIndex.value > 0) selectedIndex.value--;
      e.preventDefault();
      break;
    case 40:
      if (selectedIndex.value === maxIndex && resultCount) selectedIndex.value = 0;
      else if (selectedIndex.value < maxIndex) selectedIndex.value++;
      e.preventDefault();
      break;
  }
}

async function handleEnter() {
  try {
    if (!keyword.value.length && !resultList.value.length) return;

    const item = resultList.value[selectedIndex.value];
    const keywordStr = props.source?.searchApi?.replace('{w}', encodeURI(item ? item.name : keyword.value)) || '';
    if (!keywordStr) return;

    await exeCommand(keywordStr);
    emit('closeWindow');
  } catch (e) {
    notification.error({
      content: t('search.launchFailed'),
      meta: e as string,
      duration: 3000,
      keepAliveOnHover: true,
    });
  }
}

async function searchSuggestion(): Promise<SearchLauncItem[]> {
  if (!props.source?.suggestionApi) return [];

  const url = props.source.suggestionApi.replace('{w}', encodeURIComponent(keyword.value));
  const data = await fetch(url).then(res => res.json());

  return data[1].map((item: string, i: number) => ({
    id: i,
    name: item,
    path: '',
    icon: '',
    type: 'url',
    category_name: '',
    subcategory_name: '',
  }));
}

async function handleSearch() {
  const currentId = ++searchRequestId;

  if (!keyword.value.trim()) {
    handleClose();
    return;
  }

  const launchs = await searchSuggestion();
  if (currentId !== searchRequestId) return;

  resultList.value = launchs;
  resizeWindow();
}

watch(() => keyword.value, handleSearch);

watch(selectedIndex, async newIndex => {
  await nextTick();
  itemRefs.value[newIndex]?.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
});

watch(chromeHeight, resizeWindow);

onMounted(() => {
  nextTick(focus);
});

defineExpose({
  focus,
  handleClose,
  handleKeydown,
  getDefaultHeight: () => chromeHeight.value + SEARCH_INPUT_HEIGHT,
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
