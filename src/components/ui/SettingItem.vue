<!-- SettingItem.vue -->
<template>
  <div>
    <div
      :class="rootClass"
      class="setting-item gap-2 rounded bg-secondary/50 px-2.5 py-2 transition-colors hover:bg-secondary/80"
    >
      <div class="flex items-center justify-between">
        <!-- Left -->
        <div class="flex items-center gap-2 min-w-0">
          <div
            v-if="icon"
            class="flex h-6 w-6 flex-shrink-0 items-center justify-center rounded bg-accent text-muted-foreground"
          >
            <Icon :name="icon" :color="iconColor" size="14" />
          </div>

          <div class="min-w-0">
            <slot name="title" :handle-toggle="handleToggle">
              <p
                class="text-xs font-medium leading-tight text-foreground truncate"
                :class="props.expandable ? 'cursor-pointer text-[var(--primary)]!' : ''"
                @click="handleToggle"
                v-html="title"
              ></p>
              <p
                v-if="description"
                class="text-[10px] leading-tight text-muted-foreground line-clamp-2"
              >
                {{ description }}
              </p>
            </slot>
          </div>
        </div>

        <!-- Right slot -->
        <div class="flex-shrink-0">
          <slot></slot>
        </div>
      </div>

      <!-- body slot -->
      <slot name="body"> </slot>
    </div>

    <!-- Expand content -->
    <Transition name="expand">
      <div
        v-if="expandable && expanded"
        class="overflow-hidden rounded-b-md border-t border-white/10 bg-secondary/60 dark:bg-white/5 px-2.5 py-2.5 shadow-[inset_0_1px_0_rgba(255,255,255,0.06)] dark:shadow-[inset_0_1px_0_rgba(255,255,255,0.03)]"
      >
        <slot name="expand"></slot>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";

interface Props {
  icon?: any;
  iconColor?: string;
  title?: string;
  description?: string;
  className?: string;
  expandable?: boolean;
}

const props = defineProps<Props>();

const expanded = defineModel<boolean>("expanded", { default: false });

const rootClass = computed(() => {
  return ["", "", props.className].filter(Boolean).join(" ");
});

function handleToggle() {
  if (!props.expandable) return;

  expanded.value = !expanded.value;
}
</script>
