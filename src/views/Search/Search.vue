<template>
  <div class="search-shell">
    <div v-if="showModeTabs" class="search-mode-tabs">
      <button
        v-for="item in visibleModeTabs"
        :key="item.mode"
        class="search-mode-tab"
        :class="{ active: searchModel === item.mode }"
        type="button"
        @click="handleSwitchMode({ mode: item.mode })"
      >
        <n-icon :component="item.icon" size="20" />
        <span>{{ item.label }}</span>
      </button>

      <Kbd>Shift + Tab</Kbd>
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
import type { SearchModelType } from "./searchModes";
import {
  cursorPosition,
  getCurrentWindow,
  LogicalPosition,
  LogicalSize,
} from "@tauri-apps/api/window";
import { CheckboxOutline, GlobeOutline, LanguageOutline, SearchOutline } from "@vicons/ionicons5";
import { nextTick, ref } from "vue";
import { isForegroundFullscreen } from "@/api";
import { useAppConfig, useAppConfigActions } from "@/composables";
import { AppEvent, SEARCH_INPUT_HEIGHT, SEARCH_WINDOW_WIDTH } from "@/constant";
import { EventBus } from "@/utils/eventBus";
import DefaultSearchMode from "./components/DefaultSearchMode.vue";
import TodoMode from "./components/TodoMode.vue";
import TranslationMode from "./components/TranslationMode.vue";
import WebSearchMode from "./components/WebSearchMode.vue";
import { SEARCH_MODE_TABS_HEIGHT, SEARCH_MODEL } from "./searchModes";

interface SearchModeExpose {
  focus?: () => void;
  handleKeydown?: (event: KeyboardEvent) => void;
  handleClose?: () => void;
  getDefaultHeight?: () => number;
}

interface SwitchModePayload {
  mode: SearchModelType;
  keyword?: string;
  source?: WebSearchSource;
}

const { appConfigStore } = useAppConfig();
const searchWindow = getCurrentWindow();
const searchModel = ref<SearchModelType>(SEARCH_MODEL.DEFAULT_MODEL);
const pendingKeyword = ref("");
const activeWebSearchSource = ref<WebSearchSource>();
const activeModeRef = useTemplateRef<SearchModeExpose>("activeModeRef");
const visibleModeTabs = [
  { mode: SEARCH_MODEL.DEFAULT_MODEL, label: "搜索", icon: SearchOutline },
  { mode: SEARCH_MODEL.TODO_MODEL, label: "Todo", icon: CheckboxOutline },
  { mode: SEARCH_MODEL.TRANSLATION_MODEL, label: "翻译", icon: LanguageOutline },
  { mode: SEARCH_MODEL.WEB_SEARCH_MODEL, label: "WebSearch", icon: GlobeOutline },
];
const showModeTabs = computed(() => appConfigStore.showSearchModeTabs);

const activeModeComponent = computed(() => {
  if (searchModel.value === SEARCH_MODEL.WEB_SEARCH_MODEL) return WebSearchMode;
  if (searchModel.value === SEARCH_MODEL.TRANSLATION_MODEL) return TranslationMode;
  if (searchModel.value === SEARCH_MODEL.TODO_MODEL) return TodoMode;
  return DefaultSearchMode;
});

const activeModeProps = computed(() => {
  const chromeHeight = showModeTabs.value ? SEARCH_MODE_TABS_HEIGHT : 0;

  if (searchModel.value === SEARCH_MODEL.WEB_SEARCH_MODEL) {
    return {
      chromeHeight,
      keyword: pendingKeyword.value,
      source: activeWebSearchSource.value,
    };
  }

  return {
    chromeHeight,
    keyword: pendingKeyword.value,
  };
});

async function handleSwitchMode(payload: SwitchModePayload) {
  if (payload.mode === searchModel.value && !payload.keyword && !payload.source) return;

  activeModeRef.value?.handleClose?.();
  pendingKeyword.value = payload.keyword || "";
  activeWebSearchSource.value =
    payload.mode === SEARCH_MODEL.WEB_SEARCH_MODEL
      ? payload.source || activeWebSearchSource.value || appConfigStore.webSearchSourceList[0]
      : undefined;
  searchModel.value = payload.mode;

  await resizeToActiveModeDefaultHeight();
  activeModeRef.value?.focus?.();
}

async function resizeToActiveModeDefaultHeight() {
  await nextTick();
  const chromeHeight = showModeTabs.value ? SEARCH_MODE_TABS_HEIGHT : 0;
  const height = activeModeRef.value?.getDefaultHeight?.() ?? SEARCH_INPUT_HEIGHT + chromeHeight;
  await searchWindow.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, height));
}

async function handleClose(isEscClose: boolean = false) {
  searchModel.value = SEARCH_MODEL.DEFAULT_MODEL;
  pendingKeyword.value = "";
  activeWebSearchSource.value = undefined;
  activeModeRef.value?.handleClose?.();
  await resizeToActiveModeDefaultHeight();

  if (appConfigStore.searchHideAfterOpen || isEscClose) {
    searchWindow.hide();
  }
}

async function handleShow() {
  if (appConfigStore.searchOpenOnMouseDisplay) {
    const { x, y } = await cursorPosition();
    const { width } = await searchWindow.innerSize();
    await searchWindow.setPosition(new LogicalPosition(x - width / 2, y));
  } else {
    await searchWindow.setPosition(new LogicalPosition(1, 1));
  }

  await searchWindow.center();
  await searchWindow.show();
  await searchWindow.setFocus();

  nextTick(() => {
    activeModeRef.value?.focus?.();
  });
}

function handleBlur() {
  if (appConfigStore.searchLostFocusHide) handleClose();
}

function handleKeydown(event: KeyboardEvent) {
  // && showModeTabs.value
  if (event.shiftKey && event.key === "Tab") {
    event.preventDefault();
    const currentIndex = visibleModeTabs.findIndex((item) => item.mode === searchModel.value);
    const nextIndex = currentIndex === visibleModeTabs.length - 1 ? 0 : currentIndex + 1;
    void handleSwitchMode({ mode: visibleModeTabs[nextIndex].mode });
    return;
  }

  activeModeRef.value?.handleKeydown?.(event);
}

useAppConfigActions().registerSearchShortcutKey();

let unlistenFocus: any = null;
let unlistenShortcut: any = null;

onMounted(async () => {
  unlistenFocus = await searchWindow.onFocusChanged(({ payload }) => {
    if (!payload) handleBlur();
  });
  unlistenShortcut = await EventBus.listen(AppEvent.SEARCH_SHORTCU_KEY, async () => {
    if (!appConfigStore.enableSearch) return;

    const isFull = await isForegroundFullscreen();
    const windowVisible = await searchWindow.isVisible();
    if (appConfigStore.doNotDisturbMode && isFull && !windowVisible) return;
    if (windowVisible) handleClose();
    else handleShow();
  });
  window.addEventListener("keydown", handleKeydown);
});

onUnmounted(() => {
  window.removeEventListener("keydown", handleKeydown);
  if (unlistenFocus) unlistenFocus();
  if (unlistenShortcut) unlistenShortcut();
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
