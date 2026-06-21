<template>
  <label class="input-container max-h-[45px]">
    <n-input
      ref="searchInputRef"
      v-model:value="tranStr"
      tabindex="-1"
      type="text"
      size="medium"
      class="w-full h-full max-h-[45px] resize-none text-sm hover:outline-0 focus-visible:outline-0 border-none bg-card shadow-none rounded-[10px]"
      :class="hasResult ? '!border-b-0 !rounded-b-none' : ''"
      :placeholder="placeholder"
    >
      <template #prefix>
        <n-icon size="22" class="iconfont icon-fanyi" />
      </template>

      <template #suffix>
        <div class="shortcut-list">
          <template v-if="showDictButton && !isChangeTranslationLanguage">
            <span
              class="shortcut-item cursor-pointer hover:text-blue-500"
              @click="handleToggleDictionary"
            >
              <Kbd>Ctrl + D</Kbd>
              <span class="text-xs ml-1">{{ t("translation.dictionary") }}</span>
            </span>
          </template>
          <span class="shortcut-item">
            <Kbd>Tab</Kbd>
            <span class="text-xs ml-1">{{ t("translation.switchLang") }}</span>
          </span>

          <span class="shortcut-item">
            <Kbd>↵</Kbd>
            <span class="text-xs ml-1">{{ t("translation.copySelected") }}</span>
          </span>
        </div>
      </template>
    </n-input>
  </label>

  <!-- 词典详情视图 -->
  <div
    v-if="showDictionary"
    class="absolute z-50 w-full bg-card rounded-b-[10px] overflow-y-auto"
    :style="{
      maxHeight: `calc(${searchWindowHeight}px - ${chromeHeight}px - ${SEARCH_INPUT_HEIGHT}px)`,
    }"
  >
    <!-- 骨架屏加载效果 -->
    <div v-if="dictionaryLoading" class="p-4 space-y-3">
      <div class="flex items-center gap-2">
        <div class="w-6 h-6 bg-gray-200 rounded animate-pulse"></div>
        <div class="h-6 w-24 bg-gray-200 rounded animate-pulse"></div>
        <div class="h-4 w-16 bg-gray-200 rounded animate-pulse"></div>
      </div>
      <div class="flex gap-2">
        <div class="h-5 w-12 bg-gray-200 rounded animate-pulse"></div>
        <div class="h-5 w-16 bg-gray-200 rounded animate-pulse"></div>
      </div>
      <div class="space-y-2">
        <div class="h-4 w-full bg-gray-200 rounded animate-pulse"></div>
        <div class="h-4 w-3/4 bg-gray-200 rounded animate-pulse"></div>
        <div class="h-4 w-5/6 bg-gray-200 rounded animate-pulse"></div>
      </div>
      <div class="border-t pt-3 mt-3">
        <div class="h-4 w-20 bg-gray-200 rounded animate-pulse mb-2"></div>
        <div class="flex gap-2">
          <div class="h-5 w-14 bg-gray-200 rounded animate-pulse"></div>
          <div class="h-5 w-18 bg-gray-200 rounded animate-pulse"></div>
          <div class="h-5 w-12 bg-gray-200 rounded animate-pulse"></div>
        </div>
      </div>
      <div class="border-t pt-3 mt-3">
        <div class="h-4 w-16 bg-gray-200 rounded animate-pulse mb-2"></div>
        <div class="h-16 w-full bg-gray-200 rounded animate-pulse"></div>
      </div>
    </div>
    <!-- 词典内容 -->
    <DictionaryCard v-else-if="dictionaryData" :data="dictionaryData" />
  </div>

  <!-- 翻译结果列表 -->
  <ul
    v-else-if="hasResult"
    tabindex="-1"
    class="search-container absolute z-50 w-full overflow-y-scroll bg-card border-none rounded-b-[10px] !border-t-border max-h-[300px]"
    :style="{
      maxHeight: `calc(${searchWindowHeight}px - ${chromeHeight}px - ${SEARCH_INPUT_HEIGHT}px)`,
    }"
  >
    <template v-for="(item, index) of resultList" :key="item.value">
      <li
        :ref="(el) => (itemRefs[index] = el as any)"
        class="flex items-center h-[48px] px-4 py-2 cursor-pointer"
        :class="[index === selectedIndex ? 'bg-muted' : 'hover:bg-muted']"
        @click="
          () => {
            selectedIndex = index;
            handleEnter();
          }
        "
      >
        <span class="flex-1 !ml-0.5">{{ item.label }}</span>
        <span
          v-if="index === 0 && showDictButton"
          class="text-gray-400 hover:text-blue-500 px-2"
          :title="t('translation.dictionary')"
          @click.stop="handleToggleDictionary"
        >
          📖
        </span>
      </li>
    </template>
  </ul>
