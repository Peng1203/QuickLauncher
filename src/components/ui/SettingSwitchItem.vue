<!-- SettingSwitchItem.vue -->
<template>
  <SettingItem
    :icon="icon"
    :title="title"
    :description="description"
    :class-name="className"
  >
    <slot></slot>
    <n-switch v-model:value="modelValue" />
  </SettingItem>
</template>

<script setup lang="ts">
import SettingItem from './SettingItem.vue';

interface Props {
  icon?: any;
  title: string;
  description?: string;
  className?: string;
}

defineProps<Props>();

defineEmits(['update:modelValue']);

const modelValue = defineModel<boolean>();
</script>
