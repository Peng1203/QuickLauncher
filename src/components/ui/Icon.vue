<template>
  <n-icon
    :size="size"
    :class="`iconfont ${name}`"
  />
</template>

<script setup lang="ts">
interface Props {
  name?: string;
  size?: number | string;
}

withDefaults(defineProps<Props>(), {
  name: '',
  size: 18,
});
</script>
