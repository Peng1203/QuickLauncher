<template>
  <n-icon
    :size="size"
    :class="`iconfont ${name}`"
    :color="color"
  />
</template>

<script setup lang="ts">
interface Props {
  name?: string;
  size?: number | string;
  color?: string;
}

withDefaults(defineProps<Props>(), {
  name: '',
  size: 18,
  color: '',
});
</script>
