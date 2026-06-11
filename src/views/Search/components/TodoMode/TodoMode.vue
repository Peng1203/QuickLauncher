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
        :placeholder="isEditing ? t('todo.editingPlaceholder') : t('todo.inputPlaceholder')"
        :class="viewState === 'create' ? 'border-b-0! rounded-b-none!' : ''"
        @input="syncCreateState"
      >
        <template #prefix>
          <Icon name="icon-TODO_INFO" size="22" color="#155dfc" />
        </template>
        <template v-if="isListState" #suffix>
          <div class="shortcut-list">
            <span class="shortcut-item">
              <Kbd>↑↓</Kbd>
              <span>{{ t("todo.switch") }}</span>
            </span>

            <span class="shortcut-item">
              <Kbd>↵</Kbd>
              <span>{{ t("todo.complete") }}</span>
            </span>

            <span class="shortcut-item">
              <Kbd>Tab</Kbd>
              <span>{{ t("todo.switchSort") }}</span>
            </span>
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
            <Icon name="icon-fangkuangxuanzhong" size="42" />
          </div>
          <div class="text-xl font-semibold">{{ t("todo.emptyTitle") }}</div>
          <div class="mt-2.5 text-sm text-gray-500">{{ t("todo.emptyDesc") }}</div>
          <div
            class="inline-flex items-center gap-2 mt-7 px-3.5 py-2 text-gray-700 bg-gray-50 rounded-lg text-sm"
          >
            <Icon name="icon-add" />
            {{ t("todo.emptyHint") }}
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

            <!-- 排序 -->
            <div class="flex items-center gap-1 shrink-0">
              <n-select
                v-model:value="sortType"
                size="small"
                :options="sortOptions"
                :style="{ width: `calc(${sortTypeLabel.length * 2}ch + 38px ) !important` }"
                :consistent-menu-width="false"
              />
            </div>
          </div>

          <ul
            class="m-0 p-0 overflow-y-auto list-none"
            :style="{ height: `${TODO_LIST_UL_HEIGHT}px` }"
          >
            <TransitionGroup name="todo" tag="ul" class="space-y-0">
              <!-- 虚拟预览项 -->
              <li
                v-if="viewState === 'create'"
                class="h-20 flex items-start gap-3 px-4.5 py-3.5 box-border border-l-4 border-l-blue-400/40 bg-blue-50/20 opacity-50"
                :style="{ minHeight: `${TODO_ITEM_MIN_HEIGHT}px` }"
              >
                <Icon name="icon-add" size="22" color="oklch(70.7% 0.165 254.624)" />
                <div class="min-w-0 flex-1">
                  <div
                    class="text-[15px] font-medium overflow-hidden text-ellipsis whitespace-nowrap text-blue-500/80"
                  >
                    {{ inputValue }}
                  </div>
                  <div class="flex items-center gap-3 mt-2 text-sm text-gray-400">
                    <span>
                      {{ t("todo.pressEnter") }}
                      <Kbd>Enter</Kbd>
                      {{ t("todo.quickCreate") }}
                    </span>
                    <span>
                      {{ t("todo.or") }}
                      <Kbd>Ctrl + Enter</Kbd>
                      {{ t("todo.addDetail") }}
                    </span>
                  </div>
                </div>
              </li>

              <TodoItem
                v-for="(todo, index) in todos"
                :key="todo.id"
                :ref="(el) => (todoItemRefs[index] = (el as any)?.$el)"
                :todo="todo"
                :selected="selectedTodoIndex === index"
                :min-height="TODO_ITEM_MIN_HEIGHT"
                @open="openDetail"
                @toggle="toggleTodo"
                @delete="deleteTodoById"
              />
            </TransitionGroup>
          </ul>

          <div
            class="flex items-center justify-between px-3.5 border-t border-border box-border text-sm text-gray-500"
            :style="{ height: `${STATUS_BAR_HEIGHT}px` }"
          >
            <span
              >{{ activeCount }} {{ t("todo.activeCount") }}，{{ completedCount }}
              {{ t("todo.completedCount") }}</span
            >
            <button
              class="px-2.5 py-1.5 text-sm text-blue-600 bg-transparent border-0 cursor-pointer hover:underline"
              type="button"
              @click="clearCompleted"
            >
              {{ t("todo.clearCompleted") }}
            </button>
          </div>
        </div>

        <!-- 待办项详情 -->
        <div
          v-else-if="isDetailState"
          class="flex flex-col overflow-hidden bg-(--search-bg)"
          :style="{ height: `${TODO_DETAIL_HEIGHT}px` }"
        >
          <div
            class="flex-sb-c gap-4 px-4.5 border-b border-border box-border font-medium shrink-0"
            :style="{ height: `${DETAIL_HEADER_HEIGHT}px` }"
          >
            <div class="flex-s-c">
              <button
                class="p-0 border-0 text-gray-400 bg-transparent cursor-pointer hover:text-gray-600"
                type="button"
                @click="backToList"
              >
                <Icon name="icon-fanhui" size="22" />
              </button>
              <span class="flex ml-2 h-5.5">
                {{ isDetailCreate ? t("todo.newTask") : t("todo.taskDetail") }}
              </span>
            </div>

            <Icon name="icon-gengduo" size="22" />
          </div>
          <!-- {{ editingTodo }} -->

          <div class="flex-1 min-h-0 px-7 py-5 overflow-y-auto box-border">
            <label class="flex items-center gap-3 mb-5.5">
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
                <Icon name="icon-shandian" size="18" />
                {{ t("todo.priority") }}
              </span>
              <PrioritySelector v-model="editingTodo.priority" />

              <span class="inline-flex items-center gap-2 text-gray-600">
                <Icon name="icon-jiezhishijian1" />
                {{ t("todo.dueDate") }}
              </span>
              <div class="flex items-center gap-2">
                <n-date-picker
                  v-model:value="editingTodo.due_date"
                  clearable
                  class="w-50! todo-date-picker"
                  type="datetime"
                  size="small"
                  :to="false"
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
                <Icon name="icon-tixingshijian" />
                {{ t("todo.reminderTime") }}
              </span>

              <div class="flex items-center gap-2">
                <n-date-picker
                  v-model:value="editingTodo.reminder_at"
                  clearable
                  class="w-50! todo-date-picker"
                  type="datetime"
                  size="small"
                  :to="false"
                  :shortcuts="reminderAtshortcuts"
                  :is-date-disabled="reminderDateDisabled"
                  :is-time-disabled="reminderTimeDisabled"
                />

                <span
                  v-if="editingTodo.reminder_at !== null"
                  class="text-xs"
                  :class="
                    editingTodo.reminder_at < 0
                      ? 'text-red-500'
                      : editingTodo.reminder_at === 0
                        ? 'text-amber-500'
                        : 'text-gray-500'
                  "
                >
                  {{ getFromNow(editingTodo.reminder_at) }}
                </span>
              </div>

              <span class="inline-flex items-center gap-2 text-gray-600">
                <Icon name="icon-biaoqian" />
                {{ t("todo.tags") }}
              </span>
              <n-dynamic-tags v-model:value="tagList" size="small" type="primary" :max="5" />

              <template v-if="editingTodo.created_at">
                <span class="inline-flex items-center gap-2 text-gray-600">
                  <Icon name="icon-chuangjianshijian1-copy" />
                  {{ t("todo.createdAt") }}
                </span>
                <span class="inline-flex items-center text-gray-500">
                  {{ dateTimeFormat(editingTodo.created_at) }}
                  <b class="ml-3">
                    {{ t("todo.createdBy") }} {{ getFromNow(editingTodo.created_at) }}
                  </b>
                </span>
              </template>
            </div>

            <label class="block mt-5.5 mb-2 text-gray-700">{{ t("todo.note") }}</label>
            <n-input
              v-model:value="editingTodo.note"
              type="textarea"
              :placeholder="t('todo.notePlaceholder')"
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
            <n-button v-if="isDetailEdit" text type="error" @click="deleteEditingTodo">
              <template #icon>
                <Icon name="icon-shanchu" class="-mt-0.75" />
              </template>
              {{ t("todo.deleteTask") }}
            </n-button>
            <div v-else />
            <div class="flex gap-2.5">
              <n-button quaternary class="h-9! !px-3.5" @click="backToList">
                {{ t("todo.cancel") }}
              </n-button>
              <n-button type="primary" class="!h-9 !px-3.5" @click="saveEditingTodo">
                {{ isDetailCreate ? t("todo.create") : t("todo.save") }}
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
import { CheckmarkCircleOutline, RadioButtonOffOutline } from "@vicons/ionicons5";
import { computed, nextTick, ref, watch } from "vue";
import { addTodo, deleteTodo, getTodos, updateTodo, clearCompletedTodos } from "@/api";
import { SEARCH_INPUT_HEIGHT, SEARCH_WINDOW_WIDTH } from "@/constant";
import { getFromNow } from "@/utils/date";
import { dateTimeFormat } from "@/utils/date";
import PrioritySelector from "./components/PrioritySelector.vue";
import TodoItem from "./components/TodoItem.vue";
import { useTodoDomain } from "./useTodoDomain";
import { useTodoViewState } from "./useTodoViewState";
import { t } from "@/i18n";
import { useStorage } from "@vueuse/core";
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

