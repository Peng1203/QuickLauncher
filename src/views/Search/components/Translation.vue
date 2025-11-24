<template>
  <label class="input-container max-h-[45px]">
    <n-input
      ref="searchInputRef"
      v-model:value="tranStr"
      tabindex="-1"
      type="text"
      size="medium"
      class="w-full h-full max-h-[45px] resize-none text-sm hover:outline-0 focus-visible:outline-0 border-none bg-white shadow-none rounded-[10px]"
      :class="hasResult ? '!border-b-0 !rounded-b-none' : ''"
      :placeholder="placeholder"
    >
      <!-- @keydown="handleKeydown" -->
      <!-- @keydown.up -->
      <template #prefix>
        <!-- {{ hasResult }} {{ resultList.length }} -->
        <svg
          t="1763950060235"
          class="icon"
          viewBox="0 0 1024 1024"
          version="1.1"
          xmlns="http://www.w3.org/2000/svg"
          p-id="1398"
          width="22"
          height="22"
        >
          <path
            d="M414.254545 595.781818H172.218182c-86.109091 0-155.927273-67.490909-155.927273-148.945454V218.763636C16.290909 137.309091 86.109091 69.818182 172.218182 69.818182h242.036363c86.109091 0 155.927273 67.490909 155.927273 148.945454v228.072728c-2.327273 81.454545-69.818182 148.945455-155.927273 148.945454zM172.218182 137.309091c-48.872727 0-86.109091 34.909091-86.109091 81.454545v228.072728c0 44.218182 39.563636 81.454545 86.109091 81.454545h242.036363c48.872727 0 86.109091-34.909091 86.109091-81.454545V218.763636c0-44.218182-39.563636-81.454545-86.109091-81.454545H172.218182z"
            fill="#666666"
            p-id="1399"
          />
          <path
            d="M837.818182 861.090909H595.781818c-90.763636 0-155.927273-69.818182-155.927273-167.563636v-141.963637c0-18.618182 16.290909-34.909091 34.909091-34.909091s34.909091 16.290909 34.909091 34.909091v141.963637c0 58.181818 34.909091 100.072727 86.109091 100.072727H837.818182c48.872727 0 86.109091-34.909091 86.109091-81.454545v-228.072728c0-44.218182-39.563636-81.454545-86.109091-81.454545H544.581818c-18.618182 0-34.909091-16.290909-34.909091-34.909091s16.290909-34.909091 34.909091-34.909091H837.818182c86.109091 0 155.927273 67.490909 155.927273 148.945455v228.072727c0 86.109091-69.818182 151.272727-155.927273 151.272727zM262.981818 847.127273c-102.4 0-183.854545-74.472727-183.854545-167.563637 0-18.618182 16.290909-34.909091 34.909091-34.909091s34.909091 16.290909 34.909091 34.909091c0 55.854545 51.2 100.072727 116.363636 100.072728 18.618182 0 34.909091 16.290909 34.909091 34.909091-4.654545 18.618182-18.618182 32.581818-37.236364 32.581818zM861.090909 281.6c-18.618182 0-34.909091-16.290909-34.909091-34.909091 0-55.854545-51.2-100.072727-116.363636-100.072727-18.618182 0-34.909091-16.290909-34.909091-34.909091s16.290909-34.909091 34.909091-34.909091c102.4 0 183.854545 74.472727 183.854545 167.563636 2.327273 20.945455-11.636364 37.236364-32.581818 37.236364z"
            fill="#666666"
            p-id="1400"
          />
          <path
            d="M660.945455 686.545455h-39.563637l88.436364-165.236364h41.890909l88.436364 165.236364h-41.89091l-23.272727-46.545455h-93.090909l-20.945454 46.545455z m69.818181-139.636364l-37.236363 72.145454H768l-37.236364-72.145454z"
            fill="#303030"
            p-id="1401"
          />
          <path
            d="M286.254545 200.145455h23.272728v39.563636H395.636364V349.090909h-23.272728v-13.963636h-62.836363v76.8h-23.272728v-76.8H223.418182v13.963636h-23.272727v-109.381818h86.10909V200.145455z m-62.836363 116.363636h62.836363v-55.854546H223.418182v55.854546z m86.109091 0H372.363636v-55.854546h-62.836363v55.854546z"
            fill="#303030"
            p-id="1402"
          />
        </svg>
      </template>
    </n-input>

    <div class="suggestion-con">
      <div class="flex items-center gap-5 mr-3">
        <span class="flex items-center">
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
          <span class="text-xs ml-1">切换翻译语言</span>
        </span>

        <span class="flex items-center">
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

            <!-- 主干：从左到右，然后向上竖一段 -->
            <path
              d="M10 12 H28 V8"
              fill="none"
              stroke="#333"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            />

            <!-- 左侧箭头三角（←） -->
            <polygon points="6,12 10,8 10,16" fill="#333" />
          </svg>

          <span class="text-xs ml-1">复制选中</span>
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
    <template v-for="(item, index) of resultList" :key="item.value">
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
        <span class="!ml-0.5">{{ item.label }}</span>
      </li>
    </template>
  </ul>
