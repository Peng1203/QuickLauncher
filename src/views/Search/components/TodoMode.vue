<template>
  <section class="todo-mode">
    <label class="input-container max-h-11.25">
      <n-input
        ref="todoInputRef"
        v-model:value="inputValue"
        :disabled="isEditing"
        tabindex="-1"
        type="text"
        size="medium"
        class="w-full h-full max-h-11.25 resize-none text-sm hover:outline-0 focus-visible:outline-0 border-none bg-card shadow-none rounded-[10px]"
        :placeholder="isEditing ? '编辑中...' : '输入任务内容，回车创建...'"
        :class="viewState === 'create' ? 'border-b-0! rounded-b-none!' : ''"
        @input="syncCreateState"
      >
        <template #prefix>
          <n-icon :component="CheckboxOutline" size="22" class="shrink-0 text-blue-600" />
        </template>
        <template v-if="!isEditing" #suffix>
          <div class="flex items-center gap-1 text-xs text-gray-400">
            <Kbd>↑↓</Kbd>
            <span>切换</span>
            <Kbd>↵</Kbd>
            <span>完成</span>
          </div>
        </template>
      </n-input>
    </label>

    <div class="relative border-top-color! content">
      <transition name="todo-fade" mode="out-in">
        <div
          v-if="viewState === 'empty'"
          class="flex flex-col items-center justify-center text-center"
          :style="{ height: `${TODO_EMPTY_HEIGHT}px` }"
        >
          <div
            class="grid place-items-center mb-5 text-gray-400 bg-gray-100 rounded-full"
            :style="{ width: `${EMPTY_ICON_SIZE}px`, height: `${EMPTY_ICON_SIZE}px` }"
          >
            <n-icon :component="CheckboxOutline" size="42" />
          </div>
          <div class="text-xl font-semibold">还没有任何待办事项</div>
          <div class="mt-2.5 text-sm text-gray-500">在上方输入框中输入任务内容，按回车即可创建</div>
          <div
            class="inline-flex items-center gap-2 mt-7 px-3.5 py-2 text-gray-700 bg-gray-50 rounded-lg text-sm"
          >
            <n-icon :component="AddOutline" size="18" />
            快捷键：Enter 创建任务
          </div>
        </div>
      </transition>

      <transition name="todo-slide" mode="out-in">
        <div
          v-if="viewState === 'create'"
          class="overflow-hidden"
          :style="{ height: `${TODO_LIST_HEIGHT}px` }"
        >
          <div
            class="flex items-center justify-between px-3.5 border-b border-border box-border"
            :style="{ height: `${FILTER_HEADER_HEIGHT}px` }"
          >
            <div class="flex flex-nowrap gap-2.5 shrink">
              <button
                v-for="item in filterTabs"
                class="inline-flex items-center gap-2 h-8 px-2.5 border-0 rounded text-sm cursor-pointer whitespace-nowrap"
                :key="item.value"
                :class="
                  activeFilter === item.value
                    ? 'text-blue-600 bg-blue-100'
                    : 'text-gray-600 bg-transparent hover:bg-gray-100'
                "
                type="button"
                @click="activeFilter = item.value"
              >
                {{ item.label }}
                <span class="text-xs opacity-70">{{ item.count }}</span>
              </button>
            </div>

            <n-select
              v-model:value="sortType"
              :options="sortOptions"
              size="small"
              :style="{ width: `calc(${sortTypeLabel.length * 2}ch + 38px ) !important` }"
              class="shrink-0"
              :consistent-menu-width="false"
            />
          </div>

          <ul
            class="m-0 p-0 overflow-y-auto list-none"
            :style="{ height: `${TODO_LIST_UL_HEIGHT}px` }"
          >
            <!-- 虚拟预览项 -->
            <li
              class="flex items-start gap-3 px-[18px] py-3.5 box-border border-l-4 border-l-blue-400/40 bg-blue-50/20 opacity-50"
              :style="{ minHeight: `${TODO_ITEM_MIN_HEIGHT}px` }"
            >
              <n-icon :component="AddOutline" size="22" class="shrink-0 text-blue-400 mt-0.5" />
              <div class="min-w-0 flex-1">
                <div
                  class="text-[15px] font-medium overflow-hidden text-ellipsis whitespace-nowrap text-blue-500/80"
                >
                  {{ inputValue }}
                </div>
                <div class="flex items-center gap-3 mt-2 text-sm text-gray-400">
                  <span>
                    按
                    <Kbd>Enter</Kbd>
                    快速创建
                  </span>
                  <span>
                    或
                    <Kbd>Ctrl + Enter</Kbd>
                    添加详情
                  </span>
                </div>
              </div>
            </li>

            <li
              v-for="(todo, index) in filteredTodos"
              :key="todo.id"
              :ref="(el) => (todoItemRefs[index] = el as HTMLElement)"
              class="group relative flex items-start gap-3 px-[18px] py-3.5 box-border cursor-pointer border-l-4 border-transparent hover:bg-[var(--muted)]"
              :style="{ minHeight: `${TODO_ITEM_MIN_HEIGHT}px` }"
              :class="[
                todo.priority === 'high'
                  ? 'border-l-[#ff2d55]'
                  : todo.priority === 'medium'
                    ? 'border-l-[#f5b301]'
                    : 'border-l-[#16c784]',
                todo.completed ? 'text-gray-400 line-through' : '',
                selectedTodoIndex === index ? 'bg-[var(--muted)]' : '',
              ]"
              @click="openDetail(todo)"
              @mouseenter="selectedTodoIndex = index"
            >
              <button
                class="inline-flex items-center justify-center p-0 border-0 bg-transparent cursor-pointer"
                :class="todo.completed ? 'text-green-600' : 'text-gray-400'"
                type="button"
                @click.stop="toggleTodo(todo.id)"
              >
                <n-icon
                  :component="todo.completed ? CheckmarkCircleOutline : RadioButtonOffOutline"
                  size="22"
                />
              </button>

              <div class="min-w-0 flex-1">
                <div
                  class="text-[15px] font-medium overflow-hidden text-ellipsis whitespace-nowrap"
                >
                  {{ todo.title }}
                </div>
                <div class="flex items-center gap-2 mt-2 text-gray-500 text-[13px]">
                  <span class="inline-flex items-center gap-1">
                    <n-icon :component="TimeOutline" size="14" />
                    {{ formatDueDate(todo) }}
                  </span>
                  <n-tag
                    v-for="tag in parseTags(todo.tags)"
                    :key="tag"
                    size="small"
                    :bordered="false"
                    class="!text-xs"
                  >
                    {{ tag }}
                  </n-tag>
                </div>
              </div>

              <div
                class="absolute right-3 top-1/2 -translate-y-1/2 flex items-center gap-2 text-xs text-gray-400 opacity-0 group-hover:opacity-100 transition-opacity"
                :class="selectedTodoIndex === index ? 'opacity-100!' : ''"
              >
                <button
                  class="inline-flex items-center justify-center p-1 border-0 bg-transparent cursor-pointer text-gray-400 hover:text-red-500 rounded transition-colors"
                  type="button"
                  @click.stop="deleteTodoById(todo.id)"
                >
                  <n-icon :component="TrashOutline" size="16" />
                </button>
                <Kbd>→</Kbd>
                <span>详情</span>
              </div>
            </li>
          </ul>

          <div
            class="flex items-center justify-between px-3.5 border-t border-border box-border text-sm text-gray-500"
            :style="{ height: `${STATUS_BAR_HEIGHT}px` }"
          >
            <span>{{ activeCount }} 个进行中，{{ completedCount }} 个已完成</span>
            <button
              class="px-2.5 py-1.5 text-sm text-blue-600 bg-transparent border-0 cursor-pointer hover:underline"
              type="button"
              @click="clearCompleted"
            >
              清除已完成
            </button>
          </div>
        </div>
      </transition>

      <transition name="todo-slide" mode="out-in">
        <div
          v-if="viewState === 'list'"
          class="overflow-hidden"
          :style="{ height: `${TODO_LIST_HEIGHT}px` }"
        >
          <div
            class="flex items-center justify-between px-3.5 border-b border-border box-border"
            :style="{ height: `${FILTER_HEADER_HEIGHT}px` }"
          >
            <div class="flex flex-nowrap gap-2.5 shrink">
              <button
                v-for="item in filterTabs"
                :key="item.value"
                class="inline-flex items-center gap-2 h-8 px-2.5 border-0 rounded text-sm cursor-pointer whitespace-nowrap"
                :class="
                  activeFilter === item.value
                    ? 'text-blue-600 bg-blue-100'
                    : 'text-gray-600 bg-transparent hover:bg-gray-100'
                "
                type="button"
                @click="activeFilter = item.value"
              >
                {{ item.label }}
                <span class="text-xs opacity-70">{{ item.count }}</span>
              </button>
            </div>

            <n-select
              v-model:value="sortType"
              :options="sortOptions"
              size="small"
              :style="{ width: `calc(${sortTypeLabel.length * 2}ch + 38px ) !important` }"
              class="shrink-0"
              :consistent-menu-width="false"
            />
          </div>

          <ul
            class="m-0 p-0 overflow-y-auto list-none"
            :style="{ height: `${TODO_LIST_UL_HEIGHT}px` }"
          >
            <li
              v-for="(todo, index) in filteredTodos"
              :key="todo.id"
              :ref="(el) => (todoItemRefs[index] = el as HTMLElement)"
              class="group relative flex items-start gap-3 px-[18px] py-3.5 box-border cursor-pointer border-l-4 border-transparent hover:bg-[var(--muted)]"
              :style="{ minHeight: `${TODO_ITEM_MIN_HEIGHT}px` }"
              :class="[
                todo.priority === 'high'
                  ? 'border-l-[#ff2d55]'
                  : todo.priority === 'medium'
                    ? 'border-l-[#f5b301]'
                    : 'border-l-[#16c784]',
                todo.completed ? 'text-gray-400 line-through' : '',
                selectedTodoIndex === index ? 'bg-[var(--muted)]' : '',
              ]"
              @click="openDetail(todo)"
              @mouseenter="selectedTodoIndex = index"
            >
              <button
                class="inline-flex items-center justify-center p-0 border-0 bg-transparent cursor-pointer"
                :class="todo.completed ? 'text-green-600' : 'text-gray-400'"
                type="button"
                @click.stop="toggleTodo(todo.id)"
              >
                <n-icon
                  :component="todo.completed ? CheckmarkCircleOutline : RadioButtonOffOutline"
                  size="22"
                />
              </button>

              <div class="min-w-0 flex-1">
                <div
                  class="text-[15px] font-medium overflow-hidden text-ellipsis whitespace-nowrap"
                >
                  {{ todo.title }}
                </div>
                <div class="flex items-center gap-2 mt-2 text-gray-500 text-[13px]">
                  <span class="inline-flex items-center gap-1">
                    <n-icon :component="TimeOutline" size="14" />
                    {{ formatDueDate(todo) }}
                  </span>
                  <n-tag
                    v-for="tag in parseTags(todo.tags)"
                    :key="tag"
                    size="small"
                    :bordered="false"
                    class="!text-xs"
                  >
                    {{ tag }}
                  </n-tag>
                </div>
              </div>

              <div
                class="absolute right-3 top-1/2 -translate-y-1/2 flex items-center gap-2 text-xs text-gray-400 opacity-0 group-hover:opacity-100 transition-opacity"
                :class="selectedTodoIndex === index ? 'opacity-100!' : ''"
              >
                <button
                  class="inline-flex items-center justify-center p-1 border-0 bg-transparent cursor-pointer text-gray-400 hover:text-red-500 rounded transition-colors"
                  type="button"
                  @click.stop="deleteTodoById(todo.id)"
                >
                  <n-icon :component="TrashOutline" size="16" />
                </button>
                <Kbd>→</Kbd>
                <span>详情</span>
              </div>
            </li>
          </ul>

          <div
            class="flex items-center justify-between px-3.5 border-t border-border box-border text-sm text-gray-500"
            :style="{ height: `${STATUS_BAR_HEIGHT}px` }"
          >
            <span>{{ activeCount }} 个进行中，{{ completedCount }} 个已完成</span>
            <button
              class="px-2.5 py-1.5 text-sm text-blue-600 bg-transparent border-0 cursor-pointer hover:underline"
              type="button"
              @click="clearCompleted"
            >
              清除已完成
            </button>
          </div>
        </div>
      </transition>

      <transition name="todo-slide" mode="out-in">
        <div
          v-if="viewState === 'detail'"
          class="flex flex-col overflow-hidden bg-[var(--search-bg)]"
          :style="{ height: `${TODO_DETAIL_HEIGHT}px` }"
        >
          <div
            class="flex items-center gap-4 px-[18px] border-b border-border box-border font-medium shrink-0"
            :style="{ height: `${DETAIL_HEADER_HEIGHT}px` }"
          >
            <button
              class="inline-flex items-center justify-center p-0 border-0 text-gray-400 bg-transparent cursor-pointer hover:text-gray-600"
              type="button"
              @click="backToList"
            >
              <n-icon :component="ArrowBackOutline" size="22" />
            </button>
            <span>任务详情</span>
            <n-icon :component="EllipsisHorizontal" size="22" class="ml-auto" />
          </div>

          <div class="flex-1 min-h-0 px-7 py-5 overflow-y-auto box-border">
            <label class="flex items-center gap-3 mb-[22px]">
              <button
                class="inline-flex items-center justify-center p-0 border-0 bg-transparent cursor-pointer"
                :class="editingTodo.completed ? 'text-green-600' : 'text-gray-400'"
                type="button"
                @click="editingTodo.completed = !editingTodo.completed"
              >
                <n-icon
                  :component="
                    editingTodo.completed ? CheckmarkCircleOutline : RadioButtonOffOutline
                  "
                  size="28"
                />
              </button>
              <input
                v-model="editingTodo.title"
                class="flex-1 min-w-0 border-0 outline-none text-xl font-semibold bg-transparent color-[var(--foreground)]"
                type="text"
              />
            </label>

            <div class="grid grid-cols-[110px_1fr] gap-x-3 gap-y-4 pl-4 border-l-2 border-gray-200">
              <span class="inline-flex items-center gap-2 text-gray-600">
                <n-icon :component="PricetagOutline" size="18" />
                优先级
              </span>
              <div class="flex gap-2">
                <button
                  v-for="priority in priorityOptions"
                  :key="priority.value"
                  class="h-8 min-w-[44px] border-0 rounded cursor-pointer"
                  :class="getPriorityClass(priority.value)"
                  type="button"
                  @click="editingTodo.priority = priority.value"
                >
                  {{ priority.label }}
                </button>
              </div>

              <span class="inline-flex items-center gap-2 text-gray-600">
                <n-icon :component="TimeOutline" size="18" />
                截止时间
              </span>
              <input
                v-model="editingTodo.due_date"
                type="date"
                class="h-8 w-[160px] px-2.5 border border-border rounded text-sm color-[var(--foreground)] bg-[var(--search-bg)] outline-none"
              />

              <span class="inline-flex items-center gap-2 text-gray-600">
                <n-icon :component="PricetagOutline" size="18" />
                标签
              </span>
              <n-dynamic-tags v-model:value="tagList" size="small" :max="5" />

              <span class="inline-flex items-center gap-2 text-gray-600">
                <n-icon :component="TimeOutline" size="18" />
                创建时间
              </span>
              <span class="inline-flex items-center text-gray-500">{{
                formatDateTime(editingTodo.created_at)
              }}</span>
            </div>

            <label class="block mt-[22px] mb-2 text-gray-700">备注</label>
            <n-input
              v-model:value="editingTodo.note"
              type="textarea"
              :rows="4"
              placeholder="添加备注..."
              class="!resize-y"
            />
          </div>

          <div
            class="flex items-center justify-between px-[18px] border-t border-border box-border shrink-0"
            :style="{ height: `${STATUS_BAR_HEIGHT}px` }"
          >
            <n-button text type="error" class="!h-9 !px-3.5" @click="deleteEditingTodo">
              <template #icon>
                <n-icon :component="TrashOutline" size="18" />
              </template>
              删除任务
            </n-button>
            <div class="flex gap-2.5">
              <n-button quaternary class="!h-9 !px-3.5" @click="backToList"> 取消 </n-button>
              <n-button type="primary" class="!h-9 !px-3.5" @click="saveEditingTodo">
                保存更改
              </n-button>
            </div>
          </div>
        </div>
      </transition>
    </div>
  </section>
