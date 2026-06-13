<template>
  <div class="flex-sb-c">
    <template v-if="props.drag">
      <VueDraggable
        v-model="localOptions"
        target=".sort-target"
        ghost-class="opacity-50"
        class="flex-sb-c w-full"
        :animation="200"
        @end="handleDragEnd"
      >
        <TransitionGroup name="mode-tab" tag="div" class="sort-target flex-sb-c w-full">
          <button
            v-for="item in localOptions"
            :title="$t(`searchSetting.${item.label}`)"
            :key="item.value"
            :disabled="item.disabled"
            :class="[
              'search-mode-tab',
              props.size,
              { active: activeModelValue?.includes(item.value) },
              { disabled: item.disabled },
            ]"
            type="button"
            @click="!item.disabled && handleSwitchMode(item.value)"
          >
            <Icon :name="item.icon" />
            <span>{{ $t(`searchSetting.${item.label}`) }}</span>
          </button>
        </TransitionGroup>
      </VueDraggable>
    </template>
    <template v-else>
      <TransitionGroup name="mode-tab" tag="div" class="flex-sb-c w-full">
        <button
          v-for="item in props.options"
          :title="$t(`searchSetting.${item.label}`)"
          :key="item.value"
          :disabled="item.disabled"
          :class="[
            'search-mode-tab',
            props.size,
            { active: activeModelValue?.includes(item.value) },
            { disabled: item.disabled },
          ]"
          type="button"
          @click="!item.disabled && handleSwitchMode(item.value)"
        >
          <Icon :name="item.icon" />
          <span>{{ $t(`searchSetting.${item.label}`) }}</span>
        </button>
      </TransitionGroup>
    </template>
  </div>
</template>

<script setup lang="ts" generic="T extends number | number[]">
import { computed, ref, watch } from "vue";
import { VueDraggable } from "vue-draggable-plus";
import { MODE_TABS } from "@/constant";

interface Props {
  type?: "single" | "multiple";
  size?: "large" | "default" | "small" | "mini";
  options?: SearchModeItem[];
  drag?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  type: "single",
  size: "default",
  options: () => MODE_TABS,
  drag: false,
});

const emit = defineEmits(["change", "drag"]);

const model = defineModel<T>({ required: true });

const localOptions = ref<SearchModeItem[]>([...props.options]);

watch(
  () => props.options,
  (val) => {
    localOptions.value = [...val];
  },
  { deep: true },
);

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

function handleSwitchMode(newModel: SearchMode) {
  if (Array.isArray(searchModel.value)) {
    const findIndex = searchModel.value.findIndex((model) => model === newModel);
    if (findIndex === -1) {
      searchModel.value.push(newModel);
      emitChange();
      return;
    }
    if (searchModel.value.length === 1) return;
    searchModel.value.splice(findIndex, 1);
  } else {
    if (searchModel.value === newModel) return;
    searchModel.value = newModel as any;
  }
  emitChange();
}

function handleDragEnd() {
  emit("drag", [...localOptions.value]);
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

  color: var(--text-color-disabled);
  background: var(--hover-color);

  filter: grayscale(0.3);
}

.search-mode-tab.disabled:hover,
.search-mode-tab:disabled:hover {
  opacity: 0.55;
  background: color-mix(in srgb, var(--hover-color) 80%, var(--text-color-disabled));
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

/* 拖拽过渡 */
.mode-tab-move,
.mode-tab-enter-active,
.mode-tab-leave-active {
  transition: transform 0.25s ease;
}

.mode-tab-enter-from {
  opacity: 0;
  transform: translateX(-10px);
}

.mode-tab-leave-to {
  opacity: 0;
  transform: translateX(10px);
}

.mode-tab-leave-active {
  position: absolute;
}
</style>
