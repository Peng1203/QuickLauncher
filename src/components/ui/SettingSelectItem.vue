<!-- SettingSwitchItem.vue -->
<template>
  <SettingItem
    :icon="icon"
    :title="title"
    :description="description"
    :class-name="className"
  >
    <slot>
      <n-select
        v-model:value="modelValue"
        size="small"
        filterable
        :consistent-menu-width="false"
        :placeholder="placeholder"
        :options="options"
        style="width: 160px"
        v-bind="$attrs"
      />
    </slot>
  </SettingItem>
</template>

<script setup lang="ts" generic="T extends string | number | null">
import SettingItem from './SettingItem.vue';

interface Props {
  title: string;
  options: any[];
  icon?: any;
  description?: string;
  className?: string;
  placeholder?: string;
}

defineProps<Props>();

defineEmits(['update:modelValue']);

const modelValue = defineModel<T>();
</script>