</template>

<script setup lang="ts">
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import {
  AddOutline,
  ArrowBackOutline,
  CheckboxOutline,
  CheckmarkCircleOutline,
  EllipsisHorizontal,
  PricetagOutline,
  RadioButtonOffOutline,
  TimeOutline,
  TrashOutline,
} from "@vicons/ionicons5";
import { computed, nextTick, ref, watch } from "vue";
import { addTodo, clearCompletedTodos, deleteTodo, getTodos, updateTodo } from "@/api";
import { SEARCH_INPUT_HEIGHT, SEARCH_WINDOW_WIDTH } from "@/constant";

type TodoFilter = "all" | "active" | "completed";
type TodoViewState = "empty" | "create" | "list" | "detail";
type TodoSort = "priority" | "createdAt" | "dueDate";

const props = withDefaults(
  defineProps<{
    keyword?: string;
    chromeHeight?: number;
  }>(),
  {
    keyword: "",
    chromeHeight: 0,
  },
);

const emit = defineEmits<{
  closeWindow: [isEscClose?: boolean];
}>();

const TODO_INPUT_HEIGHT = SEARCH_INPUT_HEIGHT;
const TODO_EMPTY_HEIGHT = 330;
const TODO_LIST_HEIGHT = 390;
const TODO_DETAIL_HEIGHT = 450;
const TODO_CREATE_HEIGHT = computed(() => TODO_LIST_HEIGHT);

