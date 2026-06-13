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
      :placeholder="getPlaceholder"
    >
      <template #prefix>
        <template v-if="activeHistory">
          <n-avatar
            v-if="activeHistory.icon"
            class="bg-transparent!"
            :size="22"
            :src="activeHistory.icon"
          />

          <Icon
            v-else-if="activeHistory.type === 'command' || activeHistory.type === 'alias'"
            size="22"
            name="icon-minglinghang"
          />
        </template>

        <template v-else-if="isCategoryMode">
          <n-avatar
            v-if="activeCategory?.icon"
            class="bg-transparent!"
            :size="22"
            :src="activeCategory.icon"
          />
          <Icon v-else name="icon-fenlei1" size="22" />
        </template>

        <Icon v-else name="icon-sousuo" size="22" />
        <!-- <span style="font-size: 12px; color: #999">
          {{ currentStep }} --- {{ activeCategory?.name || "msj" }} --- {{ resultList.length }} ---
          {{ searchWindowHeight }}
        </span> -->
        <!-- <Icon v-else name="icon-icon-sousuofenlei" size="22" /> -->
      </template>

      <template #suffix>
        <!-- 自动补全提示 -->
        <div v-if="autocompleteEnabled" class="suggestion-con">
          <span class="suggestion-text">
            {{ currentAutocompleteSuggestion }}
          </span>

          <div class="shortcut-list mr-3">
            <span v-show="currentAutocompleteSuggestion !== keyword" class="shortcut-item">
              <Kbd>→</Kbd>
              <span class="text-xs ml-1">{{ t("search.autocomplete") }}</span>
            </span>

            <span v-show="resultList.length" class="shortcut-item">
              <Kbd>Ctrl + W</Kbd>
              <span class="text-xs ml-1">{{ t("search.closeResults") }}</span>
            </span>

            <span v-show="autocompleteList.length !== 1" class="shortcut-item">
              <Kbd>Tab</Kbd>
              <span class="text-xs ml-1">{{ t("search.switch") }}</span>
            </span>
          </div>
        </div>

        <div class="suggestion-con">
          <span class="suggestion-text"></span>

          <div class="shortcut-list mr-3">
            <!-- 历史记录提示 -->
            <template v-if="appConfigStore.showHistory">
              <div v-show="historyEnabled" class="shortcut-list">
                <span v-show="activeHistory" class="shortcut-item">
                  <Kbd>↵</Kbd>
                  <span class="text-xs ml-1">{{ t("search.confirm") }}</span>
                </span>

                <span class="shortcut-item">
                  <Kbd>↑↓</Kbd>
                  <span class="text-xs ml-1">{{ t("search.history") }}</span>
                </span>
              </div>
            </template>

            <!-- 分类搜索提示 -->
            <span
              class="shortcut-item"
              v-if="appConfigStore.enableDefaultSearchByCategory && keyword.trim() === ''"
            >
              <Kbd>Tab</Kbd>
              <span class="text-xs ml-1">{{ t("search.switchSearchMode") }}</span>
            </span>
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
    <template v-if="isSelectCategoryStep">
      <!-- prettier-ignore -->
      <template v-for="(item, index) of (resultList as WithDisabled<CategoryItem>[])" :key="item.id">
        <CategoryResultItem
          ref="itemRefs"
          :disabled="item.disabled"
          :item="item"
          :active="index === selectedIndex"
          @click="
            () => {
              selectedIndex = index;
              handleEnter();
            }
          "
        />
        <!-- @contextmenu="(event) => handleShowContextMenu(event, item)" -->
      </template>
    </template>

    <template v-else>
      <!-- prettier-ignore -->
      <template v-for="(item, index) of (resultList as SearchLauncItem[])" :key="item.id">
        <SearchResultItem
          ref="itemRefs"
          :item="item"
          :active="index === selectedIndex"
          @click="
            () => {
              selectedIndex = index;
              handleEnter();
            }
          "
          @contextmenu="(event) => handleShowContextMenu(event, item)"
        />
      </template>
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
import type { SwitchModePayload } from "../../searchModes";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import { nextTick, ref } from "vue";
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
} from "@/api";
import LaunchItemContextMenu from "@/components/ListItemContextMenu.vue";
import { useAppConfig, useNaiveUiApi } from "@/composables";
import {
  AppEvent,
  SEARCH_INPUT_HEIGHT,
  SEARCH_RESULT_ITEM_HEIGHT,
  SEARCH_WINDOW_WIDTH,
  TranslationOpenModel,
  WebSearchOpenModel,
} from "@/constant";
import { t } from "@/i18n";
import { EventBus } from "@/utils/eventBus";
import { SEARCH_MODEL } from "@/constant";
import SearchResultItem from "./components/SearchResultItem.vue";
import CategoryResultItem from "./components/CategoryResultItem.vue";
import { useStore } from "@/store/useStore";
import { storeToRefs } from "pinia";

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
  closeWindow: [isEscClose?: boolean];
  switchMode: [payload: SwitchModePayload];
}>();

