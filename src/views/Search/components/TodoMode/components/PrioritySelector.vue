<template>
  <div class="flex-s-c gap-2">
    <button
      v-for="item in priorityOptions"
      :key="item.value"
      class="h-8 min-w-11 rounded cursor-pointer border-0 bg-gray-50 text-gray-700 dark:bg-zinc-800 dark:text-gray-300"
      :style="getPriorityStyle(item.value, modelValue === item.value)"
      @click="modelValue = item.value"
    >
      {{ item.label }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { getPriorityColor, priorityOptions } from "../index";

const modelValue = defineModel<TodoPriority>({
  required: true,
});

function getPriorityStyle(priority: TodoPriority, active: boolean) {
  if (!active) return {};

  const color = getPriorityColor(priority);

  return {
    color,
    backgroundColor: `${color}15`, // 约 8% 透明度
    outline: `1px solid ${color}40`,
  };
}
</script>
