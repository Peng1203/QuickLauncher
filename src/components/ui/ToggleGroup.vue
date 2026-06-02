<template>
  <div
    class="inline-flex items-center gap-1 p-1 rounded-lg border bg-gray-100 dark:bg-[#1d1d20]"
    :class="containerClass"
  >
    <button
      v-for="item in options"
      :key="item.value"
      class="rounded-md transition-all duration-200 cursor-pointer"
      :class="[
        sizeClass,
        model === item.value
          ? 'bg-blue-500 text-white shadow'
          : 'text-gray-600 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700',
      ]"
      @click="handleChange(item.value)"
    >
      {{ item.label }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";

type Size = "sm" | "md" | "lg";

const props = withDefaults(
  defineProps<{
    options: { label: string; value: string }[];
    size?: Size;
  }>(),
  {
    size: "md",
  },
);

const emits = defineEmits(["change"]);

const model = defineModel<string>();

const sizeClass = computed(() => {
  switch (props.size) {
    case "sm":
      return "px-2 py-1 text-xs";
    case "lg":
      return "px-5 py-2 text-base";
    case "md":
    default:
      return "px-4 py-1.5 text-sm";
  }
});

const containerClass = computed(() => {
  switch (props.size) {
    case "sm":
      return "gap-0.5 p-0.5";
    case "lg":
      return "gap-2 p-1.5";
    case "md":
    default:
      return "gap-1 p-1";
  }
});

function handleChange(value: string) {
  model.value = value;
  emits("change", value);
}
</script>