const { appConfigStore } = useAppConfig();
const { notification } = useNaiveUiApi();

const { categoryData } = storeToRefs(useStore());

enum CurrentStep {
  DEFAULT = "default",
  SELECT_CATEGORY = "selectCategory", // Tab 切换到选择分类步骤
  CATEGORY = "category", // 确认选择分类后进入分类模式
}

enum Mode {
  DEFAULT = "default", // 默认模式 全量搜索
  CATEGORY = "category", // 分类模式 按照分类搜索
}

type WithDisabled<T> = T & {
  disabled?: boolean;
};

type ResultList = WithDisabled<SearchLauncItem>[] | WithDisabled<CategoryItem>[];

const currentStep = ref<CurrentStep>(CurrentStep.DEFAULT);
const isSelectCategoryStep = computed(() => currentStep.value === CurrentStep.SELECT_CATEGORY);
const currentMode = ref<Mode>(Mode.DEFAULT);
const isDefaultMode = computed(() => currentMode.value === Mode.DEFAULT);
const isCategoryMode = computed(() => currentMode.value === Mode.CATEGORY);

const searchWindow = getCurrentWindow();
const placeholder = computed(() => t("search.placeholder"));
const inputRef = useTemplateRef("searchInputRef");
const keyword = ref(props.keyword || "");
const resultList = ref<ResultList>([]);
const selectedIndex = ref(0);
const searchFlag = ref(false);
const autocompleteList = ref<string[]>([]);
const autocompleteIndex = ref(0);
const currentAutocompleteSuggestion = computed(
  () => autocompleteList.value[autocompleteIndex.value],
);
const hasResult = computed(() => !!resultList.value.length);
const chromeHeight = computed(() => props.chromeHeight);
const getPlaceholder = computed(() => {
  if (isSelectCategoryStep.value) return t("search.selectCategory");
  if (isCategoryMode.value) return activeCategory.value?.name;
  return activeHistory.value?.command || placeholder.value;
});

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
  name: "",
  path: "",
  type: "file",
  created_at: 0,
  updated_at: 0,
  launch_count: 0,
  failure_count: 0,
  pinyin_full: "",
  pinyin_abbr: "",
});
const contextMenuHeight = computed(() => {
  if (resultList.value.length === 1) return "80px";
  return resultList.value.length <= 3 ? `${50 + (resultList.value.length - 1) * 48}px` : "initial";
});

let spaceCounter = 0;
let searchRequestId = 0;

function focus() {
  inputRef.value?.focus();
}

function initParamsAndDataResult() {
  keyword.value = "";
  selectedIndex.value = 0;
  resultList.value = [];
  menuVisible.value = false;
  activeHistoryIndex.value = 0;
}

function handleClose() {
  // keyword.value = "";
  // selectedIndex.value = 0;
  // resultList.value = [];
  initParamsAndDataResult();

  menuVisible.value = false;
  currentStep.value = CurrentStep.DEFAULT;
  currentMode.value = Mode.DEFAULT;
  activeCategory.value = undefined;
  resizeWindow();
}

function handleChangeCurrentAutocomplete() {
  if (autocompleteList.value.length === 1) return;
  if (autocompleteList.value.length - 1 === autocompleteIndex.value) {
    autocompleteIndex.value = 0;
    return;
  }
  autocompleteIndex.value++;
}

