<template>
  <li class="flex items-center px-2 h-[48px]">
    <img
      :src="icon"
      alt="icon"
      class="mt-2 object-contain pointer-events-none"
    />

    <span>{{ name }}</span>
  </li>
</template>

<script setup lang="ts">
defineProps<{
  id: number
  icon: string
  name: string
}>()
</script>