</template>

<script setup lang="ts">
import type { SearchModelType } from "../searchModes";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { MD5 } from "crypto-js";
import { ref, watch, nextTick } from "vue";
import { fetch as tauriFetch } from "@tauri-apps/plugin-http";
import { useAppConfig, useNaiveUiApi } from "@/composables";
import {
  BAIDU_TRANSLATION_TO,
  SEARCH_INPUT_HEIGHT,
  SEARCH_RESULT_ITEM_HEIGHT,
  SEARCH_WINDOW_WIDTH,
} from "@/constant";
import { t } from "@/i18n";
import DictionaryCard from "@/components/DictionaryCard.vue";

const props = defineProps<{ keyword?: string; chromeHeight?: number }>();
const emit = defineEmits<{
  closeWindow: [];
  switchMode: [payload: { mode: SearchModelType; keyword?: string; source?: WebSearchSource }];
}>();

const { appConfigStore } = useAppConfig();

const current = getCurrentWindow();
const tranStr = ref("");
const placeholder = ref("");
const itemRefs = ref<HTMLElement[]>([]);
const inputRef = useTemplateRef("searchInputRef");
const selectedIndex = ref(0);

const resultList = ref<OptionItem[]>([]);
const hasResult = computed(() => !!resultList.value.length);
const initFlag = ref<boolean>(false);
const chromeHeight = computed(() => props.chromeHeight || 0);

// 通过tab切换选中的翻译目标语言
const selectedTranslationLanguage = ref<string>("");
const isChangeTranslationLanguage = ref<boolean>(false);

// 词典相关
const showDictionary = ref(false);
const dictionaryData = ref<DictionaryData | null>(null);
const dictionaryLoading = ref(false);

// 判断是否中英互译
const isEnglishToChinese = computed(() => {
  const { from, to } = getFromTo();
  return (from === "auto" && to === "zh") || to === "zh";
});

const isChineseToEnglish = computed(() => {
  return isChinese(tranStr.value) && !isChangeTranslationLanguage.value;
});

const isDictionaryMode = computed(() => {
  return (isEnglishToChinese.value || isChineseToEnglish.value) && resultList.value.length > 0;
});

const showDictButton = computed(() => {
  return isDictionaryMode.value && !isChangeTranslationLanguage.value;
});

// 动态计算 搜索窗口的总高度
const searchWindowHeight = computed(() => {
  // 词典模式使用固定高度
  if (showDictionary.value) {
    const dictHeight = dictionaryLoading.value ? 300 : 400;
    return chromeHeight.value + SEARCH_INPUT_HEIGHT + dictHeight;
  }

  if (!resultList.value.length) return chromeHeight.value + SEARCH_INPUT_HEIGHT;

  // 结果列表总高度 + 1像素的的顶部边框高度
  const resultsHeight = resultList.value.length * SEARCH_RESULT_ITEM_HEIGHT;

  const contentHeight = resultsHeight + SEARCH_INPUT_HEIGHT;
  return (
    chromeHeight.value +
    (contentHeight > appConfigStore.searchWindowMaxHeight
      ? appConfigStore.searchWindowMaxHeight
      : contentHeight + 1)
  );
});

function isChinese(text: string) {
  return /[\u4E00-\u9FA5]/.test(text);
}

function toCamelCase(str: string) {
  return str.toLowerCase().replace(/[-_\s]+(.)?/g, (_, c) => (c ? c.toUpperCase() : ""));
}

function toSnakeCase(str: string) {
  return str.trim().toLowerCase().replace(/\s+/g, "_").replace(/-/g, "_");
}

const { message } = useNaiveUiApi();