const FILTER_HEADER_HEIGHT = 62;
const TODO_LIST_UL_HEIGHT = 282;
const STATUS_BAR_HEIGHT = 46;
const DETAIL_HEADER_HEIGHT = 38;
const EMPTY_ICON_SIZE = 74;
const TODO_ITEM_MIN_HEIGHT = 74;
const priorityWeight: Record<string, number> = { high: 0, medium: 1, low: 2 };
const priorityOptions: { label: string; value: TodoPriority }[] = [
  { label: "高", value: "high" },
  { label: "中", value: "medium" },
  { label: "低", value: "low" },
];
const sortOptions = [
  { label: "按优先级", value: "priority" },
  { label: "按创建时间", value: "createdAt" },
  { label: "按截止时间", value: "dueDate" },
];

const searchWindow = getCurrentWindow();
const inputRef = useTemplateRef<HTMLInputElement>("todoInputRef");
const inputValue = ref(props.keyword || "");
const todos = ref<TodoItem[]>([]);
const viewState = ref<TodoViewState>("empty");
const activeFilter = ref<TodoFilter>("all");
const sortType = ref<TodoSort>("priority");
const sortTypeLabel = computed(
  () => sortOptions.find((item) => item.value === sortType.value)!.label,
);
const editingTodo = ref<TodoItem>(createEmptyTodo());
const tagList = ref<string[]>([]);
const selectedTodoIndex = ref(-1);
const isEditing = computed(() => viewState.value === "detail");
const todoItemRefs = ref<HTMLElement[]>([]);