const sortOptions = computed(() => [
  { label: t("todo.sortPriority"), value: "priority" },
  { label: t("todo.sortCreatedAt"), value: "createdAt" },
  { label: t("todo.sortDueDate"), value: "dueDate" },
]);

const searchWindow = getCurrentWindow();
const inputRef = useTemplateRef<HTMLInputElement>("todoInputRef");
const inputValue = ref(props.keyword || "");
const todos = ref<TodoItem[]>([]);
// const activeFilter = ref<TodoFilter>("all");
const activeFilter = useStorage("todoActiveFilter", "all");
// const sortType = ref<TodoSort>("priority");
const sortType = useStorage("todoSortType", "priority");
const totalCount = ref(0);
const activeCount = ref(0);
const completedCount = ref(0);
const sortTypeLabel = computed(
  () => sortOptions.value.find((item) => item.value === sortType.value)!.label,
);
const editingTodo = ref<TodoItem>({
  id: 0,
  title: "",
  completed: false,
  priority: 1,
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
  isTypingTarget,
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
const hasTodos = computed(() => totalCount.value > 0);
const filterTabs = computed(() => [
  { label: t("todo.filterAll"), value: "all" as const, count: totalCount.value },
  { label: t("todo.filterActive"), value: "active" as const, count: activeCount.value },
  { label: t("todo.filterCompleted"), value: "completed" as const, count: completedCount.value },
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
    priority: 1,
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
    const result = await getTodos(sortType.value, activeFilter.value);
    todos.value = result.todos;
    totalCount.value = result.total;
    activeCount.value = result.activeCount;
    completedCount.value = result.completedCount;
    setViewState(!totalCount.value && activeCount.value && completedCount.value ? "empty" : "list");
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
      loadTodos();
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
      priority: 1,
      title: "",
      due_date: null,
      tags: "",
      note: "",
      reminder_at: null,
    };

    if (isFastAdd) {
      item.title = title;
      item.priority = 1;
    } else {
      for (const key in editingTodo.value) {
        // @ts-ignore
        if (item[key] !== undefined) item[key] = editingTodo.value[key];
      }
    }
    const todo = await addTodo(item);
    todos.value.unshift(todo);
    inputValue.value = "";
    selectedTodoIndex.value = 0;
    setViewState("list", true);
  } catch (e) {
    console.log("e", e);
  }
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

  // Tab 切换排序方式
  if (key === "Tab" && isListState.value) {
    const currentIndex = sortOptions.value.findIndex((item) => item.value === sortType.value);
    const nextIndex = (currentIndex + 1) % sortOptions.value.length;
    sortType.value = sortOptions.value[nextIndex].value as TodoSort;
    event.preventDefault();
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
  if (isListState.value && key === "ArrowRight" && selectedTodoIndex.value >= 0) {
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
watch(sortType, loadTodos);
watch(activeFilter, loadTodos);

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
.todo-date-picker {
  ::v-deep(.v-binder-follower-content) {
    scale: 0.9;
  }
}

.todo-move,
.todo-enter-active,
.todo-leave-active {
  transition: all 0.25s ease;
}

/* 进入起点 */
.todo-enter-from {
  opacity: 0;
  transform: translateY(-8px);
}

/* 离开终点 */
.todo-leave-to {
  opacity: 0;
  transform: translateY(8px);
}

/* 离开时脱离布局流（关键） */
.todo-leave-active {
  position: absolute;
  width: 100%;
}

/* 让其他元素平滑移动（核心体验） */
.todo-move {
  transition: transform 0.25s ease;
}
</style>
