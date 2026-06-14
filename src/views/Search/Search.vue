<template>
  <div class="search-shell">
    <div v-if="showModeTabs" class="flex-sb-c search-mode-tabs">
      <SearchModeTabs
        v-model="searchModel"
        :options="visibleModeTabs"
        @change="(mode) => handleSwitchMode({ mode })"
      />
      <!-- {{ searchModel }} -->
      <Kbd v-if="appConfigStore.switchModeShortcutKey">
        {{ appConfigStore.switchModeShortcutKey }}
      </Kbd>
    </div>

    <component
      :is="activeModeComponent"
      ref="activeModeRef"
      v-bind="activeModeProps"
      @close-window="handleClose"
      @switch-mode="handleSwitchMode"
    />
  </div>
</template>

<script setup lang="ts">
import type { SearchModelType, SearchModeExpose, SwitchModePayload, FormType } from "./searchModes";
import {
  cursorPosition,
  getCurrentWindow,
  LogicalPosition,
  LogicalSize,
} from "@tauri-apps/api/window";
import { nextTick, ref } from "vue";
import { isForegroundFullscreen } from "@/api";
import { useAppConfig, useAppConfigActions } from "@/composables";
import { AppEvent, SEARCH_INPUT_HEIGHT, SEARCH_WINDOW_WIDTH, SEARCH_MODEL } from "@/constant";
import { EventBus } from "@/utils/eventBus";
import DefaultSearchMode from "./components/DefaultMode/DefaultSearchMode.vue";
import FileSearchMode from "./components/FileSearchMode/FileSearchMode.vue";
import TodoMode from "./components/TodoMode/TodoMode.vue";
import TranslationMode from "./components/TranslationMode.vue";
import WebSearchMode from "./components/WebSearchMode.vue";
import { SEARCH_MODE_TABS_HEIGHT } from "./searchModes";
import { ALT, CTRL, SHIFT, WIN } from "@/utils/shortcutKey";
import { useEventListener } from "@vueuse/core";

const { appConfigStore } = useAppConfig();
const searchWindow = getCurrentWindow();

// 当前选中的搜索模式，默认取配置中的默认模式
const searchModel = ref<SearchModelType>(appConfigStore.defaultMode);
// 待传入子组件的关键词
const pendingKeyword = ref("");
// 当前激活的网络搜索来源
const activeWebSearchSource = ref<WebSearchSource>();
// 当前激活模式组件的 ref
const activeModeRef = useTemplateRef<SearchModeExpose>("activeModeRef");

// 根据配置过滤可见的模式 tab 列表
const visibleModeTabs = computed(() =>
  appConfigStore.modeOptions
    .filter((item) => appConfigStore.showModes.includes(item.value))
    .map((item) => ({ ...item, disabled: getModelDisabled(item.value) })),
);

const getModelDisabled = (val: (typeof SEARCH_MODEL)[keyof typeof SEARCH_MODEL]) => {
  switch (val) {
    case SEARCH_MODEL.DEFAULT_MODEL:
      return !appConfigStore.enableDefaultSearch;
    case SEARCH_MODEL.WEB_SEARCH_MODEL:
      return !appConfigStore.enableWebSearch;
    case SEARCH_MODEL.TRANSLATION_MODEL:
      return !appConfigStore.enableTranslation;
    case SEARCH_MODEL.FILE_MODEL:
      return !appConfigStore.enableFileSearch;
    // case SEARCH_MODEL.TODO_MODEL:
    //   return appConfigStore.enableTodoMode;
    default:
      return false;
  }
};

// 是否显示模式切换 tab 栏
const showModeTabs = computed(() => appConfigStore.showSearchModeTabs);

const chromeHeight = computed(() => (showModeTabs.value ? SEARCH_MODE_TABS_HEIGHT : 0));

// 根据当前模式动态返回对应的组件
const activeModeComponent = computed(() => {
  const map: Record<string, any> = {
    [SEARCH_MODEL.WEB_SEARCH_MODEL]: WebSearchMode,
    [SEARCH_MODEL.TRANSLATION_MODEL]: TranslationMode,
    [SEARCH_MODEL.TODO_MODEL]: TodoMode,
    [SEARCH_MODEL.FILE_MODEL]: FileSearchMode,
  };
  return map[searchModel.value] ?? DefaultSearchMode;
});

