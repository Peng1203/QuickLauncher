<template>
  <li
    class="todo-item h-20 group relative flex items-start gap-3 px-4.5 py-3.5 box-border cursor-pointer border-l-4 transition-all duration-200 ease-out"
    :class="[
      completed ? 'text-gray-400' : '',
      isActive ? 'bg-muted' : 'bg-transparent',
      isOverdue && !completed ? 'overdue-item' : '',
    ]"
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
        class="text-[15px] max-w-100 font-medium truncate transition-all duration-200"
        :class="[completed ? 'line-through' : '', isOverdue && !completed ? 'text-red-600' : '']"
      >
        {{ title }}
      </div>

      <!-- metadata -->
      <div class="flex items-center justify-between mt-2 text-[12px] text-gray-500">
        <div class="flex items-center gap-2 min-w-0">
          <!-- due_date -->
          <div
            v-if="todo.due_date"
            class="flex items-center gap-1 whitespace-nowrap"
            :class="isOverdue && !completed ? 'text-red-500' : 'text-gray-400'"
          >
            <Icon name="icon-shijian" size="14" />
            <span class="text-xs">{{ formatDueDate(todo) }}</span>
          </div>

          <!-- reminder_at -->
          <div
            v-if="todo.reminder_at && !completed"
            class="flex items-center gap-1 whitespace-nowrap"
            :class="isReminded ? 'text-gray-400' : 'text-blue-500'"
          >
            <Icon name="icon-tixingshijian" size="14" />
            <span class="text-xs">{{ formatRelativeReminder(todo.reminder_at, t) }}</span>
          </div>

          <!-- completed badge -->
          <div v-if="completed" class="flex items-center gap-1 text-green-500 whitespace-nowrap">
            <Icon name="icon-fangkuangxuanzhong" size="14" />
            <span class="text-xs">{{ t("todo.completed") }}</span>
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
          <Icon name="icon-shanchu" />
        </button>

        <span class="flex-s-c gap-1 opacity-60 whitespace-nowrap"
          ><Kbd>→</Kbd> {{ t("todo.detail") }}</span
        >
      </div>
    </div>
  </li>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { getPriorityColor, formatDueDate } from "../index";
import { t } from "@/i18n";
import { CheckmarkCircleOutline, RadioButtonOffOutline } from "@vicons/ionicons5";
import { getFromNow, getDaysUntil, formatRelativeReminder } from "@/utils/date";

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

/**
 * overdue
 */
const isOverdue = computed(() => {
  if (!props.todo.due_date) return false;
  return getDaysUntil(props.todo.due_date)! < 0;
});

const isReminded = computed(() => {
  if (!props.todo.reminder_at) return false;
  return Date.now() >= props.todo.reminder_at;
});
</script>

<style scoped>
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

/* 过期样式 */
.overdue-item {
  background-color: rgba(239, 68, 68, 0.04) !important;
}

.overdue-item::before {
  background-color: rgba(239, 68, 68, 0.04) !important;
}

.overdue-item:hover::before {
  background-color: rgba(239, 68, 68, 0.08) !important;
}
</style>
