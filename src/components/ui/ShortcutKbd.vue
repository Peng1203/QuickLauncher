<template>
  <div class="flex items-center">
    <template v-for="(key, index) in keys" :key="key">
      <Kbd :size="`${size}`">{{ formatKey(key) }}</Kbd>

      <span v-if="index !== keys.length - 1" class="mx-1 text-gray-400"> + </span>
    </template>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  value: string | string[];
  size?: string | number;
}>();

/**
 * 统一转数组
 */
const keys = computed(() => {
  if (Array.isArray(props.value)) return props.value;

  return props.value
    .split("+")
    .map((v) => v.trim())
    .filter(Boolean);
});

/**
 * 可扩展：做映射（后面支持 macOS / icon）
 */
function formatKey(key: string) {
  const map: Record<string, string> = {
    Ctrl: "Ctrl",
    Control: "Ctrl",
    Alt: "Alt",
    Shift: "Shift",
    Meta: "⌘",
    Command: "⌘",
    Enter: "↵",
    Escape: "Esc",
  };

  return map[key] ?? key;
}
</script>