// 组装传给当前模式组件的 props
const activeModeProps = computed(() => {
  const base = { chromeHeight: chromeHeight.value, keyword: pendingKeyword.value };
  // WebSearch 模式额外传入 source
  if (searchModel.value === SEARCH_MODEL.WEB_SEARCH_MODEL) {
    return { ...base, source: activeWebSearchSource.value };
  }
  return base;
});

// 记录模式切换来源，用于 ESC 键特殊回退行为
const switchModeFrom = ref<FormType>("");

function setWebSearchActive(source?: WebSearchSource) {
  // 确定网络搜索默认来源（配置 > 已选 > 列表第一个）
  const defaultWebSearch =
    appConfigStore.webSearchSourceList.find(
      (item) => item.id === appConfigStore.webSearchDefaultSourceId,
    ) ?? null;

  activeWebSearchSource.value =
    source ||
    activeWebSearchSource.value ||
    defaultWebSearch ||
    appConfigStore.webSearchSourceList[0];
}

/**
 * 切换搜索模式
 * 调用入口：① tab 栏点击 ② Shift+Tab 快捷键 ③ 默认模式触发的特殊切换
 */
async function handleSwitchMode(payload: SwitchModePayload) {
  const { mode, keyword, source, from = "" } = payload;
  switchModeFrom.value = from;
  activeModeRef.value?.handleClose?.();
  pendingKeyword.value = keyword || "";
  setWebSearchActive(source);

  searchModel.value = mode;
  await resizeToActiveModeDefaultHeight();
  activeModeRef.value?.focus?.();
}

/** 将窗口高度调整为当前模式的默认高度 */
async function resizeToActiveModeDefaultHeight(shortcutKeyOpen = false) {
  await nextTick();
  // shortcutKeyOpen 时强制用输入框最小高度（防止展开状态残留）
  const height = shortcutKeyOpen
    ? SEARCH_INPUT_HEIGHT + chromeHeight.value
    : (activeModeRef.value?.getDefaultHeight?.() ?? SEARCH_INPUT_HEIGHT + chromeHeight.value);
  await searchWindow.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, height));
}

/** 关闭/隐藏搜索窗口，并重置状态 */
async function handleClose() {
  pendingKeyword.value = "";
  activeWebSearchSource.value = undefined;
  activeModeRef.value?.handleClose?.();
  await resizeToActiveModeDefaultHeight(true);
  searchWindow.hide();
}

// 窗口第一次打开的时候 将其设置为外层容器固定的宽高
/** 显示搜索窗口，并定位、聚焦 */
async function handleShow() {
  searchModel.value = appConfigStore.defaultMode;

  if (appConfigStore.searchOpenOnMouseDisplay) {
    const { width } = await searchWindow.innerSize();
    // 跟随鼠标位置居中显示
    const { x, y } = await cursorPosition();
    await searchWindow.setPosition(new LogicalPosition(x - width / 2, y));
  } else {
    await searchWindow.setPosition(new LogicalPosition(1, 1));
  }
  setWebSearchActive();
  // 打开窗口高度固定为 输入框高度 + tab 高度
  await searchWindow.setSize(
    new LogicalSize(SEARCH_WINDOW_WIDTH, SEARCH_INPUT_HEIGHT + chromeHeight.value),
  );

  await searchWindow.center();
  await searchWindow.show();
  await searchWindow.setFocus();
  nextTick(() => {
    activeModeRef.value?.focus?.();
    activeModeRef.value?.handleBeforeShow?.();
  });

  await resizeToActiveModeDefaultHeight();
}

/** 失焦时根据配置决定是否隐藏窗口 */
function handleBlur() {
  if (appConfigStore.searchLostFocusHide) handleClose();
}

