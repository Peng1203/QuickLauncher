<template>
  <section class="todo-mode">
    <label class="input-container max-h-11.25 border-b border-border">
      <n-input
        style="--n-border: 0px !important"
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
      <transition :name="`todo-slide-${direction}`" mode="out-in">
        <!-- 没有任何待办项时 -->
        <div
          v-if="isEmptyState"
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

        <!-- 添加预览/列表 -->
        <div
          v-else-if="isCreateState || isListState"
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
              v-if="viewState === 'create'"
              class="flex items-start gap-3 px-4.5 py-3.5 box-border border-l-4 border-l-blue-400/40 bg-blue-50/20 opacity-50"
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
              v-for="(todo, index) in todos"
              :key="todo.id"
              :ref="(el) => (todoItemRefs[index] = el as HTMLElement)"
              class="group relative flex items-start gap-3 px-4.5 py-3.5 box-border cursor-pointer border-l-4 border-transparent hover:bg-muted"
              :style="{
                minHeight: `${TODO_ITEM_MIN_HEIGHT}px`,
                borderLeftColor: getPriorityColor(todo.priority),
              }"
              :class="[
                todo.completed ? 'text-gray-400 line-through' : '',
                selectedTodoIndex === index ? 'bg-muted' : '',
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
                  <span v-if="todo.due_date" class="inline-flex items-center gap-1">
                    <n-icon :component="TimeOutline" size="14" />
                    {{ formatDueDate(todo) }}
                  </span>

                  <n-tag
                    v-for="tag in renderTags(todo.tags)"
                    size="small"
                    type="primary"
                    :key="tag"
                    :bordered="false"
                    class="text-xs!"
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

        <!-- 待办项详情 -->
        <div
          v-else-if="isDetailState"
          class="flex flex-col overflow-hidden bg-[var(--search-bg)]"
          :style="{ height: `${TODO_DETAIL_HEIGHT}px` }"
        >
          <div
            class="flex items-center gap-4 px-4.5 border-b border-border box-border font-medium shrink-0"
            :style="{ height: `${DETAIL_HEADER_HEIGHT}px` }"
          >
            <button
              class="inline-flex items-center justify-center p-0 border-0 text-gray-400 bg-transparent cursor-pointer hover:text-gray-600"
              type="button"
              @click="backToList"
            >
              <n-icon :component="ArrowBackOutline" size="22" />
            </button>
            <span>{{ isDetailCreate ? "新建任务" : "任务详情" }}</span>
            <n-icon :component="EllipsisHorizontal" size="22" class="ml-auto" />
          </div>
          <!-- {{ editingTodo }} -->

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
              <n-input
                style="--n-border: 0px !important; --n-padding-left: 3px"
                v-model:value="editingTodo.title"
                class="flex-1 min-w-0 border-0 outline-none text-xl font-semibold bg-transparent color-[var(--foreground)]"
                type="text"
              />
            </label>

            <div class="grid grid-cols-[110px_1fr] gap-x-3 gap-y-4 pl-4 border-l-2 border-gray-200">
              <span class="inline-flex items-center gap-2 text-gray-600">
                <n-icon :component="PricetagOutline" size="18" />
                优先级
              </span>
              <PrioritySelector v-model="editingTodo.priority" />

              <span class="inline-flex items-center gap-2 text-gray-600">
                <n-icon :component="TimeOutline" size="18" />
                截止时间
              </span>
              <div class="flex items-center gap-2">
                <n-date-picker
                  v-model:value="editingTodo.due_date"
                  clearable
                  class="w-50!"
                  type="datetime"
                  size="small"
                  :shortcuts="dueDateshortcuts"
                />
                <span
                  v-if="dueDateDays !== null"
                  class="text-xs"
                  :class="
                    dueDateDays < 0
                      ? 'text-red-500'
                      : dueDateDays === 0
                        ? 'text-amber-500'
                        : 'text-gray-500'
                  "
                >
                  {{ dueDateDaysLabel }}
                </span>
              </div>

              <span class="inline-flex items-center gap-2 text-gray-600">
                <n-icon :component="NotificationsOutline" size="18" />
                提醒时间
              </span>

              <n-date-picker
                v-model:value="editingTodo.reminder_at"
                clearable
                class="w-50!"
                type="datetime"
                size="small"
                :shortcuts="reminderAtshortcuts"
                :is-date-disabled="reminderDateDisabled"
                :is-time-disabled="reminderTimeDisabled"
              />

              <span class="inline-flex items-center gap-2 text-gray-600">
                <n-icon :component="PricetagOutline" size="18" />
                标签
              </span>
              <n-dynamic-tags v-model:value="tagList" size="small" type="primary" :max="5" />

              <template v-if="editingTodo.created_at">
                <span class="inline-flex items-center gap-2 text-gray-600">
                  <n-icon :component="TimeOutline" size="18" />
                  创建时间
                </span>
                <span class="inline-flex items-center text-gray-500">
                  {{ dateTimeFormat(editingTodo.created_at) }}
                  <b class="ml-3"> 创建于 {{ getFromNow(editingTodo.created_at) }} </b>
                </span>
              </template>
            </div>

            <label class="block mt-5.5 mb-2 text-gray-700">备注</label>
            <n-input
              v-model:value="editingTodo.note"
              type="textarea"
              placeholder="添加备注..."
              :rows="5"
            />
            <!-- style="border: var(--n-border) !important" -->
            <!-- class="h-10!" -->
            <!--  -->
          </div>

          <div
            class="flex items-center justify-between px-[18px] border-t border-border box-border shrink-0"
            :style="{ height: `${STATUS_BAR_HEIGHT}px` }"
          >
            <n-button
              v-if="isDetailEdit"
              text
              type="error"
              class="!h-9 !px-3.5"
              @click="deleteEditingTodo"
            >
              <template #icon>
                <n-icon :component="TrashOutline" size="18" />
              </template>
              删除任务
            </n-button>
            <div v-else />
            <div class="flex gap-2.5">
              <n-button quaternary class="!h-9 !px-3.5" @click="backToList"> 取消 </n-button>
              <n-button type="primary" class="!h-9 !px-3.5" @click="saveEditingTodo">
                {{ isDetailCreate ? "创建" : "保存更改" }}
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
  NotificationsOutline,
  PricetagOutline,
  RadioButtonOffOutline,
  TimeOutline,
  TrashOutline,
} from "@vicons/ionicons5";
import { computed, nextTick, ref, watch } from "vue";
import { addTodo, clearCompletedTodos, deleteTodo, getTodos, updateTodo } from "@/api";
import { SEARCH_INPUT_HEIGHT, SEARCH_WINDOW_WIDTH } from "@/constant";
import { getDaysUntil, getFromNow } from "@/utils/date";
import { dateTimeFormat } from "@/utils/date";
import PrioritySelector from "./components/PrioritySelector.vue";
import { useTodoDomain } from "./useTodoDomain";
import type { TodoViewState } from "./index";
import { useTodoViewState } from "./useTodoViewState";

