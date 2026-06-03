import type { TodoViewState } from "./index";

export const useTodoViewState = (loadTodos: () => void) => {
  const viewState = ref<TodoViewState>();
  const previousState = ref<TodoViewState>("empty");
  const stateOrder: TodoViewState[] = ["empty", "create", "list", "detail-create", "detail-edit"];

  const isEmptyState = computed(() => viewState.value === "empty");
  const isCreateState = computed(() => viewState.value === "create");
  const isListState = computed(() => viewState.value === "list");
  const isDetailCreate = computed(() => viewState.value === "detail-create");
  const isDetailEdit = computed(() => viewState.value === "detail-edit");
  const isDetailState = computed(() => isDetailCreate.value || isDetailEdit.value);
  const isEditing = computed(() => isDetailState.value);

  const direction = computed(() => {
    const from = stateOrder.indexOf(previousState.value);
    const to = stateOrder.indexOf(viewState.value ?? "empty");

    return to > from ? "left" : "right";
  });

  function setViewState(next: TodoViewState, refreshDate: boolean = false) {
    previousState.value = viewState.value ?? "empty";
    if (refreshDate) loadTodos();
    viewState.value = next;
  }

  return {
    viewState,
    previousState,
    stateOrder,
    isEmptyState,
    isCreateState,
    isListState,
    isDetailCreate,
    isDetailEdit,
    isDetailState,
    isEditing,
    direction,
    setViewState,
  };
};