const chromeHeight = computed(() => props.chromeHeight);
const hasTodos = computed(() => todos.value.length > 0);
const activeCount = computed(() => todos.value.filter((item) => !item.completed).length);
const completedCount = computed(() => todos.value.filter((item) => item.completed).length);
const filterTabs = computed(() => [
  { label: "全部", value: "all" as const, count: todos.value.length },
  { label: "进行中", value: "active" as const, count: activeCount.value },
  { label: "已完成", value: "completed" as const, count: completedCount.value },
]);
const filteredTodos = computed(() => {
  const list = todos.value.filter((item) => {
    if (activeFilter.value === "active") return !item.completed;
    if (activeFilter.value === "completed") return item.completed;
    return true;
  });

  return [...list].sort((a, b) => {
    if (sortType.value === "priority")
      return priorityWeight[a.priority] - priorityWeight[b.priority];
    if (sortType.value === "dueDate")
      return (a.due_date || "9999-12-31").localeCompare(b.due_date || "9999-12-31");
    return b.created_at.localeCompare(a.created_at);
  });
});
const contentHeight = computed(() => {
  if (viewState.value === "detail") return TODO_DETAIL_HEIGHT;
  if (viewState.value === "list") return TODO_LIST_HEIGHT;
  if (viewState.value === "create") return TODO_CREATE_HEIGHT.value;
  return TODO_EMPTY_HEIGHT;
});
const windowHeight = computed(() => chromeHeight.value + TODO_INPUT_HEIGHT + contentHeight.value);
const defaultHeight = computed(
  () =>
    chromeHeight.value +
    TODO_INPUT_HEIGHT +
    (hasTodos.value ? TODO_LIST_HEIGHT : TODO_EMPTY_HEIGHT),
);

