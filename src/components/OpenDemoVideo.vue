<template>
  <NButton
    quaternary
    size="tiny"
    class="text-blue-500 hover:text-blue-600 !mr-2"
    @click="handleOpenVideoDemo"
  >
    ▶ 功能演示
  </NButton>
</template>

<script setup lang="ts">
import { exeCommand } from '@/api';

const props = defineProps(['videoUrl']);

function handleOpenVideoDemo() {
  exeCommand(props.videoUrl);
}
</script>
