<template>
  <!-- border border-gray-100 -->
  <Translation
    v-if="isTranslationModel"
    ref="translationRef"
    :keyword="tranStr"
    @close-window="handleClose"
  />
  <template v-else>
    <label class="input-container max-h-[45px]">
      <n-input
        ref="searchInputRef"
        v-model:value="keyword"
        tabindex="-1"
        type="text"
        size="medium"
        class="w-full h-full max-h-[45px] resize-none text-sm hover:outline-0 focus-visible:outline-0 border-none bg-white shadow-none rounded-[10px]"
        :class="hasResult ? '!border-b-0 !rounded-b-none' : ''"
        :placeholder="placeholder"
        @blur="handleBlur"
      >
        <!-- @keydown="handleKeydown" -->
        <!-- @keydown.up -->
        <template #prefix>
          <!-- {{ keyword }} -->
          <!-- {{ selectedIndex }} -->
          <!-- {{ selectedIndex }}--- {{ resultList.length }}--{{ hasResult }} -->
          <template v-if="isWebSearchModel">
            <n-avatar
              v-if="searchSourch!.icon"
              class="!bg-transparent"
              :size="22"
              :src="searchSourch!.icon"
            />
            <n-icon v-else :component="GlobeOutline" size="22" />
          </template>
          <n-icon v-else :component="SearchOutline" size="22" />
        </template>
      </n-input>
      <!-- v-show="keyword.length" && currentAutocompleteSuggestion !== keyword -->
      <div v-if="autocompleteList.length" class="suggestion-con">
        <span class="suggestion-text">
          {{ currentAutocompleteSuggestion }}
        </span>

        <div class="flex items-center gap-5 mr-3">
          <span
            v-show="!(autocompleteList.length === 1)"
            class="flex items-center"
          >
            <svg
              width="40"
              height="24"
              viewBox="0 0 40 24"
              xmlns="http://www.w3.org/2000/svg"
            >
              <!-- 背景矩形 -->
              <rect
                x="0"
                y="0"
                width="40"
                height="24"
                rx="4"
                ry="4"
                fill="#f9f9f9"
                stroke="#ccc"
                stroke-width="1"
              />

              <!-- Tab 文本 -->
              <text
                x="50%"
                y="50%"
                alignment-baseline="middle"
                text-anchor="middle"
                font-size="12"
                font-family="Arial, sans-serif"
                fill="#000"
              >
                Tab
              </text>
            </svg>
            <span class="text-xs ml-1">切换</span>
          </span>

          <span
            v-show="currentAutocompleteSuggestion !== keyword"
            class="flex items-center"
          >
            <svg
              width="40"
              height="24"
              viewBox="0 0 40 24"
              xmlns="http://www.w3.org/2000/svg"
            >
              <!-- 背景矩形 -->
              <rect
                x="0"
                y="0"
                width="40"
                height="24"
                rx="4"
                ry="4"
                fill="#f9f9f9"
                stroke="#ccc"
                stroke-width="1"
              />

              <!-- 横线（箭杆） -->
              <line
                x1="10"
                y1="12"
                x2="28"
                y2="12"
                stroke="#333"
                stroke-width="2"
                stroke-linecap="round"
              />

              <!-- 右箭头三角 -->
              <polygon points="26,8 36,12 26,16" fill="#333" />
            </svg>
            <span class="text-xs ml-1">补全</span>
          </span>
        </div>
      </div>
    </label>

    <ul
      v-if="hasResult"
      tabindex="-1"
      class="search-container absolute z-50 w-full overflow-y-scroll bg-white border-none rounded-b-[10px] !border-t-gray-200 max-h-[300px]"
      :style="{
        maxHeight: `calc(${searchWindowHeight}px - ${SEARCH_INPUT_HEIGHT}px)`,
      }"
    >
      <template v-for="(item, index) of resultList" :key="item.id">
        <!-- <LaunchItem
        :icon="item.icon!"
        :name="item.name"
      /> -->
        <li
          :ref="el => (itemRefs[index] = el as any)"
          class="flex items-center h-[48px] px-4 py-2 cursor-pointer"
          :class="[
            index === selectedIndex ? 'bg-[#f5f5f5]' : 'hover:bg-gray-100',
          ]"
          @click="
            () => {
              selectedIndex = index;
              handleEnter();
            }
          "
        >
          <!-- @mouseenter="selectedIndex = index" -->
          <img
            v-if="!isWebSearchModel"
            :src="item.icon || ''"
            alt="icon"
            class="!m-2 object-contain pointer-events-none w-8"
          />

          <span class="!ml-0.5">{{ item.name }}</span>

          <!-- TODO 个性化控制 分类是否显示 -->
          <span v-if="item.category_name" class="!ml-3">
            （{{ item.category_name }}）
          </span>
        </li>
      </template>
    </ul>
  </template>

  <!-- </div> -->