function focus() {
  inputRef.value?.focus();
}

function scrollToSelected() {
  nextTick(() => {
    todoItemRefs.value[selectedTodoIndex.value]?.scrollIntoView({
      behavior: "smooth",
      block: "nearest",
    });
  });
}

function resizeWindow() {
  searchWindow.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, windowHeight.value));
}

function createEmptyTodo(): TodoItem {
  return {
    id: 0,
    title: "",
    completed: false,
    priority: "low",
    due_date: null,
    tags: null,
    note: null,
    reminder_at: null,
    created_at: "",
    updated_at: "",
  };
}

function parseTags(tags: string | null): string[] {
  if (!tags) return [];
  try {
    return JSON.parse(tags);
  } catch {
    return [];
  }
}

function stringifyTags(tags: string[]): string | null {
  const filtered = tags.map((t) => t.trim()).filter(Boolean);
  return filtered.length ? JSON.stringify(filtered) : null;
}

async function loadTodos() {
  try {
    todos.value = await getTodos();
    viewState.value = todos.value.length ? "list" : "empty";
  } catch (e) {
    console.error("加载待办事项失败:", e);
  }
}

function syncCreateState() {
  if (inputValue.value.trim()) {
    viewState.value = "create";
    return;
  }
  viewState.value = hasTodos.value ? "list" : "empty";
}

