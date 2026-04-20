<template>
  <!-- class="inline-flex items-center justify-center select-none rounded border border-gray-200 bg-white px-1.5 py-0.5 text-[12px] font-medium text-gray-700 shadow-[0_1px_0_0_rgba(0,0,0,0.05)] tracking-wide" -->

  <!-- class="inline-flex items-center justify-center px-2 py-1 text-xs font-medium text-gray-800 bg-[#f5f5f5] border border-[#e0e0e0] rounded-md shadow-[inset_0_-1px_0_#ffffff,inset_0_1px_0_#d4d4d4] leading-none select-none" -->

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
