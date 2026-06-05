import { getDaysUntil } from "@/utils/date";
import { t } from "@/i18n";

export const useTodoDomain = (editingTodo: Ref<TodoItem>) => {
  const HOUR_MS = 60 * 60 * 1000;
  const DAY_MS = HOUR_MS * 24;

  const dueDateshortcuts = computed(() => ({
    [t("todo.after3h")]: () => Date.now() + HOUR_MS * 3,
    [t("todo.after3d")]: () => Date.now() + DAY_MS * 3,
    [t("todo.after1w")]: () => Date.now() + DAY_MS * 7,
    "114514😋": () => Date.now() + DAY_MS * 114514,
  }));
  const reminderAtshortcuts = computed(() => {
    let obj = {};
    if (editingTodo.value.due_date) {
      const dueDate = editingTodo.value.due_date;
      obj = {
        [t("todo.before1h")]: () => {
          const ts = dueDate - HOUR_MS * 1;
          if (ts > Date.now()) return ts;
          else {
            return null;
          }
        },
        [t("todo.before1d")]: () => {
          const ts = dueDate - DAY_MS * 1;
          if (ts > Date.now()) return ts;
          else {
            return null;
          }
        },
      };
      return obj;
    } else {
      obj = {
        [t("todo.after1h")]: () => Date.now() + HOUR_MS,
        [t("todo.after3h")]: () => Date.now() + HOUR_MS * 3,
        [t("todo.after3d")]: () => Date.now() + DAY_MS * 3,
        [t("todo.after1w")]: () => Date.now() + DAY_MS * 7,
      };
    }
    return obj;
  });
  // 不能设置现在之前的提醒日期 如果存在截止日期 也不能设置超过截止日期的提醒日期
  function reminderDateDisabled(ts: number) {
    const now = Date.now();

    const due = editingTodo.value?.due_date;

    // 归一化到当天 00:00（避免时间影响日期选择）
    const target = new Date(ts).setHours(0, 0, 0, 0);

    const nowDay = new Date(now).setHours(0, 0, 0, 0);

    // 1. 不能早于今天
    if (target < nowDay) return true;

    // 2. 如果有 due_date，不能超过截止日期所在的天
    if (due) {
      const dueDay = new Date(due).setHours(0, 0, 0, 0);
      if (target > dueDay) return true;
    }

    return false;
  }

  function reminderTimeDisabled(current: number) {
    const now = new Date();
    const due = editingTodo.value?.due_date ? new Date(editingTodo.value.due_date) : null;

    const selectedDate = new Date(current);

    const isSameDay =
      selectedDate.getFullYear() === now.getFullYear() &&
      selectedDate.getMonth() === now.getMonth() &&
      selectedDate.getDate() === now.getDate();

    const isDueDay = due
      ? selectedDate.getFullYear() === due.getFullYear() &&
        selectedDate.getMonth() === due.getMonth() &&
        selectedDate.getDate() === due.getDate()
      : false;

    return {
      // 🚫 小时限制
      isHourDisabled: (hour: number) => {
        // 今天：不能选过去小时
        if (isSameDay && hour < now.getHours()) return true;

        // due_date：不能超过结束小时
        if (due && isDueDay && hour > due.getHours()) return true;

        return false;
      },

      // 🚫 分钟限制
      isMinuteDisabled: (minute: number, hour: number | null) => {
        if (hour === null) return false;

        // 今天同小时：不能选过去分钟
        if (isSameDay && hour === now.getHours() && minute < now.getMinutes()) {
          return true;
        }

        // due 同小时：不能超过
        if (due && isDueDay && hour === due.getHours() && minute > due.getMinutes()) {
          return true;
        }

        return false;
      },

      // 🚫 秒限制
      isSecondDisabled: (second: number, minute: number | null, hour: number | null) => {
        if (hour === null || minute === null) return false;

        // 今天同分钟
        if (
          isSameDay &&
          hour === now.getHours() &&
          minute === now.getMinutes() &&
          second < now.getSeconds()
        ) {
          return true;
        }

        // due 同分钟
        if (
          due &&
          isDueDay &&
          hour === due.getHours() &&
          minute === due.getMinutes() &&
          second > due.getSeconds()
        ) {
          return true;
        }

        return false;
      },
    };
  }

  function isTypingTarget(el: EventTarget | null) {
    if (!(el instanceof HTMLElement)) return false;

    const tag = el.tagName;
    return tag === "INPUT" || tag === "TEXTAREA" || el.isContentEditable;
  }

  function getPriorityColor(val: TodoPriority) {
    return {
      3: "#ff2d55",
      2: "#f5b301",
      1: "#16c784",
    }[val];
  }

  const dueDateDays = computed(() => getDaysUntil(editingTodo.value.due_date));
  const dueDateDaysLabel = computed(() => {
    const days = dueDateDays.value;
    if (days === null) return "";
    if (days < 0) return `${t("todo.overdue")} ${Math.abs(days)} ${t("todo.dayUnit")}`;
    if (days === 0) return t("todo.dueToday");
    if (days === 1) return t("todo.dueTomorrow");
    return `${t("todo.remaining")} ${days} ${t("todo.dayUnit")}`;
  });

  const renderTags = (tagVal: string | null) => {
    if (!tagVal) return [];
    return tagVal.split(",");
  };
  const tagList = computed({
    get: () => (editingTodo.value?.tags || "").split(",").filter((item) => item),
    set: (val) => (editingTodo.value.tags = val.join()),
  });

  return {
    dueDateshortcuts,
    reminderAtshortcuts,
    reminderDateDisabled,
    reminderTimeDisabled,
    getPriorityColor,
    dueDateDays,
    dueDateDaysLabel,
    renderTags,
    tagList,
    isTypingTarget,
  };
};