function handleChangeHistory(type: "up" | "down") {
  if (!appConfigStore.showHistory) return;
  if (!historyData.value.length) return;
  if (activeHistoryIndex.value >= historyData.value.length && type === "up") return;
  if (activeHistoryIndex.value <= 0 && type === "down") return;

  if (type === "up") activeHistoryIndex.value += 1;
  else activeHistoryIndex.value -= 1;
}

function tryOpenWebSearch() {
  setTimeout(() => {
    if (!keyword.value.trim() || appConfigStore.webSearchOpenModel === WebSearchOpenModel.CLOSE)
      return;

    let flag = false;
    let key = "";
    if (appConfigStore.webSearchOpenModel === WebSearchOpenModel.KEY_SPACE) {
      flag = true;
      key = keyword.value.trim();
    } else if (appConfigStore.webSearchOpenModel === WebSearchOpenModel.COLON_KEY_SPACE) {
      if (keyword.value.trim().substring(0, 1) === ":") flag = true;
      key = keyword.value.trim().substring(1, keyword.value.trim().length);
    }
    if (!flag) return;

    const source = appConfigStore.webSearchSourceList.find(({ keywords }) => keywords === key);
    if (!source) return;

    emit("switchMode", { mode: SEARCH_MODEL.WEB_SEARCH_MODEL, source, from: "search" });
  }, 50);
}
// 连续按下2次 / 键 进入指定分类查询模式
// const enterCategoryCount = ref(0);
// const enableCategorySearch = ref(false);
// function handleEnterCategoryMode(e: KeyboardEvent) {
//   if (!isDefaultMode.value) return;
//   if (e.keyCode === 191) {
//     enterCategoryCount.value++;
//     if (enterCategoryCount.value === 2) {
//       enableCategorySearch.value = true;
//       enterCategoryCount.value = 0;
//       console.log("进入指定分类查询模式");
//       currentMode.value = Mode.CATEGORY;
//     }
//     return;
//   } else {
//     enterCategoryCount.value = 0;
//     enableCategorySearch.value = false;
//   }
// }

function handleSwitchMode(step?: CurrentStep) {
  keyword.value = "";
  selectedIndex.value = 0;
  resultList.value = [];
  autocompleteIndex.value = 0;
  activeHistoryIndex.value = 0;
  menuVisible.value = false;
  activeCategory.value = undefined;

  nextTick(() => {
    if (isDefaultMode.value || step === CurrentStep.SELECT_CATEGORY) {
      currentMode.value = Mode.CATEGORY;
      currentStep.value = CurrentStep.SELECT_CATEGORY;
      getCategoryData();
    } else {
      currentMode.value = Mode.DEFAULT;
      currentStep.value = CurrentStep.DEFAULT;
    }

    resizeWindow();
  });
}

function handleKeydown(e: KeyboardEvent) {
  const { keyCode, ctrlKey, key } = e;

  spaceCounter = keyCode === 32 ? spaceCounter + 1 : 0;

  if (ctrlKey && (key === "w" || key === "W")) {
    selectedIndex.value = 0;
    resultList.value = [];
    resizeWindow();
    e.preventDefault();
    return;
  }

  switch (keyCode) {
    case 9: // TAB
      if (autocompleteList.value.length && isDefaultMode.value && keyword.value.trim().length)
        handleChangeCurrentAutocomplete();
      else if (!keyword.value.trim().length) handleSwitchMode();
      e.preventDefault();
      break;
    case 13: // ENTER
      handleEnter();
      break;
    case 27: // ESC
      // 处于分类模式时 退到选择分类
      if (currentStep.value === CurrentStep.CATEGORY) {
        handleSwitchMode(CurrentStep.SELECT_CATEGORY);
      } else if (currentStep.value === CurrentStep.SELECT_CATEGORY) {
        handleSwitchMode();
      } else {
        emit("closeWindow", true);
      }
      break;
    case 32: // 空格
      if (
        appConfigStore.enableTranslation &&
        appConfigStore.translationOpenModel === TranslationOpenModel.THREE_HITS_ON_SPACES &&
        isDefaultMode.value &&
        spaceCounter === 3
      ) {
        emit("switchMode", {
          mode: SEARCH_MODEL.TRANSLATION_MODEL,
          keyword: keyword.value,
          from: "search",
        });
        break;
      }
      if (appConfigStore.enableWebSearch) tryOpenWebSearch();
      break;
    case 38: // 上
      if (historyEnabled.value) {
        handleChangeHistory("up");
      } else handleSelectIndexKeydownChange("up");
      e.preventDefault();
      break;
    case 39: // 右
      if (autocompleteEnabled.value) keyword.value = currentAutocompleteSuggestion.value;
      break;
    case 40: // 下
      if (historyEnabled.value) {
        handleChangeHistory("down");
      } else handleSelectIndexKeydownChange("down");
      e.preventDefault();
      break;
  }
}

