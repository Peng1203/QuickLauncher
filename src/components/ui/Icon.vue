<!-- <template>
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
</script> -->

<template>
  <div
    v-if="background"
    class="inline-flex items-center justify-center rounded-md"
    :class="link ? 'cursor-pointer' : ''"
    :style="backgroundStyle"
  >
    <n-icon
      :size="size"
      :class="`iconfont ${name}`"
      :color="iconColor"
    />
  </div>

  <n-icon
    v-else
    :size="size"
    :class="`iconfont ${name} ${link ? 'cursor-pointer' : ''}`"
    :color="link ? '#409EFF' : color"
  />
</template>

<script setup lang="ts">
import { computed } from 'vue';

interface Props {
  name?: string;
  size?: number | string;
  color?: string;
  background?: boolean;
  link?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  name: '',
  size: 18,
  color: '',
  background: false,
  link: false,
});

/**
 * 转 rgba 背景色
 */
function hexToRgba(hex: string, alpha = 0.12) {
  if (!hex.startsWith('#')) return hex;

  let value = hex.replace('#', '');

  if (value.length === 3) {
    value = value
      .split('')
      .map(i => i + i)
      .join('');
  }

  const num = Number.parseInt(value, 16);

  const r = (num >> 16) & 255;
  const g = (num >> 8) & 255;
  const b = num & 255;

  return `rgba(${r}, ${g}, ${b}, ${alpha})`;
}

const iconColor = computed(() => (props.link ? '#409EFF' : props.color));

const backgroundStyle = computed(() => {
  const size = `${props.size}px`;

  return {
    width: `calc(${size} + 14px)`,
    height: `calc(${size} + 14px)`,
    backgroundColor: props.color ? hexToRgba(props.color, 0.15) : '',
  };
});
</script>