</template>

<script setup lang="ts">
import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
// import { useNaiveUiApi } from '@/composables/useNaiveUiApi';
import { fetch } from '@tauri-apps/plugin-http';
import { MD5 } from 'crypto-js';
import { ref } from 'vue';
import { useAppConfig } from '@/composables/useAppConfig';
import {
  BAIDU_TRANSLATION_TO,
  SEARCH_INPUT_HEIGHT,
  SEARCH_RESULT_ITEM_HEIGHT,
  SEARCH_WINDOW_WIDTH,
} from '@/constant';

const props = defineProps<{ keyword: string }>();
const emits = defineEmits(['closeWindow']);

const { appConfigStore } = useAppConfig();

const current = getCurrentWindow();
const tranStr = ref('');
const placeholder = ref('');
const itemRefs = ref<HTMLElement[]>([]);
const inputRef = useTemplateRef('searchInputRef');
const selectedIndex = ref(0);

const resultList = ref<OptionItem[]>([]);
const hasResult = computed(() => !!resultList.value.length);
const initFlag = ref<boolean>(false);

// 通过tab切换选中的翻译目标语言
const selectedTranslationLanguage = ref<string>('');
const isChangeTranslationLanguage = ref<boolean>(false);

// 动态计算 搜索窗口的总高度
const searchWindowHeight = computed(() => {
  if (!resultList.value.length) return SEARCH_INPUT_HEIGHT;

  // 结果列表总高度 + 1像素的的顶部边框高度
  const resultsHeight = resultList.value.length * SEARCH_RESULT_ITEM_HEIGHT;

  return resultsHeight + SEARCH_INPUT_HEIGHT >
    appConfigStore.searchWindowMaxHeight
    ? appConfigStore.searchWindowMaxHeight
    : resultsHeight + SEARCH_INPUT_HEIGHT + 1;
});

function isChinese(text: string) {
  return /[\u4E00-\u9FA5]/.test(text);
}

function toCamelCase(str: string) {
  return str
    .toLowerCase()
    .replace(/[-_\s]+(.)?/g, (_, c) => (c ? c.toUpperCase() : ''));
}

function toSnakeCase(str: string) {
  return str.trim().toLowerCase().replace(/\s+/g, '_').replace(/-/g, '_');
}

async function baiduTranslate() {
  try {
    const { BDTranslationAppid, BDTranslationKey } = appConfigStore;
    if (!BDTranslationAppid || !BDTranslationKey) return;

    const salt = `${Date.now()}`;
    const { from, to } = getFromTo();
    const { BDTranslationAppid: appid, BDTranslationKey: key } = appConfigStore;

    const sign = `${MD5(appid + tranStr.value + salt + key)}`;

    const params = {
      q: tranStr.value,
      from,
      to,
      appid,
      salt,
      sign,
    };
    const queryString = new URLSearchParams(params).toString();

    const res = await fetch(
      `https://fanyi-api.baidu.com/api/trans/vip/translate?${queryString}`,
      { method: 'GET' }
    ).then(res => res.json());

    const { dst = '' } = res.trans_result?.[0];
    if (!dst) return [];

    // 非英文状态下只返回一个结果
    if (to !== 'en') {
      return [{ label: dst, value: dst }];
    }

    // TODO 根据用户配置选择是否开启
    const camel = toCamelCase(dst);
    const snake = toSnakeCase(dst);

    const results = [
      { label: dst, value: dst }, // 正常
      { label: dst.toUpperCase(), value: dst.toUpperCase() }, // 全大写
      { label: dst.toLowerCase(), value: dst.toLowerCase() }, // 全小写
      { label: camel, value: camel }, // 驼峰
      { label: snake, value: snake }, // 下划线
    ];
    const map = new Map();
    return results.filter(v => !map.has(v.value) && map.set(v.value, 1));
    // const seen = new Set();
    // return results.filter(i => !seen.has(i.value) && seen.add(i.value));
    // return [{ label: dst, value: dst }];
  } catch (e) {
    console.log('e', e);
    return [];
  }
}

