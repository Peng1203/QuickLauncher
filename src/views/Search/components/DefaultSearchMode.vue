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
      :placeholder="activeHistory?.command || placeholder"
    >
      <template #prefix>
        <template v-if="activeHistory">
          <n-avatar
            v-if="activeHistory.icon"
            class="!bg-transparent"
            :size="22"
            :src="activeHistory.icon"
          />

          <n-icon
            v-else-if="activeHistory.type === 'command'"
            size="22"
            class="iconfont icon-minglinghang"
          />
        </template>

        <n-icon
          v-else
          :component="SearchOutline"
          size="22"
        />
      </template>
    </n-input>

    <div
      v-if="autocompleteList.length"
      class="suggestion-con"
    >
      <span class="suggestion-text">
        {{ currentAutocompleteSuggestion }}
      </span>

      <div class="flex items-center gap-5 mr-3">
        <span
          v-show="autocompleteList.length !== 1"
          class="flex items-center select-none"
        >
          <Kbd>Tab</Kbd>
          <span class="text-xs ml-1">{{ t('search.switch') }}</span>
        </span>

        <span
          v-show="currentAutocompleteSuggestion !== keyword"
          class="flex items-center select-none"
        >
          <Kbd>-></Kbd>
          <span class="text-xs ml-1">{{ t('search.autocomplete') }}</span>
        </span>

        <div
          v-show="resultList.length"
          class="flex"
        >
          <span class="flex items-center select-none mr-3">
            <Kbd>Ctrl</Kbd>
            <span>+</span>
            <Kbd>W</Kbd>
            <span class="text-xs ml-1">{{ t('search.closeResults') }}</span>
          </span>
        </div>
      </div>
    </div>

    <div
      v-if="appConfigStore.showHistory"
      v-show="!keyword.length"
      class="suggestion-con"
    >
      <span class="suggestion-text"></span>

      <div class="flex gap-5 mr-3">
        <span
          v-show="activeHistory"
          class="flex items-center select-none"
        >
          <Kbd>Enter</Kbd>
          <span class="text-xs ml-1">{{ t('search.confirm') }}</span>
        </span>

        <span class="flex gap-1">
          <span class="flex items-center select-none">
            <Kbd>Up</Kbd>
          </span>
          <span class="flex items-center select-none">
            <Kbd>Down</Kbd>
            <span class="text-xs ml-1">{{ t('search.history') }}</span>
          </span>
        </span>
      </div>
    </div>
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
        @contextmenu.prevent.stop="handleShowContextMenu($event, item)"
      >
        <div class="flex items-center">
          <n-icon
            v-if="item.type === 'alias'"
            size="32"
            class="iconfont icon-minglinghangchaxun !m-2 text-[32px]"
          />

          <img
            v-else
            :src="item.icon || ''"
            alt="icon"
            class="!m-2 object-contain pointer-events-none w-8 h-8"
          />

          <span class="!ml-0.5">{{ item.name }}</span>
        </div>

        <div
          v-if="appConfigStore.showCategory"
          class="flex items-end space-x-1"
        >
          <n-tag
            v-if="item.type === 'alias'"
            bordered
            size="small"
            type="info"
          >
            {{ t('search.commandAlias') }}
          </n-tag>

          <template v-else>
            <n-tag
              v-if="item.category_name"
              bordered
              size="small"
              type="default"
            >
              {{ item.category_name }}
            </n-tag>

            <n-tag
              v-if="appConfigStore.showSubCategory && item.subcategory_name"
              bordered
              size="tiny"
              type="default"
            >
              {{ item.subcategory_name }}
            </n-tag>
          </template>
        </div>
      </li>
    </template>
  </transition-group>

  <LaunchItemContextMenu
    v-model="menuVisible"
    type="SearchLaunchList"
    li-style="padding-top: 4px; padding-bottom: 4px;"
    style="transform: scale(0.85); overflow-y: scroll"
    :style="{ height: contextMenuHeight }"
    :viewport-margin="0"
    :item="itemDetail!"
    :category-item="categoryItem"
    :selected-ids="[]"
    :position="menuPosition"
    :item-path="itemDetail!.path"
    :item-name="itemDetail!.name"
  />