type TodoFilter = "all" | "active" | "completed";
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
const TODO_DETAIL_HEIGHT = 490;
const TODO_CREATE_HEIGHT = computed(() => TODO_LIST_HEIGHT);

const FILTER_HEADER_HEIGHT = 62;
const TODO_LIST_UL_HEIGHT = 282;
const STATUS_BAR_HEIGHT = 46;
const DETAIL_HEADER_HEIGHT = 38;
const EMPTY_ICON_SIZE = 74;
const TODO_ITEM_MIN_HEIGHT = 74;
const priorityWeight: Record<string, number> = { high: 0, medium: 1, low: 2 };

const sortOptions = [
  { label: "按优先级", value: "priority" },
  { label: "按创建时间", value: "createdAt" },
  { label: "按截止时间", value: "dueDate" },
];

const searchWindow = getCurrentWindow();
const inputRef = useTemplateRef<HTMLInputElement>("todoInputRef");
const inputValue = ref(props.keyword || "");
const todos = ref<TodoItem[]>([]);
const activeFilter = ref<TodoFilter>("all");
const sortType = ref<TodoSort>("priority");
const sortTypeLabel = computed(
  () => sortOptions.find((item) => item.value === sortType.value)!.label,
);
const editingTodo = ref<TodoItem>({
  id: 0,
  title: "",
  completed: false,
  priority: "low",
  due_date: null,
  tags: null,
  note: null,
  reminder_at: null,
  created_at: 0,
  updated_at: 0,
});