function handleChangeSwitchModeByShortkey(event: KeyboardEvent): boolean {
  const SMSK = appConfigStore.switchModeShortcutKey;
  if (!SMSK) return false;
  const isCompKey = SMSK.includes("+");

  const SMSKArr = SMSK.split("+").map((s) => s.trim());
  const primaryKey = SMSKArr[SMSKArr.length - 1].trim();

  let eventMappingModifierKeys = [];
  if (isCompKey) {
    const hasCtrl = SMSKArr.includes(CTRL);
    if (hasCtrl) eventMappingModifierKeys.push("ctrlKey");

    const hasALT = SMSKArr.includes(ALT);
    if (hasALT) eventMappingModifierKeys.push("altKey");

    const hasSHIFT = SMSKArr.includes(SHIFT);
    if (hasSHIFT) eventMappingModifierKeys.push("shiftKey");

    const hasWIN = SMSKArr.includes(WIN);
    if (hasWIN) eventMappingModifierKeys.push("metaKey");

    if (!eventMappingModifierKeys.length) return false;
  }
  let flag = false;

  for (let i = 0; i < eventMappingModifierKeys.length; i++) {
    const key = eventMappingModifierKeys[i];
    // @ts-ignore
    if (!event[key]) return false;
    // @ts-ignore
    flag = event[key];
  }
  flag = primaryKey.toLowerCase() === event.key.toLowerCase();

  if (flag) {
    event.preventDefault();
    const tabs = visibleModeTabs.value;
    const len = tabs.length;
    const currentIndex = tabs.findIndex((item) => item.value === searchModel.value);

    let nextIndex = (currentIndex + 1) % len;
    let count = 0;
    while (tabs[nextIndex].disabled && count < len) {
      nextIndex = (nextIndex + 1) % len;
      count++;
    }

    if (count < len && tabs[nextIndex].value !== searchModel.value) {
      void handleSwitchMode({ mode: tabs[nextIndex].value });
    }
    return true;
  }
  return false;
}

/** 全局键盘事件处理 */
function handleKeydown(event: KeyboardEvent) {
  const isChanged = handleChangeSwitchModeByShortkey(event);
  if (isChanged) return event.preventDefault();

  // ESC：若当前是从默认模式跳转来的，则回退到默认模式
  if (event.keyCode === 27 && switchModeFrom.value === "search") {
    event.preventDefault();
    const defaultIndex = visibleModeTabs.value.findIndex(
      (item) => item.value === SEARCH_MODEL.DEFAULT_MODEL,
    );
    void handleSwitchMode({ mode: visibleModeTabs.value[defaultIndex].value });
    return;
  }

  // 其余按键透传给当前模式组件处理
  activeModeRef.value?.handleKeydown?.(event);
}

// 注册全局搜索快捷键
useAppConfigActions().registerSearchShortcutKey();

let unlistenFocus: any = null;
let unlistenShortcut: any = null;

useEventListener("keydown", handleKeydown);

onMounted(async () => {
  // 监听窗口焦点变化
  unlistenFocus = await searchWindow.onFocusChanged(({ payload }) => {
    if (!payload) handleBlur();
  });

  // 监听全局搜索快捷键事件：可见时关闭，不可见时显示（勿扰模式+全屏时跳过）
  unlistenShortcut = await EventBus.listen(AppEvent.SEARCH_SHORTCU_KEY, async () => {
    if (!appConfigStore.enableSearch) return;
    const isFull = await isForegroundFullscreen();
    const windowVisible = await searchWindow.isVisible();
    if (appConfigStore.doNotDisturbMode && isFull && !windowVisible) return;
    if (windowVisible) handleClose();
    else handleShow();
  });
});

onUnmounted(() => {
  unlistenFocus?.();
  unlistenShortcut?.();
});
</script>

<style>
.n-config-provider {
  height: 100%;
}

.n-input {
  transition: none !important;
  --n-caret-color: inherit !important;
  --n-border-hover: inherit !important;
  --n-border-focus: inherit !important;
  --n-box-shadow-focus: none !important;
}

.n-input * {
  transition: none !important;
  --n-caret-color: inherit !important;
  --n-border-hover: inherit !important;
  --n-border-focus: inherit !important;
  --n-box-shadow-focus: none !important;
}

.n-input {
  --n-color: var(--search-bg) !important;
  --n-color-focus: var(--search-bg) !important;
}

.search-shell {
  width: 100%;
  height: 100%;
  background: var(--search-bg);
}

.search-mode-tabs {
  display: flex;
  align-items: center;
  gap: 10px;
  height: 44px;
  padding: 6px 14px;
  border-bottom: 1px solid var(--border);
  box-sizing: border-box;
}

.search-mode-tab {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  height: 32px;
  padding: 0 12px;
  border: 0;
  border-radius: 8px;
  color: var(--foreground);
  background: transparent;
  font-size: 15px;
  cursor: pointer;
}

.search-mode-tab.active {
  color: #155dfc;
  background: #dbeafe;
}

.mode-switch-kbd {
  margin-left: auto;
  color: #8a94a6;
  background: #f5f6f8;
}
</style>