const autocompleteEnabled = computed(() => autocompleteList.value.length && isDefaultMode.value);
const historyEnabled = computed(
  () =>
    appConfigStore.showHistory &&
    !isSelectCategoryStep.value &&
    !keyword.value.length &&
    isDefaultMode.value,
);
/**
 * 处理上下方向键切换选中项
 *
 * up:
 *  - 当前为第一项时跳转到最后一项（循环）
 *  - 否则向上移动一项
 *
 * down:
 *  - 当前为最后一项时跳转到第一项（循环）
 *  - 否则向下移动一项
 */
function handleSelectIndexKeydownChange(type: "up" | "down") {
  if (!resultList.value.length) return;

  selectedIndex.value = findNextEnabledIndex(selectedIndex.value, type);

  // // 当前结果总数
  // const resultCount = resultList.value.length;
  // // 最后一个元素索引
  // const maxIndex = resultCount - 1;
  // if (type === "up") {
  //   // 第一项继续向上 → 跳转到最后一项
  //   if (selectedIndex.value === 0 && resultCount) {
  //     selectedIndex.value = maxIndex;
  //   }
  //   // 普通向上移动
  //   else if (selectedIndex.value > 0) {
  //     selectedIndex.value--;
  //   }
  // } else if (type === "down") {
  //   // 最后一项继续向下 → 跳转到第一项
  //   if (selectedIndex.value === maxIndex && resultCount) {
  //     selectedIndex.value = 0;
  //   }
  //   // 普通向下移动
  //   else if (selectedIndex.value < maxIndex) {
  //     selectedIndex.value++;
  //   }
  // }
}

/**
 * 查找下一个可用索引
 *
 * @param current 当前索引
 * @param direction 方向
 * @returns 可用索引，不存在则返回原索引
 */
function findNextEnabledIndex(current: number, direction: "up" | "down") {
  const list = resultList.value;
  const count = list.length;

  if (!count) return current;

  let next = current;

  // 最多遍历一次列表
  for (let i = 0; i < count; i++) {
    next = direction === "up" ? (next - 1 + count) % count : (next + 1) % count;

    if (!list[next]?.disabled) {
      return next;
    }
  }

  // 全部禁用
  return current;
}

async function handleEnter() {
  if (isSelectCategoryStep.value) return handleCategoryEnter();
  await handleLaunchEnter();
}
const activeCategory = ref<CategoryItem>();
async function handleCategoryEnter() {
  const category = resultList.value[selectedIndex.value] as CategoryItem;
  if (!category) return;
  activeCategory.value = category;

  currentMode.value = Mode.CATEGORY;
  currentStep.value = CurrentStep.CATEGORY;
  // 切换到指定分类查询模式
  if (
    appConfigStore.enableCategorySearchDefaultData &&
    activeCategory.value &&
    resultList.value.length &&
    selectedIndex.value >= 0
  ) {
    initParamsAndDataResult();
    nextTick(() => {
      handleLaunchSearch(true);
    });
  } else {
    autocompleteIndex.value = 0;
    autocompleteList.value = [];
    activeHistoryIndex.value = 0;
    resultList.value = [];
    resizeWindow();
  }
}