async function createTodo() {
  const title = inputValue.value.trim();
  if (!title) return;
  try {
    const todo = await addTodo(title, "low");
    todos.value.unshift(todo);
    inputValue.value = "";
    viewState.value = "list";
  } catch (e) {
    console.error("创建待办事项失败:", e);
  }
}

async function openDetailDraft() {
  const title = inputValue.value.trim();
  if (!title) return;
  try {
    const todo = await addTodo(title, "low");
    todos.value.unshift(todo);
    editingTodo.value = JSON.parse(JSON.stringify(todo));
    tagList.value = [];
    inputValue.value = "";
    viewState.value = "detail";
  } catch (e) {
    console.error("创建待办事项失败:", e);
  }
}

function openDetail(todo: TodoItem) {
  editingTodo.value = JSON.parse(JSON.stringify(todo));
  tagList.value = parseTags(todo.tags);
  inputValue.value = "";
  viewState.value = "detail";
}

async function saveEditingTodo() {
  const title = editingTodo.value.title.trim();
  if (!title) return;

  const tagsStr = stringifyTags(tagList.value);
  try {
    const updated = await updateTodo(editingTodo.value.id, {
      title,
      completed: editingTodo.value.completed,
      priority: editingTodo.value.priority,
      due_date: editingTodo.value.due_date || null,
      tags: tagsStr,
      note: editingTodo.value.note || null,
      reminder_at: editingTodo.value.reminder_at || null,
    });
    const index = todos.value.findIndex((item) => item.id === updated.id);
    if (index >= 0) {
      todos.value.splice(index, 1, updated);
    }
    viewState.value = "list";
  } catch (e) {
    console.error("保存待办事项失败:", e);
  }
}

