<template>
  <n-button
    quaternary
    size="tiny"
    class="text-primary hover:text-primary/80 !mr-2"
    @click="handleOpenVideoDemo"
  >
    ▶ {{ t('demo.label') }}
  </n-button>
</template>

<script setup lang="ts">
import { exeCommand } from '@/api';
import { t } from '@/i18n';

const props = defineProps(['videoUrl']);

function handleOpenVideoDemo() {
  exeCommand(props.videoUrl);
}
</script>