async function handleLaunchEnter() {
  try {
    if (!keyword.value.length && isDefaultMode.value) {
      await handleHistoryEnterLaunch();
      emit("closeWindow");
      return;
    }

    if (!searchFlag.value) return;
    await handleEnterLaunch();
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

async function handleEnterLaunch() {
  if (!resultList.value.length) {
    await exeCommand(keyword.value);
    addOrUpdateAutocompleteRecord(keyword.value);
    if (appConfigStore.enableHistory) addLaunchHistory(keyword.value, "command");
    return;
  }

  const item = resultList.value[selectedIndex.value] as SearchLauncItem;
  if (!item) return;

  await runLaunch(item.id);
  EventBus.emit(AppEvent.UPDATE_LAUNCH_ITEM_COUNT, item.id);
  addOrUpdateAutocompleteRecord(keyword.value, item.id);
  if (appConfigStore.enableHistory) {
    addLaunchHistory(keyword.value || item.name, item.type, item.id);
  }
}

async function handleHistoryEnterLaunch() {
  if (!activeHistory.value) return;
  const { launch_item_id, command, type } = activeHistory.value;

  if (launch_item_id) {
    await runLaunch(launch_item_id);
    EventBus.emit(AppEvent.UPDATE_LAUNCH_ITEM_COUNT, launch_item_id);
    addOrUpdateAutocompleteRecord(command, launch_item_id);
    if (appConfigStore.enableHistory) addLaunchHistory(command, type, launch_item_id);
  } else {
    await exeCommand(command);
    addOrUpdateAutocompleteRecord(command);
    if (appConfigStore.enableHistory) addLaunchHistory(command, "command");
  }
}

async function handleSearch() {
  selectedIndex.value = 0;
  autocompleteIndex.value = 0;
  autocompleteList.value = [];
  activeHistoryIndex.value = 0;

  // 当处于 选择分类步骤时 不执行搜索
  if (currentStep.value === CurrentStep.SELECT_CATEGORY) return handleCategorySearch();
  await handleLaunchSearch();
}

function getCategoryData() {
  const data = categoryData.value.map((item) => ({
    ...item,
    disabled: item.exclude || false,
  }));
  if (!keyword.value.trim()) resultList.value = data;
  else resultList.value = data.filter((item) => item.name.includes(keyword.value));
}

async function handleCategorySearch() {
  // 前端实现分类的过滤
  getCategoryData();

  resizeWindow();
}

async function handleLaunchSearch(init?: boolean) {
  const currentId = ++searchRequestId;

  if (!isCategoryMode.value && !keyword.value.trim() && !init) {
    handleClose();
    return;
  }

  if (appConfigStore.enableAutocomplete && isDefaultMode.value) {
    getAutocomplete(keyword.value).then((res) => {
      if (currentId === searchRequestId) autocompleteList.value = res;
    });
  }

  if (
    isCategoryMode.value &&
    !appConfigStore.enableCategorySearchDefaultData &&
    !keyword.value.trim()
  )
    return;
  let launchs = await searchLaunch(keyword.value, activeCategory.value?.id);
  if (!searchFlag.value) searchFlag.value = true;
  if (!appConfigStore.enableCommandAlias) {
    // 处于分类查询模式时
    // if (activeCategory.value?.id) return;
    launchs = launchs.filter((item) => item.type !== "alias");
  }

  resultList.value = launchs;
  if (launchs.length < selectedIndex.value) selectedIndex.value = 0;

  if (currentId === searchRequestId) {
    resizeWindow();
  }

  // await getLaunchData(currentId);
}

async function handleShowContextMenu(e: MouseEvent, item: SearchLauncItem) {
  const { id, category_id, type } = item;
  if (type === "alias") return;

  const [launch, category] = await Promise.all([
    getLaunchByID(id),
    getCategoryByID(category_id!).catch(() => null),
  ]);
  categoryItem.value = category;
  if (!launch) {
    return notification.error({
      content: t("search.queryFailed"),
      meta: t("search.notFound"),
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

const itemRefs = useTemplateRef("itemRefs");
watch(selectedIndex, async (newIndex) => {
  await nextTick();
  // itemRefs.value[newIndex]?.scrollIntoView({ behavior: "smooth", block: "nearest" });
  itemRefs.value?.[newIndex]?.scrollToIntoView();
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

function getHisData() {
  if (appConfigStore.showHistory) {
    getRecentLaunchHistory().then((res) => (historyData.value = res));
  }
}

function handleBeforeShow() {
  getHisData();
}

onMounted(() => {
  nextTick(() => {
    focus();
    getHisData();
  });
});

defineExpose({
  focus,
  handleClose,
  handleBeforeShow,
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
  font-size: 12px !important;
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
  cursor: text;

  .suggestion-text {
    opacity: 0.3;
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