async function deleteEditingTodo() {
  try {
    await deleteTodo(editingTodo.value.id);
    todos.value = todos.value.filter((item) => item.id !== editingTodo.value.id);
    inputValue.value = "";
    backToList();
  } catch (e) {
    console.error("删除待办事项失败:", e);
  }
}

async function deleteTodoById(id: number) {
  try {
    await deleteTodo(id);
    todos.value = todos.value.filter((item) => item.id !== id);
    if (!todos.value.length) viewState.value = "empty";
  } catch (e) {
    console.error("删除待办事项失败:", e);
  }
}

function backToList() {
  viewState.value = hasTodos.value ? "list" : "empty";
  selectedTodoIndex.value = -1;
}

async function toggleTodo(id: number) {
  const item = todos.value.find((todo) => todo.id === id);
  if (!item) return;
  try {
    const updated = await updateTodo(id, { completed: !item.completed });
    const index = todos.value.findIndex((t) => t.id === id);
    if (index >= 0) {
      todos.value.splice(index, 1, updated);
    }
  } catch (e) {
    console.error("更新待办事项失败:", e);
  }
}

async function clearCompleted() {
  try {
    await clearCompletedTodos();
    todos.value = todos.value.filter((item) => !item.completed);
    if (!todos.value.length) viewState.value = "empty";
  } catch (e) {
    console.error("清除已完成待办事项失败:", e);
  }
}

function formatDueDate(todo: TodoItem) {
  if (todo.completed) return "已完成";
  if (!todo.due_date) return "今天";

  const today = new Date();
  const due = new Date(`${todo.due_date}T00:00:00`);
  const start = new Date(today.getFullYear(), today.getMonth(), today.getDate()).getTime();
  const diff = Math.round((due.getTime() - start) / 86400000);
  if (diff === 0) return "今天";
  if (diff === 1) return "明天";
  if (diff > 1 && diff < 7) return `还剩 ${diff} 天`;
  return todo.due_date.replaceAll("-", "/");
}