</template>

<script setup lang="ts">
import type { SearchModelType } from '../searchModes';
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';
import { SearchOutline } from '@vicons/ionicons5';
import { nextTick, ref } from 'vue';
import {
  addLaunchHistory,
  addOrUpdateAutocompleteRecord,
  exeCommand,
  getAutocomplete,
  getCategoryByID,
  getLaunchByID,
  getRecentLaunchHistory,
  runLaunch,
  searchLaunch,
  updateLaunch,
} from '@/api';
import LaunchItemContextMenu from '@/components/ListItemContextMenu.vue';
import { useAppConfig, useNaiveUiApi } from '@/composables';
import {
  AppEvent,
  SEARCH_INPUT_HEIGHT,
  SEARCH_RESULT_ITEM_HEIGHT,
  SEARCH_WINDOW_WIDTH,
  TranslationOpenModel,
  WebSearchOpenModel,
} from '@/constant';
import { t } from '@/i18n';
import { EventBus } from '@/utils/eventBus';
import { SEARCH_MODEL } from '../searchModes';

const props = withDefaults(
  defineProps<{
    keyword?: string;
    chromeHeight?: number;
  }>(),
  {
    keyword: '',
    chromeHeight: 0,
  },
);
const emit = defineEmits<{
  closeWindow: [isEscClose?: boolean];
  switchMode: [payload: { mode: SearchModelType; keyword?: string; source?: WebSearchSource }];
}>();

const { appConfigStore } = useAppConfig();
const { notification } = useNaiveUiApi();
const searchWindow = getCurrentWindow();
const placeholder = t('search.placeholder');
const inputRef = useTemplateRef('searchInputRef');
const keyword = ref(props.keyword || '');
const resultList = ref<SearchLauncItem[]>([]);
const itemRefs = ref<HTMLElement[]>([]);
const selectedIndex = ref(0);
const searchFlag = ref(false);
const autocompleteList = ref<string[]>([]);
const autocompleteIndex = ref(0);
const currentAutocompleteSuggestion = computed(() => autocompleteList.value[autocompleteIndex.value]);
const hasResult = computed(() => !!resultList.value.length);
const chromeHeight = computed(() => props.chromeHeight);

const searchWindowHeight = computed(() => {
  if (!resultList.value.length) return chromeHeight.value + SEARCH_INPUT_HEIGHT;

  const resultsHeight = resultList.value.length * SEARCH_RESULT_ITEM_HEIGHT;
  const contentHeight = resultsHeight + SEARCH_INPUT_HEIGHT;
  return (
    chromeHeight.value +
    (contentHeight > appConfigStore.searchWindowMaxHeight ? appConfigStore.searchWindowMaxHeight : contentHeight + 1)
  );
});

const activeHistoryIndex = ref(0);
const historyData = ref<LaunchHistoryWithIcon[]>([]);
const activeHistory = computed<LaunchHistoryWithIcon | null>(() => {
  if (!activeHistoryIndex.value) return null;
  return historyData.value[activeHistoryIndex.value - 1];
});

const menuVisible = ref(false);
const menuPosition = ref({ x: 0, y: 0 });
const categoryItem = ref<CategoryItem | null>();
const itemDetail = ref<LaunchItem>({
  id: 0,
  name: '',
  path: '',
  type: 'file',
  created_at: '',
  updated_at: '',
  launch_count: 0,
  failure_count: 0,
  pinyin_full: '',
  pinyin_abbr: '',
});
const contextMenuHeight = computed(() => {
  if (resultList.value.length === 1) return '80px';
  return resultList.value.length <= 3 ? `${50 + (resultList.value.length - 1) * 48}px` : 'initial';
});