</template>

<script setup lang="ts">
import {
  cursorPosition,
  getCurrentWindow,
  LogicalPosition,
  LogicalSize,
} from '@tauri-apps/api/window';
import { fetch } from '@tauri-apps/plugin-http';
import { GlobeOutline, SearchOutline } from '@vicons/ionicons5';
import { nextTick, ref } from 'vue';
import {
  addOrUpdateAutocompleteRecord,
  exeCommand,
  getAutocomplete,
  searchLaunch,
} from '@/api';
import { useAppConfig } from '@/composables/useAppConfig';
import { useAppConfigActions } from '@/composables/useAppConfigActions';
import { useLaunchAction } from '@/composables/useLaunchAction';
import {
  AppEvent,
  SEARCH_INPUT_HEIGHT,
  SEARCH_RESULT_ITEM_HEIGHT,
  SEARCH_WINDOW_WIDTH,
  WebSearchOpenModel,
} from '@/constant';
import { EventBus } from '@/utils/eventBus';
import Translation from './components/Translation.vue';

const { runLaunch } = useLaunchAction();

const { appConfigStore } = useAppConfig();
// prettier-ignore
const placeTip = '名称 / 拼音 / 关键字 / 文件(目录)地址 / URL / Win内置命令 (mstsc)';
const placeholder = ref(placeTip);
const inputRef = useTemplateRef('searchInputRef');

const keyword = ref('');
const resultList = ref<SearchLauncItem[]>([]);
const current = getCurrentWindow();

const itemRefs = ref<HTMLElement[]>([]);
// 选中启动光标
const selectedIndex = ref(0);

const searchModel = ref<0 | 1 | 2>(0);
const isWebSearchModel = computed(() => searchModel.value === 1);
const isTranslationModel = computed(() => searchModel.value === 2);

const autocompleteList = ref<string[]>([]);
const autocompleteIndex = ref<number>(0);
const currentAutocompleteSuggestion = computed(
  () => autocompleteList.value[autocompleteIndex.value]
);

function handleChangeCurrentAutocomplete() {
  if (autocompleteList.value.length === 1) return;
  else if (autocompleteList.value.length - 1 === autocompleteIndex.value)
    return (autocompleteIndex.value = 0);
  autocompleteIndex.value++;
}

const spaceCounter = ref<number>(0);
const translationRef = useTemplateRef('translationRef');
function handleKeydown(e: KeyboardEvent) {
  const reultCount = resultList.value.length;
  const minIndex = 0;
  const maxIndex = reultCount - 1;
  const { keyCode } = e;
  // code
  // Enter keyCode=13 code=Enter
  // Esc   keyCode=27 code=Escape
  // 上箭头 keyCode=38 code=ArrowUp
  // 下箭头 keyCode=40 code=ArrowDown
  console.log('keyCode ------', keyCode);
  // 连续按下3次空格 进入翻译模式
  if (keyCode === 32) {
    spaceCounter.value++;
  } else {
    spaceCounter.value = 0;
  }

  switch (keyCode) {
    case 8:
      // prettier-ignore
      // isWebSearchModel.value &&
      // !keyword.value.length &&
      // handleToggleSearchModel(0);
      break;
    case 9: // Tab 按键 切换补全建议
      if (isTranslationModel.value) {
        translationRef.value?.handleChangeTranslationLanguage();
      } else {
        if (autocompleteList.value.length) handleChangeCurrentAutocomplete();
      }
      e.preventDefault();
      break;
    case 13:
      if (isTranslationModel.value) {
        translationRef.value?.handleEnter();
      } else {
        handleEnter();
      }
      break;
    case 27:
      // Esc 键 关闭搜索窗口
      // 当处于 web 搜索模式或者翻译模式时 按下esc 退出当前模式回到 快速搜索模式
      if (isWebSearchModel.value || isTranslationModel.value) {
        if (
          isTranslationModel.value &&
          translationRef.value?.isChangeTranslationLanguage
        ) {
          translationRef.value?.handleCloseChangeTranslationLanguage();
        } else {
          handleToggleSearchModel(0);
          // 处于翻译模式下需要多进行一次判断 判断是否处于语言切换操作中
          translationRef.value?.handleClose();
          nextTick(() => {
            inputRef.value?.focus();
          });
        }
      } else {
        handleClose();
      }
      break;
    case 32: // 空格键盘 判断是否呼出网络搜索
      if (spaceCounter.value === 3) handleToggleSearchModel(2);
      // prettier-ignore
      appConfigStore.enableWebSearch && !isWebSearchModel.value && handleOpenWebSearch();
      break;
    case 38: // 上移动按键
      if (isTranslationModel.value) {
        translationRef.value?.handleKeyUp();
      } else {
        if (selectedIndex.value === minIndex && reultCount)
          selectedIndex.value = reultCount - 1;
        else selectedIndex.value > 0 && selectedIndex.value--;
      }

      e.preventDefault();
      break;
    case 39: // 补全提示 补全关键字
      if (autocompleteList.value.length)
        keyword.value = currentAutocompleteSuggestion.value;
      break;
    case 40:
      if (isTranslationModel.value) {
        translationRef.value?.handleKeyDown();
      } else {
        if (selectedIndex.value === maxIndex && reultCount)
          selectedIndex.value = minIndex;
        else selectedIndex.value < maxIndex && selectedIndex.value++;
      }

      e.preventDefault();
      break;
  }
}