const {
  tagList,
  dueDateDays,
  dueDateDaysLabel,
  dueDateshortcuts,
  reminderAtshortcuts,
  renderTags,
  formatDueDate,
  getPriorityColor,
  reminderDateDisabled,
  reminderTimeDisabled,
} = useTodoDomain(editingTodo);

const {
  viewState,
  // previousState,
  // stateOrder,
  isEmptyState,
  isCreateState,
  isListState,
  isDetailCreate,
  isDetailEdit,
  isDetailState,
  isEditing,
  direction,
  setViewState,
} = useTodoViewState(loadTodos);

const selectedTodoIndex = ref(-1);
const todoItemRefs = ref<HTMLElement[]>([]);

// const dueDateDays = computed(() => getDaysUntil(editingTodo.value.due_date));
// const dueDateDaysLabel = computed(() => {
//   const days = dueDateDays.value;
//   if (days === null) return "";
//   if (days < 0) return `已过期 ${Math.abs(days)} 天`;
//   if (days === 0) return "今天截止";
//   if (days === 1) return "明天截止";
//   return `还剩 ${days} 天`;
// });

const chromeHeight = computed(() => props.chromeHeight);
const hasTodos = computed(() => todos.value.length > 0);
const activeCount = computed(() => todos.value.filter((item) => !item.completed).length);
const completedCount = computed(() => todos.value.filter((item) => item.completed).length);
const filterTabs = computed(() => [
  { label: "全部", value: "all" as const, count: todos.value.length },
  { label: "进行中", value: "active" as const, count: activeCount.value },
  { label: "已完成", value: "completed" as const, count: completedCount.value },
]);

const contentHeight = computed(() => {
  if (isDetailState.value) return TODO_DETAIL_HEIGHT;
  if (isListState.value) return TODO_LIST_HEIGHT;
  if (isCreateState.value) return TODO_CREATE_HEIGHT.value;
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
    created_at: 0,
    updated_at: 0,
  };
}

async function loadTodos() {
  try {
    todos.value = await getTodos();
    setViewState(todos.value.length ? "list" : "empty");
  } catch (e) {
    console.error("加载待办事项失败:", e);
  }
}

function syncCreateState() {
  if (inputValue.value.trim()) {
    setViewState("create");
    return;
  }
  setViewState(hasTodos.value ? "list" : "empty");
}

function openDetailDraft() {
  const title = inputValue.value.trim();
  if (!title) return;
  editingTodo.value = { ...createEmptyTodo(), title };
  tagList.value = [];
  inputValue.value = "";
  setViewState("detail-create");
}

function openDetail(todo: TodoItem) {
  editingTodo.value = JSON.parse(JSON.stringify(todo));
  inputValue.value = "";
  setViewState("detail-edit");
}

async function saveEditingTodo() {
  const title = editingTodo.value.title.trim();
  if (!title) return;

  try {
    if (isDetailCreate.value) {
      await addNewTodo();
    } else {
      await saveEditTodo();
    }
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
    if (!todos.value.length) setViewState("empty");
  } catch (e) {
    console.error("删除待办事项失败:", e);
  }
}

function backToList() {
  setViewState(hasTodos.value ? "list" : "empty");
  // 编辑返回时保持原选中，新建返回时重置
  if (isDetailCreate.value) {
    selectedTodoIndex.value = -1;
  }
  nextTick(focus);
}

async function toggleTodo(id: number) {
  const item = todos.value.find((todo) => todo.id === id);
  if (!item) return;
  try {
    const updated = await updateTodo({ ...item, completed: !item.completed });
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
    if (!todos.value.length) setViewState("empty");
  } catch (e) {
    console.error("清除已完成待办事项失败:", e);
  }
}

