<template>
  <div :class="classes">
    <slot></slot>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

interface Props {
  /**
   * 是否使用更小字号（可选扩展）
   */
  small?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  small: false,
});

const classes = computed(() => {
  return [props.small ? 'text-[11px]' : 'text-[12px]', 'leading-6', 'text-[#6b7280]'];
});
</script>