const searchSourch = ref<WebSearchSource>();
async function handleOpenWebSearch() {
  setTimeout(() => {
    if (!keyword.value.trim()) return;
    let flag = false;
    let key = '';
    if (appConfigStore.webSearchOpenModel === WebSearchOpenModel.KEY_SPACE) {
      flag = true;
      key = keyword.value.trim();
    } else if (
      appConfigStore.webSearchOpenModel === WebSearchOpenModel.COLON_KEY_SPACE
    ) {
      if (keyword.value.trim().substring(0, 1) === ':') flag = true;
      key = keyword.value.trim().substring(1, keyword.value.trim().length);
    }
    if (!flag) return;

    const searchSource = appConfigStore.webSearchSourceList.find(
      ({ keywords }) => keywords === key
    );

    if (!searchSource) return;
    searchSourch.value = searchSource;
    nextTick(() => {
      handleToggleSearchModel(1);
    });
  }, 50);
}

async function handleEnter() {
  if (!keyword.value.trim()) return;

  // 根据搜索模式调用不同的执行接口
  if (!isWebSearchModel.value) await handleEnterLaunch();
  else await handleEnterWebSearch();

  // 添加不全记录
  handleClose();
}

async function handleEnterLaunch() {
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
    await exeCommand(keyword.value);
    addOrUpdateAutocompleteRecord(keyword.value);
  } else {
    const item = resultList.value[selectedIndex.value];
    if (!item) return;
    // 执行启动
    await runLaunch(item.id);
    // TODO 根据返回结果进行统计 对应次数

    addOrUpdateAutocompleteRecord(keyword.value, item.id);
  }
}

async function handleEnterWebSearch() {
  const item = resultList.value[selectedIndex.value];

  const keywordStr =
    searchSourch.value?.searchApi?.replace(
      '{w}',
      item ? item.name : keyword.value
    ) || '';

  await exeCommand(keywordStr!);
}

const tranStr = ref<string>('');

function handleToggleSearchModel(newModel: 0 | 1 | 2) {
  // 当切换的为 翻译模式 记录当前输入框字符串
  if (newModel === 2) tranStr.value = keyword.value;
  searchModel.value = newModel;
  keyword.value = '';
  resultList.value = [];
  selectedIndex.value = 0;
  placeholder.value = newModel ? searchSourch.value?.desc || '' : placeTip;
}

// 调用web搜索的搜索建议接口
async function searchSuggestion(): Promise<SearchLauncItem[]> {
  // 部分网络搜索没有 搜索建议接口 所以不支持 直接返回 return
  // TODO 根据用户传入的自定义的eval函数来 获取到
  if (!searchSourch.value?.suggestionApi) return [];
  let result = [];

  const userCode = `
    const url = searchSourch?.suggestionApi?.replace('{w}', encodeURIComponent(keyword));
    const data = await fetch(url).then(res => res.json());

    return data[1].map((item, i) => ({
      id: i,
      name: item,
      category_name: '',
      subcategory_name: ''
    }));
  `;

  async function runUserCode(userCode: string) {
    // 用 async Function 创建沙箱作用域
    // eslint-disable-next-line no-new-func
    const fn = new Function(
      'fetch',
      'searchSourch',
      'keyword',
      `"use strict"; return (async () => { ${userCode} })();`
    );
    const result = await fn(fetch, searchSourch.value, keyword.value);
    return result;
  }
  result = await runUserCode(userCode);

  return result;
}