let spaceCounter = 0;
let searchRequestId = 0;

function focus() {
  inputRef.value?.focus();
}

function handleClose() {
  keyword.value = '';
  selectedIndex.value = 0;
  resultList.value = [];
  autocompleteIndex.value = 0;
  autocompleteList.value = [];
  activeHistoryIndex.value = 0;
  menuVisible.value = false;
  searchWindow.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value));
}

function handleChangeCurrentAutocomplete() {
  if (autocompleteList.value.length === 1) return;
  if (autocompleteList.value.length - 1 === autocompleteIndex.value) {
    autocompleteIndex.value = 0;
    return;
  }
  autocompleteIndex.value++;
}

function handleChangeHistory(type: 'up' | 'down') {
  if (!historyData.value.length) return;
  if (activeHistoryIndex.value >= historyData.value.length && type === 'up') return;
  if (activeHistoryIndex.value <= 0 && type === 'down') return;
  type === 'up' ? (activeHistoryIndex.value += 1) : (activeHistoryIndex.value -= 1);
}

function tryOpenWebSearch() {
  setTimeout(() => {
    if (!keyword.value.trim() || appConfigStore.webSearchOpenModel === WebSearchOpenModel.CLOSE) return;

    let flag = false;
    let key = '';
    if (appConfigStore.webSearchOpenModel === WebSearchOpenModel.KEY_SPACE) {
      flag = true;
      key = keyword.value.trim();
    } else if (appConfigStore.webSearchOpenModel === WebSearchOpenModel.COLON_KEY_SPACE) {
      if (keyword.value.trim().substring(0, 1) === ':') flag = true;
      key = keyword.value.trim().substring(1, keyword.value.trim().length);
    }
    if (!flag) return;

    const source = appConfigStore.webSearchSourceList.find(({ keywords }) => keywords === key);
    if (!source) return;

    emit('switchMode', { mode: SEARCH_MODEL.WEB_SEARCH_MODEL, source });
  }, 50);
}

function handleKeydown(e: KeyboardEvent) {
  const resultCount = resultList.value.length;
  const maxIndex = resultCount - 1;
  const { keyCode, ctrlKey, key } = e;

  spaceCounter = keyCode === 32 ? spaceCounter + 1 : 0;

  if (ctrlKey && (key === 'w' || key === 'W')) {
    handleClose();
    e.preventDefault();
    return;
  }

  switch (keyCode) {
    case 9:
      if (autocompleteList.value.length) handleChangeCurrentAutocomplete();
      e.preventDefault();
      break;
    case 13:
      handleEnter();
      break;
    case 27:
      emit('closeWindow', true);
      break;
    case 32:
      if (
        appConfigStore.enableTranslation &&
        appConfigStore.translationOpenModel === TranslationOpenModel.THREE_HITS_ON_SPACES &&
        spaceCounter === 3
      ) {
        emit('switchMode', { mode: SEARCH_MODEL.TRANSLATION_MODEL, keyword: keyword.value });
        break;
      }
      if (appConfigStore.enableWebSearch) tryOpenWebSearch();
      break;
    case 38:
      if (appConfigStore.showHistory && !keyword.value.length) {
        handleChangeHistory('up');
      } else if (selectedIndex.value === 0 && resultCount) {
        selectedIndex.value = maxIndex;
      } else if (selectedIndex.value > 0) {
        selectedIndex.value--;
      }
      e.preventDefault();
      break;
    case 39:
      if (autocompleteList.value.length) keyword.value = currentAutocompleteSuggestion.value;
      break;
    case 40:
      if (!keyword.value.length) {
        handleChangeHistory('down');
      } else if (selectedIndex.value === maxIndex && resultCount) {
        selectedIndex.value = 0;
      } else if (selectedIndex.value < maxIndex) {
        selectedIndex.value++;
      }
      e.preventDefault();
      break;
  }
}

