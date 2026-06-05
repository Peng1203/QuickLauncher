import { getDaysUntil, dateTimeFormat } from "@/utils/date";
import { t } from "@/i18n";

export type TodoViewState = "empty" | "create" | "list" | "detail-create" | "detail-edit";

export function getPriorityColor(val: TodoPriority) {
  return {
    3: "#ff2d55",
    2: "#f5b301",
    1: "#16c784",
  }[val];
}

export const priorityOptions: { label: string; value: TodoPriority }[] = [
  { label: "高", value: 3 },
  { label: "中", value: 2 },
  { label: "低", value: 1 },
] as const;

export function formatDueDate(todo: TodoItem) {
  if (todo.completed) return t("todo.completed");
  if (!todo.due_date) return "";

  const days = getDaysUntil(todo.due_date);
  if (days === null) return t("todo.today");
  if (days === 0) return t("todo.today");
  if (days === 1) return t("todo.tomorrow");
  if (days > 1 && days < 7) return `${t("todo.remaining")} ${days} ${t("todo.dayUnit")}`;
  if (days >= 7) return dateTimeFormat(todo.due_date).replaceAll("-", "/");
  return `${t("todo.overdue")} ${Math.abs(days)} ${t("todo.dayUnit")}`;
}