function handleClose() {
  searchModel.value = 0;
  // 清空输入框
  keyword.value = '';
  resultList.value = [];
  selectedIndex.value = 0;
  placeholder.value = placeTip;
  // 隐藏搜索窗口
  // current.setSize(new LogicalSize(600, 45))
  current.hide();
}

async function handleShow() {
  // 当存在多个显示器时 将搜索窗口显示在鼠标停留的显示器上
  const { x, y } = await cursorPosition();

  // TODO 适配多显示器上下排版呼出
  // TODO 可作为个性化设置 搜索框呼出位置跟随鼠标 需要适配搜索结果显示位置 朝上或者朝下
  // await current.setPosition(new LogicalPosition(x, y))
  const { width } = await current.innerSize();
  // 存在多个显示器时 鼠标边缘呼出适配
  await current.setPosition(new LogicalPosition(x - width / 2, y));
  await current.center();

  // 显示搜索窗口
  await current.show();
  // 窗口聚焦
  await current.setFocus();
  // 输入框聚焦
  inputRef.value?.focus();
}

// const searchWindowHeight = ref<number>(300)
// console.log('searchWindowHeight ------', searchWindowHeight.value)
// 当使用箭头控制选项是 自动将预选项滚到可视窗口内
watch(selectedIndex, async newIndex => {
  await nextTick();
  const el = itemRefs.value[newIndex];
  el?.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
});

const searchWindowHeight = computed(() => {
  if (!resultList.value.length) return SEARCH_INPUT_HEIGHT;

  // 结果列表总高度 + 1像素的的顶部边框高度
  const resultsHeight = resultList.value.length * SEARCH_RESULT_ITEM_HEIGHT;

  return resultsHeight + SEARCH_INPUT_HEIGHT >
    appConfigStore.searchWindowMaxHeight
    ? appConfigStore.searchWindowMaxHeight
    : resultsHeight + SEARCH_INPUT_HEIGHT + 1;
});
let searchRequestId = 0;
watch(
  () => keyword.value,
  async keyword => {
    const currentId = ++searchRequestId;

    autocompleteIndex.value = 0;
    autocompleteList.value = [];

    if (!keyword.trim()) {
      selectedIndex.value = 0;
      resultList.value = [];
      return current.setSize(
        new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value)
      );
    }

    // 根据当前搜索模式 调用不同的搜索接口
    let launchs: SearchLauncItem[] = [];
    if (isWebSearchModel.value) {
      launchs = await searchSuggestion();
    } else {
      if (appConfigStore.enableAutocomplete) {
        getAutocomplete(keyword).then(res => {
          if (currentId === searchRequestId) {
            autocompleteList.value = res;
          }
        });
      }
      launchs = await searchLaunch(keyword);
    }
    resultList.value = launchs;

    if (currentId === searchRequestId) {
      current.setSize(
        new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value)
      );
    }
  }
);

function handleBlur() {
  if (appConfigStore.searchLostFocusHide) handleClose();
}

const hasResult = computed(() => !!resultList.value.length);

useAppConfigActions().registerSearchShortcutKey();
// 监听快捷键按下
EventBus.listen(AppEvent.SEARCH_SHORTCU_KEY, async () => {
  if (!appConfigStore.enableSearch) return;
  const windowVisible = await current.isVisible();
  windowVisible ? handleClose() : handleShow();
});

onMounted(() => {
  window.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
});
</script>

<style scoped lang="scss">
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

  border-radius: 5px;
  border: none !important;
  /* border-color: !important; */
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
  /* max-height: calc(v-bind(searchWindowHeight + 'px') - v-bind(SEARCH_INPUT_HEIGHT + 'px')); */
  box-sizing: border-box;
  border-top: 0.5px solid;
  border-radius: 0 0 5px 5px;
}

ul:focus-visible {
  outline: none !important; /* 例如，取消焦点时的轮廓 */
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
  /* z-index: -1; */
  .suggestion-text {
    // position: absolute;
    // left: 38px;
    // top: 7px;
    margin-left: 38px;
    width: fit-content;
  }
}
</style>

<style>
.n-config-provider {
  height: 100%;
}
</style>