async function handleEnter() {
  try {
    if (!keyword.value.length) {
      await handleHistoryEnterLaunch();
      emit('closeWindow');
      return;
    }

    if (!searchFlag.value) return;
    await handleEnterLaunch();
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

async function handleEnterLaunch() {
  if (!resultList.value.length) {
    await exeCommand(keyword.value);
    addOrUpdateAutocompleteRecord(keyword.value);
    appConfigStore.enableHistory && addLaunchHistory(keyword.value, 'command');
    return;
  }

  const item = resultList.value[selectedIndex.value];
  if (!item) return;

  await runLaunch(item.id);
  EventBus.emit(AppEvent.UPDATE_LAUNCH_ITEM_COUNT, item.id);
  addOrUpdateAutocompleteRecord(keyword.value, item.id);
  appConfigStore.enableHistory && addLaunchHistory(keyword.value, item.type, item.id);
}

async function handleHistoryEnterLaunch() {
  if (!activeHistory.value) return;
  const { launch_item_id, command, type } = activeHistory.value;

  if (launch_item_id) {
    await runLaunch(launch_item_id);
    EventBus.emit(AppEvent.UPDATE_LAUNCH_ITEM_COUNT, launch_item_id);
    addOrUpdateAutocompleteRecord(command, launch_item_id);
    appConfigStore.enableHistory && addLaunchHistory(command, type, launch_item_id);
  } else {
    await exeCommand(command);
    addOrUpdateAutocompleteRecord(command);
    appConfigStore.enableHistory && addLaunchHistory(command, 'command');
  }
}

async function handleSearch() {
  const currentId = ++searchRequestId;
  autocompleteIndex.value = 0;
  autocompleteList.value = [];
  activeHistoryIndex.value = 0;

  if (!keyword.value.trim()) {
    handleClose();
    return;
  }

  if (appConfigStore.enableAutocomplete) {
    getAutocomplete(keyword.value).then(res => {
      if (currentId === searchRequestId) autocompleteList.value = res;
    });
  }

  let launchs = await searchLaunch(keyword.value);
  if (!searchFlag.value) searchFlag.value = true;
  if (!appConfigStore.enableCommandAlias) launchs = launchs.filter(item => item.type !== 'alias');
  resultList.value = launchs;

  if (currentId === searchRequestId) {
    searchWindow.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value));
  }
}

async function handleShowContextMenu(e: MouseEvent, item: SearchLauncItem) {
  const { id, category_id, type } = item;
  if (type === 'alias') return;

  const [launch, category] = await Promise.all([getLaunchByID(id), getCategoryByID(category_id!).catch(() => null)]);
  categoryItem.value = category;
  if (!launch) {
    return notification.error({
      content: t('search.queryFailed'),
      meta: t('search.notFound'),
      duration: 3000,
      keepAliveOnHover: true,
    });
  }
  itemDetail.value = launch;

  nextTick(() => {
    menuVisible.value = true;
    menuPosition.value = { x: e.clientX, y: e.clientY };
  });
}

function resizeWindow() {
  searchWindow.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value));
}

watch(() => keyword.value, handleSearch);

watch(selectedIndex, async newIndex => {
  await nextTick();
  itemRefs.value[newIndex]?.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
});

watch(chromeHeight, resizeWindow);

EventBus.listen(AppEvent.INCREASE_PRIORITY, async (item: LaunchItem) => {
  let { order_index = 0 } = item;
  await updateLaunch({
    ...item,
    order_index: (order_index += 10),
  });
  await handleSearch();
  EventBus.emit(AppEvent.UPDATE_LAUNCH_LIST);
});

onMounted(() => {
  getRecentLaunchHistory().then(res => (historyData.value = res));
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

.suggestion-con {
  position: absolute;
  display: flex;
  align-items: center;
  justify-content: space-between;
  top: 0;
  left: 0;
  width: 100%;
  height: 45px;
  font-size: 20px;
  opacity: 0.3;
  cursor: text;

  .suggestion-text {
    margin-left: 38px;
    width: fit-content;
  }
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
