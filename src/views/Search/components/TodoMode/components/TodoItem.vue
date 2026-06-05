<template>
  <li
    class="todo-item group relative flex items-start gap-3 px-4.5 py-3.5 box-border cursor-pointer border-l-4 transition-all duration-200 ease-out"
    :class="[completed ? 'text-gray-400' : '', isActive ? 'bg-muted' : 'bg-transparent']"
    :style="{
      minHeight: minHeight + 'px',
      borderLeftColor: priorityColor,
    }"
    @mouseenter="hover = true"
    @mouseleave="hover = false"
    @click="$emit('open', todo)"
  >
    <!-- checkbox -->
    <button
      class="inline-flex items-center justify-center p-0 bg-transparent border-0 cursor-pointer"
      :class="completed ? 'text-green-600' : 'text-gray-400'"
      @click.stop="$emit('toggle', todo.id)"
    >
      <n-icon :component="completed ? CheckmarkCircleOutline : RadioButtonOffOutline" size="22" />
    </button>

    <!-- content -->
    <div class="min-w-0 flex-1">
      <!-- title -->
      <div
        class="text-[15px] font-medium truncate transition-all duration-200"
        :class="completed ? 'line-through' : ''"
      >
        {{ title }}
      </div>

      <!-- metextow -->
      <div class="flex items-center justify-between mt-2 text-[12px] text-gray-500">
        <!-- left: created + tags -->
        <div class="flex items-center gap-2 min-w-0">
          <div class="flex items-center gap-1 text-gray-400 whitespace-nowrap">
            <n-icon :component="TimeOutline" size="14" v-if="todo.due_date || completed" />
            <span
              v-if="todo.due_date !== null"
              class="text-xs"
              :class="
                todo.due_date < 0
                  ? 'text-red-500'
                  : todo.due_date === 0
                    ? 'text-amber-500'
                    : 'text-gray-500'
              "
            >
              {{ formatDueDate(todo) }}
            </span>
          </div>

          <n-tag v-for="tag in tagsList" :key="tag" size="small" type="primary" :bordered="false">
            {{ tag }}
          </n-tag>
        </div>
      </div>
    </div>

    <!-- actions -->
    <div
      class="absolute right-3 top-1/2 -translate-y-1/2 flex items-center gap-2 text-xs text-gray-400 transition-all duration-200"
    >
      <span class="text-gray-400 whitespace-nowrap" :class="isActive ? 'opacity-0' : ''">
        {{ t("todo.createdByTime") }} {{ getFromNow(todo.created_at) }}
      </span>

      <div
        class="absolute flex-s-c gap-2 opacity-0 right-0"
        :class="isActive ? 'opacity-100' : 'group-hover:opacity-100'"
      >
        <button
          class="hover:text-red-500 transition-colors cursor-pointer"
          @click.stop="$emit('delete', todo.id)"
        >
          <n-icon :component="TrashOutline" size="16" />
        </button>

        <span class="opacity-60"><Kbd>→</Kbd> {{ t("todo.detail") }}</span>
      </div>
    </div>
  </li>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { getPriorityColor, formatDueDate } from "../index";
import { t } from "@/i18n";
import {
  CheckmarkCircleOutline,
  RadioButtonOffOutline,
  TimeOutline,
  TrashOutline,
} from "@vicons/ionicons5";
import { getFromNow } from "@/utils/date";

const props = defineProps<{
  todo: TodoItem;
  selected: boolean;
  minHeight: number;
}>();

defineEmits(["open", "toggle", "delete"]);

const hover = ref(false);

/**
 * 统一 active 状态（hover + keyboard select）
 */
const isActive = computed(() => props.selected || hover.value);

/**
 * fields
 */
const completed = computed(() => props.todo.completed);
const title = computed(() => props.todo.title);

/**
 * tags
 */
const tagsList = computed(() =>
  props.todo.tags ? props.todo.tags.split(",").filter(Boolean) : [],
);

/**
 * priority
 */
const priorityColor = computed(() => getPriorityColor(props.todo.priority));
</script>

<style scoped>
/* .todo-item {
  transition:
    background-color 0.2s ease,
    transform 0.15s ease;
} */

/* 可选：更高级一点的 hover 感 */
/* .todo-item:hover {
  transform: translateY(-1px);
} */

.todo-item {
  position: relative;
  isolation: isolate;
}

.todo-item::before {
  content: "";
  position: absolute;
  inset: 0;
  background: transparent;
  transition: background-color 0.2s ease;
  z-index: -1;
}

.todo-item:hover::before {
  background-color: rgba(0, 0, 0, 0.04);
}
</style>
