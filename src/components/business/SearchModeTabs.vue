<template>
  <div class="flex-sb-c">
    <button
      v-for="item in props.options"
      :title="$t(`searchSetting.${item.label}`)"
      :key="item.value"
      :class="['search-mode-tab', props.size, { active: activeModelValue?.includes(item.value) }]"
      type="button"
      @click="handleSwitchMode(item.value)"
    >
      <Icon :name="item.icon" />
      <span>{{ $t(`searchSetting.${item.label}`) }}</span>
    </button>
  </div>
</template>

<script setup lang="ts" generic="T extends number | number[]">
import { computed } from "vue";
import { MODE_TABS } from "@/constant";

interface Props {
  type?: "single" | "multiple";
  size?: "large" | "default" | "small" | "mini";
  options?: SearchModeItem[];
}

const props = withDefaults(defineProps<Props>(), {
  type: "single",
  size: "default",
  options: () => MODE_TABS,
});

const model = defineModel<T>({ required: true });

const emit = defineEmits(["change"]);

const searchModel = computed<T>({
  get: () => model.value,
  set: (val: T) => (model.value = val),
});

const activeModelValue = computed<number[]>(() =>
  Array.isArray(searchModel.value) ? [...searchModel.value] : [searchModel.value],
);
const activeCursorStyle = computed(() =>
  props.type === "multiple" && activeModelValue.value.length === 1 ? "not-allowed" : "pointer",
);

// const isMultiple = computed(() => props.type === "multiple");

function handleSwitchMode(newModel: SearchMode) {
  if (Array.isArray(searchModel.value)) {
    // 当只剩1个模式时 不能被取消选中
    const findIndex = searchModel.value.findIndex((model) => model === newModel);
    if (findIndex === -1) {
      searchModel.value.push(newModel);
      emitChange();
      return;
    }
    if (searchModel.value.length === 1) return;
    searchModel.value.splice(findIndex, 1);
  } else {
    // 重复点击 不做响应
    if (searchModel.value === newModel) return;
    searchModel.value = newModel as any;
  }
  emitChange();
}

function emitChange() {
  nextTick(() => {
    emit("change", searchModel.value);
  });
}
</script>

<style scoped lang="scss">
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
  word-wrap: normal;
  font-size: 15px;
  cursor: pointer;
}

.search-mode-tab.active {
  color: #155dfc;
  background: #dbeafe;
  cursor: v-bind("activeCursorStyle");
}

.search-mode-tab.disabled,
.search-mode-tab:disabled {
  opacity: 0.45;
  cursor: not-allowed;
  pointer-events: none;

  color: var(--text-color-disabled);
  background: var(--hover-color);

  filter: grayscale(0.3);
}

/* default */
.search-mode-tab.default {
  height: 32px !important;
  padding: 0 12px !important;
  font-size: 15px !important;
}

/* small */
.search-mode-tab.small {
  height: 28px !important;
  padding: 0 10px !important;
  font-size: 13px !important;
}

/* large */
.search-mode-tab.large {
  height: 38px !important;
  padding: 0 14px !important;
  font-size: 16px !important;
}

/* mini */
.search-mode-tab.mini {
  height: 24px;
  padding: 0 8px;
  font-size: 12px;
  gap: 4px;
}
</style>
