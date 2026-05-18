<template>
  <div :class="$attrs.class">
    <!-- Header -->
    <div>
      <h3 class="text-[14px] font-medium uppercase tracking-wider text-muted-foreground">
        {{ title }}
      </h3>

      <p
        v-if="description"
        class="text-[10px] text-muted-foreground/70 whitespace-nowrap"
      >
        {{ description }}
      </p>
    </div>

    <!-- Content -->
    <div class="flex flex-col gap-1">
      <slot></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  title: string;
  description?: string;
}

defineProps<Props>();
</script>