async function baiduTranslate() {
  try {
    const { BDTranslationAppid, BDTranslationKey } = appConfigStore;
    if (!BDTranslationAppid || !BDTranslationKey) {
      message.warning(t("search.baiduConfigWarning"));
      return [];
    }

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

    const res = await tauriFetch(
      `https://fanyi-api.baidu.com/api/trans/vip/translate?${queryString}`,
      {
        method: "GET",
        headers: {
          Accept: "application/json",
        },
      },
    ).then((res) => res.json());

    if (!res.trans_result?.length) return;
    const { dst = "" } = res.trans_result[0];
    if (!dst) return [];

    // 非英文状态下只返回一个结果
    if (to !== "en") {
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
    return results.filter((v) => !map.has(v.value) && map.set(v.value, 1));
    // const seen = new Set();
    // return results.filter(i => !seen.has(i.value) && seen.add(i.value));
    // return [{ label: dst, value: dst }];
  } catch (e) {
    console.log("e", e);
    return [];
  }
}

function getFromTo() {
  const from = "auto";
  let to = selectedTranslationLanguage.value || appConfigStore.BDTranslationTo;
  if (isChinese(tranStr.value)) return { from, to };
  // 当输入的文本非中文时 将其翻译至中文
  to = selectedTranslationLanguage.value || "zh";
  return { from, to };
}

function handleEnter() {
  if (!resultList.value.length) return;
  const activeRes = resultList.value[selectedIndex.value];

  if (isChangeTranslationLanguage.value) {
    selectedTranslationLanguage.value = activeRes.value as string;
    isChangeTranslationLanguage.value = false;
    resultList.value = [];
    current.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value));
    // 根据当前 输入框中是否有内容 决定是否调用翻译接口
    if (tranStr.value.trim().length) {
      baiduTranslate().then((res: any) => {
        resultList.value = res;
        current.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value));

        selectedIndex.value = 0;
      });
    }
  } else {
    writeText(activeRes.value as string);
    handleClose();
    // 通过配置控制 复制成功后是否关闭
    emit("closeWindow");
  }
}

let timer: any = null;
let searchRequestId = 0;
watch(
  () => tranStr.value,
  (tranStr) => {
    if (initFlag.value) return;

    clearTimeout(timer);
    timer = setTimeout(async () => {
      const currentId = ++searchRequestId;
      selectedIndex.value = 0;

      if (!tranStr.trim()) {
        resultList.value = [];
        showDictionary.value = false;
        dictionaryData.value = null;
        dictionaryLoading.value = false;
        nextTick(() => {
          current.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value));
        });
        return;
      }

      // 输入内容变化时退出词典模式
      if (showDictionary.value) {
        showDictionary.value = false;
        dictionaryData.value = null;
        dictionaryLoading.value = false;
        nextTick(() => {
          current.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value));
        });
      }

      // 根据当前搜索模式 调用不同的搜索接口
      resultList.value = (await baiduTranslate()) as OptionItem[];

      if (currentId === searchRequestId) {
        current.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value));
      }
    }, 600);
  },
);

watch(selectedIndex, async (newIndex) => {
  await nextTick();
  const el = itemRefs.value[newIndex];
  el?.scrollIntoView({ behavior: "smooth", block: "nearest" });
});

watch(chromeHeight, () => {
  current.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value));
});
function handleClose() {
  tranStr.value = "";
  selectedIndex.value = 0;
  resultList.value = [];
  showDictionary.value = false;
  dictionaryData.value = null;
  return current.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value));
}

async function handleToggleDictionary() {
  if (showDictionary.value) {
    // 切换回翻译结果
    showDictionary.value = false;
    dictionaryData.value = null;
    dictionaryLoading.value = false;
    current.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value));
    return;
  }

  if (!resultList.value.length) return;

  // 获取要查询的单词：输入是中文则用翻译结果，输入是英文则用输入
  const queryWord = isChinese(tranStr.value) ? resultList.value[0]?.value : tranStr.value.trim();

  if (!queryWord) return;

  // 立即切换到词典视图并显示骨架屏
  showDictionary.value = true;
  dictionaryLoading.value = true;
  dictionaryData.value = null;

  // 调整窗口高度
  const dictHeight = 300; // 骨架屏预估高度
  current.setSize(
    new LogicalSize(SEARCH_WINDOW_WIDTH, chromeHeight.value + SEARCH_INPUT_HEIGHT + dictHeight),
  );

  // 调用词典 API
  const apiData = await fetchDictionary(queryWord as string);
  if (apiData) {
    dictionaryData.value = convertToDictionaryData(apiData);
  }

  dictionaryLoading.value = false;
}

