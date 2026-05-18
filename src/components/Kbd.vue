<template>
  <kbd
    class="inline-flex items-center justify-center px-2 py-1 text-xs font-medium text-gray-200 bg-[#2b2b2b] border border-[#3a3a3a] rounded-md shadow-[inset_0_-1px_0_#1a1a1a,inset_0_1px_0_#3a3a3a] leading-none select-none"
    :style="{ fontSize: size ? `${size}px` : '' }"
  >
    <slot>{{ keyName }}</slot>
  </kbd>
</template>

<script setup lang="ts">
defineProps<{ keyName?: string; size?: string }>();
</script>
