<!-- SettingItem.vue -->
<template>
  <div
    :class="rootClass"
    class="setting-item flex items-center justify-between gap-2 rounded bg-secondary/50 px-2.5 py-2 transition-colors hover:bg-secondary/80"
  >
    <!-- Left -->
    <div class="flex items-center gap-2">
      <div
        v-if="icon"
        class="flex h-6 w-6 items-center justify-center rounded bg-accent text-muted-foreground"
      >
        <Icon
          :name="icon"
          size="14"
        />
      </div>

      <div>
        <p class="text-xs font-medium leading-tight text-foreground">
          {{ title }}
        </p>
        <p
          v-if="description"
          class="text-[10px] leading-tight text-muted-foreground whitespace-nowrap"
        >
          {{ description }}
        </p>
      </div>
    </div>

    <!-- Right slot -->
    <div>
      <slot></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

interface Props {
  icon?: any;
  title: string;
  description?: string;
  className?: string;
}

const props = defineProps<Props>();

const rootClass = computed(() => {
  return ['', '', props.className].filter(Boolean).join(' ');
});
</script>

<style lang="scss" scoped>
.setting-item {
  // 额外可扩展 hover / 动效
  // &:hover {
  //   transform: translateY(-1px);
  // }
}
</style>
