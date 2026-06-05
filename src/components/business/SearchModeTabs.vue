<template>
  <div class="flex-sb-c">
    <button
      v-for="item in props.options"
      :key="item.value"
      :class="{ active: activeModelValue?.includes(item.value) }"
      class="search-mode-tab"
      type="button"
      @click="handleSwitchMode(item.value)"
    >
      <Icon :name="item.icon" />
      <span>{{ item.label }}</span>
    </button>
  </div>
</template>

<script setup lang="ts" generic="T extends number | number[]">
import { computed } from "vue";
import { MODE_TABS } from "@/constant";

interface Props {
  type?: "single" | "multiple";
  options?: SearchModeItem[];
}

const props = withDefaults(defineProps<Props>(), {
  type: "single",
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
</style>
