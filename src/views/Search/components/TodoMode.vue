<template>
  <section class="todo-mode">
    <label class="flex items-center gap-3 h-[45px] px-3.5 border-b border-border box-border">
      <!-- :class="{ 'border-transparent!': inputValue.length && viewState === 'list' }" -->
      <n-icon :component="CheckboxOutline" size="22" class="shrink-0 text-blue-600" />
      <input
        ref="todoInputRef"
        v-model="inputValue"
        class="flex-1 min-w-0 h-full border-0 outline-none text-[18px] bg-transparent color-[var(--foreground)] placeholder:text-[#7c8798]"
        type="text"
        placeholder="输入任务内容，回车创建..."
        @input="syncCreateState"
      />
      <button
        v-if="hasTodos"
        class="h-7 px-2.5 text-sm text-blue-600 bg-blue-50 rounded cursor-pointer border-0 hover:bg-blue-100"
        type="button"
        @click="viewState = 'list'"
      >
        查看全部
      </button>
    </label>

    <div class="relative">
      <transition name="todo-fade" mode="out-in">
        <div
          v-if="viewState === 'empty'"
          class="flex flex-col items-center justify-center h-[330px] text-center"
        >
          <div
            class="grid place-items-center w-[74px] h-[74px] mb-5 text-gray-400 bg-gray-100 rounded-full"
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
        <div v-if="viewState === 'create'" class="relative px-5 py-2.5 box-border shadow-lg">
          <div class="text-lg font-semibold">{{ inputValue }}</div>
          <div class="flex items-center justify-between mt-3">
            <div class="flex items-center gap-2 text-sm text-gray-500">
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

            <button
              class="text-sm text-blue-600 bg-transparent border-0 cursor-pointer hover:underline"
              type="button"
              @click="openDetailDraft"
            >
              添加详情 ->
            </button>
          </div>
        </div>
      </transition>

      <transition name="todo-slide" mode="out-in">
        <div v-if="viewState === 'list'" class="h-[390px] overflow-hidden">
          <div
            class="flex items-center justify-between h-[62px] px-3.5 border-b border-border box-border"
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

          <ul class="h-[282px] m-0 p-0 overflow-y-auto list-none">
            <li
              v-for="todo in filteredTodos"
              :key="todo.id"
              class="flex items-start gap-3 min-h-[74px] px-[18px] py-3.5 box-border cursor-pointer border-l-4 border-transparent hover:bg-[var(--muted)]"
              :class="[
                todo.priority === 'high'
                  ? 'border-l-[#ff2d55]'
                  : todo.priority === 'medium'
                    ? 'border-l-[#f5b301]'
                    : 'border-l-[#16c784]',
                todo.completed ? 'text-gray-400 line-through' : '',
              ]"
              @click="openDetail(todo)"
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

              <div class="min-w-0">
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
            </li>
          </ul>

          <div
            class="flex items-center justify-between h-[46px] px-3.5 border-t border-border box-border text-sm text-gray-500"
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
        <div v-if="viewState === 'detail'" class="h-[590px] overflow-hidden bg-[var(--search-bg)]">
          <div
            class="flex items-center gap-4 h-[52px] px-[18px] border-b border-border box-border font-medium"
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

          <div class="h-[452px] px-7 py-5 overflow-y-auto box-border">
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
              <n-input
                v-model:value="tagText"
                size="small"
                placeholder="用空格分隔标签"
                class="!w-[200px]"
              />

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
            class="flex items-center justify-between h-[86px] px-[18px] border-t border-border box-border"
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
const TODO_DETAIL_HEIGHT = 590;
const TODO_CREATE_HEIGHT = computed(() => 80);
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
const tagText = ref("");

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

function resizeWindow() {
  searchWindow.setSize(new LogicalSize(SEARCH_WINDOW_WIDTH, windowHeight.value));
}

function createEmptyTodo(): TodoItem {
  return {
    id: 0,
    title: "",
    completed: false,
    priority: "medium",
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
    const todo = await addTodo(title, "medium");
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
    const todo = await addTodo(title, "medium");
    todos.value.unshift(todo);
    editingTodo.value = JSON.parse(JSON.stringify(todo));
    tagText.value = "";
    inputValue.value = "";
    viewState.value = "detail";
  } catch (e) {
    console.error("创建待办事项失败:", e);
  }
}

function openDetail(todo: TodoItem) {
  editingTodo.value = JSON.parse(JSON.stringify(todo));
  tagText.value = parseTags(todo.tags).join(" ");
  viewState.value = "detail";
}

async function saveEditingTodo() {
  const title = editingTodo.value.title.trim();
  if (!title) return;

  const tagsStr = stringifyTags(tagText.value.split(/\s+/));
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

function backToList() {
  viewState.value = hasTodos.value ? "list" : "empty";
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
  viewState.value = hasTodos.value ? "list" : "empty";
  resizeWindow();
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === "Escape") {
    if (viewState.value === "detail") {
      backToList();
    } else {
      emit("closeWindow", true);
    }
    event.preventDefault();
    return;
  }

  if (event.key === "Enter" && event.ctrlKey && viewState.value === "create") {
    openDetailDraft();
    event.preventDefault();
    return;
  }

  if (event.key === "Enter" && viewState.value === "create") {
    createTodo();
    event.preventDefault();
    return;
  }

  if (event.key === "Enter" && viewState.value === "detail") {
    saveEditingTodo();
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
</style>