function handleKeyUp() {
  if (selectedIndex.value === 0 && resultList.value.length)
    selectedIndex.value = resultList.value.length - 1;
  else if (selectedIndex.value > 0) selectedIndex.value--;
}
function handleKeyDown() {
  if (selectedIndex.value === resultList.value.length - 1 && resultList.value.length) {
    selectedIndex.value = 0;
  } else {
    if (selectedIndex.value < resultList.value.length - 1) selectedIndex.value++;
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
  current.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value));
}
// 当处于切换翻译语言操作中 按下 esc 取消 进行翻译状态恢复
function handleCloseChangeTranslationLanguage() {
  isChangeTranslationLanguage.value = false;
  resultList.value = originTranslationReslut.value;
  selectedIndex.value = 0;
  current.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value));
}

function handleKeydown(e: KeyboardEvent) {
  // Ctrl+D 切换词典
  if (e.ctrlKey && (e.key === "d" || e.key === "D")) {
    if (showDictButton.value) {
      handleToggleDictionary();
      e.preventDefault();
    }
    return;
  }

  switch (e.keyCode) {
    case 9:
      handleChangeTranslationLanguage();
      e.preventDefault();
      break;
    case 13:
      handleEnter();
      break;
    case 27:
      if (showDictionary.value) {
        showDictionary.value = false;
        dictionaryData.value = null;
      } else if (isChangeTranslationLanguage.value) {
        handleCloseChangeTranslationLanguage();
      } else {
        emit("closeWindow");
      }
      break;
    case 38:
      handleKeyUp();
      e.preventDefault();
      break;
    case 40:
      handleKeyDown();
      e.preventDefault();
      break;
  }
}

async function fetchDictionary(word: string): Promise<DictionaryWord | null> {
  try {
    const { fetch: tauriFetch } = await import("@tauri-apps/plugin-http");
    const res = await tauriFetch(
      `https://api.dictionaryapi.dev/api/v2/entries/en/${encodeURIComponent(word)}`,
      {
        method: "GET",
      },
    );
    if (!res.ok) return null;
    const data = await res.json();
    return data[0] || null;
  } catch (e) {
    console.error("词典查询失败:", e);
    return null;
  }
}

function convertToDictionaryData(apiData: DictionaryWord): DictionaryData {
  const phonetic = apiData.phonetic || apiData.phonetics?.[0]?.text || "";

  const definitions =
    apiData.meanings?.flatMap((m) =>
      m.definitions.map((d) => ({
        pos: m.partOfSpeech,
        text: d.definition,
      })),
    ) || [];

  const examples =
    apiData.meanings?.flatMap((m) =>
      m.definitions.filter((d) => d.example).map((d) => ({ en: d.example!, cn: "" })),
    ) || [];

  const synonyms = [...new Set(apiData.meanings?.flatMap((m) => m.synonyms) || [])];
  const antonyms = [...new Set(apiData.meanings?.flatMap((m) => m.antonyms) || [])];

  return {
    word: apiData.word,
    translation: definitions[0]?.text || "",
    phonetic,
    tags: synonyms.slice(0, 3),
    definitions,
    forms: {},
    examples,
    synonyms,
    antonyms,
  };
}

onMounted(() => {
  if (props.keyword?.trim().length) {
    tranStr.value = props.keyword.trim();
    initFlag.value = true;
    baiduTranslate().then((res: any) => {
      resultList.value = res;
      current.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, searchWindowHeight.value));

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
  focus: () => inputRef.value?.focus(),
  handleKeydown,
  handleEnter,
  handleClose,
  handleKeyUp,
  handleKeyDown,
  handleChangeTranslationLanguage,
  handleCloseChangeTranslationLanguage,
  getDefaultHeight: () => chromeHeight.value + SEARCH_INPUT_HEIGHT,
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
</style>

<style>
.n-message-wrapper {
  --n-padding: 1px 10px !important;
}
</style>