function getFromTo() {
  const from = 'auto';
  let to = selectedTranslationLanguage.value || appConfigStore.BDTranslationTo;
  if (isChinese(tranStr.value)) return { from, to };
  // 当输入的文本非中文时 将其翻译至中文
  to = selectedTranslationLanguage.value || 'zh';
  return { from, to };
}

function handleEnter() {
  if (!resultList.value.length) return;
  const activeRes = resultList.value[selectedIndex.value];

  if (isChangeTranslationLanguage.value) {
    selectedTranslationLanguage.value = activeRes.value as string;
    isChangeTranslationLanguage.value = false;
    resultList.value = [];
    current.setSize(
      new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value)
    );
    // 根据当前 输入框中是否有内容 决定是否调用翻译接口
    if (tranStr.value.trim().length) {
      baiduTranslate().then((res: any) => {
        resultList.value = res;
        current.setSize(
          new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value)
        );

        selectedIndex.value = 0;
      });
    }
  } else {
    writeText(activeRes.value as string);
    handleClose();
    // 通过配置控制 复制成功后是否关闭
    emits('closeWindow');
  }
}

let timer: any = null;
let searchRequestId = 0;
watch(
  () => tranStr.value,
  tranStr => {
    if (initFlag.value) return;

    clearTimeout(timer);
    timer = setTimeout(async () => {
      const currentId = ++searchRequestId;
      selectedIndex.value = 0;

      if (!tranStr.trim()) {
        resultList.value = [];
        return current.setSize(
          new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value)
        );
      }

      // 根据当前搜索模式 调用不同的搜索接口
      resultList.value = (await baiduTranslate()) as OptionItem[];

      if (currentId === searchRequestId) {
        current.setSize(
          new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value)
        );
      }
    }, 600);
  }
);

watch(selectedIndex, async newIndex => {
  await nextTick();
  const el = itemRefs.value[newIndex];
  el?.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
});
function handleClose() {
  selectedIndex.value = 0;
  resultList.value = [];
  return current.setSize(
    new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value)
  );
}

function handleKeyUp() {
  if (selectedIndex.value === 0 && resultList.value.length)
    selectedIndex.value = resultList.value.length - 1;
  else selectedIndex.value > 0 && selectedIndex.value--;
}
function handleKeyDown() {
  if (
    selectedIndex.value === resultList.value.length - 1 &&
    resultList.value.length
  ) {
    selectedIndex.value = 0;
  } else {
    selectedIndex.value < resultList.value.length - 1 && selectedIndex.value++;
  }
}

// 原始翻译返回的结果
const originTranslationReslut = ref<OptionItem[]>([]);
// 通过Tab按键触发 切换翻译语言
function handleChangeTranslationLanguage() {
  isChangeTranslationLanguage.value = true;
  originTranslationReslut.value = JSON.parse(JSON.stringify(resultList.value));
  resultList.value = BAIDU_TRANSLATION_TO;
  selectedIndex.value = 0;
  current.setSize(
    new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value)
  );
}
// 当处于切换翻译语言操作中 按下 esc 取消 进行翻译状态恢复
function handleCloseChangeTranslationLanguage() {
  isChangeTranslationLanguage.value = false;
  resultList.value = originTranslationReslut.value;
  selectedIndex.value = 0;
  current.setSize(
    new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value)
  );
}

onMounted(() => {
  if (props.keyword.trim().length) {
    tranStr.value = props.keyword.trim();
    initFlag.value = true;
    baiduTranslate().then((res: any) => {
      resultList.value = res;
      current.setSize(
        new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value)
      );

      initFlag.value = false;
    });
  }

  setTimeout(() => {
    inputRef.value?.focus();
  }, 50);
  // nextTick(() => {});
});

defineExpose({
  isChangeTranslationLanguage,
  handleEnter,
  handleClose,
  handleKeyUp,
  handleKeyDown,
  handleChangeTranslationLanguage,
  handleCloseChangeTranslationLanguage,
});
</script>

<style lang="scss" scoped>
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
  justify-content: end;
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