function handleClose() {
  inputValue.value = "";
  selectedTodoIndex.value = -1;
  setViewState(hasTodos.value ? "list" : "empty");
  resizeWindow();
}

async function saveEditTodo() {
  try {
    await updateTodo(editingTodo.value);

    setViewState("list", true);
    nextTick(focus);
  } catch (e) {
    console.log("e", e);
  }
}

async function addNewTodo(isFastAdd: boolean = false) {
  try {
    const title = inputValue.value.trim();
    if (isFastAdd && !title) return;

    let item: NewTodoItem = {
      priority: "high",
      title: "",
      due_date: null,
      tags: "",
      note: "",
      reminder_at: null,
    };

    if (isFastAdd) {
      item.title = title;
      item.priority = "low";
    } else {
      for (const key in editingTodo.value) {
        // @ts-ignore
        if (item[key] !== undefined) item[key] = editingTodo.value[key];
      }
    }
    console.log("item", { ...item });
    const todo = await addTodo(item);
    todos.value.unshift(todo);
    inputValue.value = "";
    selectedTodoIndex.value = 0;
    setViewState("list", true);
  } catch (e) {
    console.log("e", e);
  }
}

function isTypingTarget(el: EventTarget | null) {
  if (!(el instanceof HTMLElement)) return false;

  const tag = el.tagName;
  return tag === "INPUT" || tag === "TEXTAREA" || el.isContentEditable;
}

function handleKeydown(event: KeyboardEvent) {
  const { key } = event;
  const todoCount = todos.value.length;

  // detail 视图：Esc/右键 返回，Enter 保存
  if (isDetailState.value) {
    if (isTypingTarget(event.target)) return;
    // || key === "ArrowLeft"
    if (key === "Escape") {
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
    if (isCreateState.value) {
      inputValue.value = "";
      setViewState(hasTodos.value ? "list" : "empty");
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
    // createTodo();
    addNewTodo(true);
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
    const todo = todos.value[selectedTodoIndex.value];
    if (todo) openDetail(todo);
    event.preventDefault();
    return;
  }

  // Enter 切换完成状态
  if (key === "Enter" && selectedTodoIndex.value >= 0 && !inputValue.value.trim()) {
    const todo = todos.value[selectedTodoIndex.value];
    if (todo) toggleTodo(todo.id);
    event.preventDefault();
  }
}

watch(viewState, resizeWindow);
watch(contentHeight, resizeWindow);
watch(chromeHeight, resizeWindow);

loadTodos();
onMounted(async () => {
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

<style scoped lang="scss">
.n-input {
  --n-border-hover: 0px !important;
  --n-border-focus: 0px !important;
  --n-caret-color: gray !important;
  --n-font-size: 18px !important;
  --n-border-disabled: none !important;
  --n-height: 100% !important;
  border-radius: 5px;
  // border: none !important;
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

.todo-slide-left-enter-active,
.todo-slide-left-leave-active {
  position: absolute;
  width: 100%;
  transition:
    opacity 0.2s ease,
    transform 0.2s ease;
}

.todo-slide-left-enter-from {
  opacity: 0;
  transform: translateX(18px) scale(0.99);
}

.todo-slide-left-leave-to {
  opacity: 0;
  transform: translateX(-18px) scale(0.99);
}

.todo-slide-right-enter-active,
.todo-slide-right-leave-active {
  position: absolute;
  width: 100%;
  transition:
    opacity 0.2s ease,
    transform 0.2s ease;
}

.todo-slide-right-enter-from {
  opacity: 0;
  transform: translateX(-18px) scale(0.99);
}

.todo-slide-right-leave-to {
  opacity: 0;
  transform: translateX(18px) scale(0.99);
}

::v-deep(.n-input__textarea-el) {
  padding: 0 !important;
}
</style>

<style>
.v-binder-follower-content {
  scale: 0.88;
}
</style>