function formatDateTime(value: string) {
  const date = new Date(value);
  const pad = (num: number) => `${num}`.padStart(2, "0");
  return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())} ${pad(date.getHours())}:${pad(date.getMinutes())}`;
}

function getPriorityClass(priority: TodoPriority) {
  const isActive = editingTodo.value.priority === priority;
  if (priority === "high")
    return isActive ? "text-red-600 bg-red-50 outline outline-red-200" : "text-gray-700 bg-gray-50";
  if (priority === "medium")
    return isActive
      ? "text-amber-700 bg-amber-50 outline outline-amber-200"
      : "text-gray-700 bg-gray-50";
  return isActive
    ? "text-green-700 bg-green-50 outline outline-green-200"
    : "text-gray-700 bg-gray-50";
}

function handleClose() {
  inputValue.value = "";
  selectedTodoIndex.value = -1;
  viewState.value = hasTodos.value ? "list" : "empty";
  resizeWindow();
}

function handleKeydown(event: KeyboardEvent) {
  const { key } = event;
  const todoCount = filteredTodos.value.length;

  // detail 视图：Esc/右键 返回，Enter 保存
  if (viewState.value === "detail") {
    if (key === "Escape" || key === "ArrowLeft") {
      backToList();
      event.preventDefault();
    } else if (key === "Enter") {
      saveEditingTodo();
      event.preventDefault();
    }
    return;
  }

  // Esc：create 视图退回列表，其他视图关闭窗口
  if (key === "Escape") {
    if (viewState.value === "create") {
      inputValue.value = "";
      viewState.value = hasTodos.value ? "list" : "empty";
      resizeWindow();
    } else {
      emit("closeWindow", true);
    }
    event.preventDefault();
    return;
  }

  // Enter 创建待办
  if (key === "Enter" && event.ctrlKey && inputValue.value.trim()) {
    openDetailDraft();
    event.preventDefault();
    return;
  }
  if (key === "Enter" && inputValue.value.trim()) {
    createTodo();
    event.preventDefault();
    return;
  }

  // 上下键导航 todo 列表
  if (key === "ArrowDown") {
    if (todoCount) {
      selectedTodoIndex.value =
        selectedTodoIndex.value < todoCount - 1 ? selectedTodoIndex.value + 1 : 0;
      scrollToSelected();
    }
    event.preventDefault();
    return;
  }
  if (key === "ArrowUp") {
    if (todoCount) {
      selectedTodoIndex.value =
        selectedTodoIndex.value > 0 ? selectedTodoIndex.value - 1 : todoCount - 1;
      scrollToSelected();
    }
    event.preventDefault();
    return;
  }

  // 右键进入详情
  if (key === "ArrowRight" && selectedTodoIndex.value >= 0) {
    const todo = filteredTodos.value[selectedTodoIndex.value];
    if (todo) openDetail(todo);
    event.preventDefault();
    return;
  }

  // Enter 切换完成状态
  if (key === "Enter" && selectedTodoIndex.value >= 0 && !inputValue.value.trim()) {
    const todo = filteredTodos.value[selectedTodoIndex.value];
    if (todo) toggleTodo(todo.id);
    event.preventDefault();
  }
}

watch(viewState, resizeWindow);
watch(contentHeight, resizeWindow);
watch(chromeHeight, resizeWindow);

onMounted(async () => {
  await loadTodos();
  nextTick(() => {
    focus();
    resizeWindow();
  });
});

defineExpose({
  focus,
  handleClose,
  handleKeydown,
  getDefaultHeight: () => defaultHeight.value,
});
</script>

<style scoped>
.n-input {
  --n-border-hover: 0px !important;
  --n-border-focus: 0px !important;
  --n-border: 0px !important;
  --n-caret-color: gray !important;
  --n-height: 100% !important;
  --n-font-size: 18px !important;

  border-radius: 5px;
  border: none !important;
}

::v-deep(.n-input__placeholder) {
  font-size: 14px !important;
  margin-left: 5px;
}

.input-container {
  width: 100%;
  height: 45px;
  position: relative;
  display: block;
}

.todo-fade-enter-active,
.todo-fade-leave-active {
  position: absolute;
  width: 100%;
  transition: opacity 0.15s ease;
}
.todo-fade-enter-from,
.todo-fade-leave-to {
  opacity: 0;
}

.todo-slide-enter-active,
.todo-slide-leave-active {
  position: absolute;
  width: 100%;
  transition:
    opacity 0.2s ease,
    transform 0.2s ease;
}
.todo-slide-enter-from {
  opacity: 0;
  transform: translateY(6px);
}
.todo-slide-leave-to {
  opacity: 0;
  transform: translateY(-6px);
}

.content {
  box-sizing: border-box;
  border-top: 0.5px solid;
}
</style>
