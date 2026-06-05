<!-- SettingSwitchItem.vue -->
<template>
  <SettingItem :icon="icon" :title="title" :description="description" :class-name="className">
    <slot></slot>
    <template #expand>
      <slot name="expand"></slot>
    </template>
    <template #body>
      <slot name="body"></slot>
    </template>

    <n-switch v-model:value="modelValue" size="small" />
  </SettingItem>
</template>

<script setup lang="ts">
import SettingItem from "./SettingItem.vue";

interface Props {
  icon?: any;
  title: string;
  description?: string;
  className?: string;
}

defineProps<Props>();

defineEmits(["update:modelValue"]);

const modelValue = defineModel<boolean>();
</script>
