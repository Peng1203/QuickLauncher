import { getDaysUntil, dateTimeFormat } from "@/utils/date";

export const useTodoDomain = (editingTodo: Ref<TodoItem>) => {
  const HOUR_MS = 60 * 60 * 1000;
  const DAY_MS = HOUR_MS * 24;

  const dueDateshortcuts = {
    "3小时后": () => Date.now() + HOUR_MS * 3,
    "3天后": () => Date.now() + DAY_MS * 3,
    一周后: () => Date.now() + DAY_MS * 7,
    "114514😋": () => Date.now() + DAY_MS * 114514,
  };
  const reminderAtshortcuts = computed(() => {
    let obj = {};
    if (editingTodo.value.due_date) {
      const dueDate = editingTodo.value.due_date;
      obj = {
        截止前1小时: () => {
          const t = dueDate - HOUR_MS * 1;
          if (t > Date.now()) return t;
          else {
            return null;
          }
        },
        // 截止前3小时: () => {
        //   const t = dueDate - HOUR_MS * 3;
        //   if (t > Date.now()) return t;
        //   else {
        //     message.warning("提醒日期不能大于截止日期");
        //     return null;
        //   }
        // },
        截止前1天: () => {
          const t = dueDate - DAY_MS * 1;
          if (t > Date.now()) return t;
          else {
            return null;
          }
        },
        // 截止前3天: () => {
        //   const t = dueDate - DAY_MS * 3;
        //   if (t > Date.now()) return t;
        //   else {
        //     message.warning("提醒日期不能大于截止日期");
        //     return null;
        //   }
        // },
      };
      return obj;
    } else {
      obj = {
        "1小时后": () => Date.now() + HOUR_MS,
        "3小时后": () => Date.now() + HOUR_MS * 3,
        "3天后": () => Date.now() + DAY_MS * 3,
        一周后: () => Date.now() + DAY_MS * 7,
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

  function getPriorityColor(val: TodoPriority) {
    return {
      high: "#ff2d55",
      medium: "#f5b301",
      low: "#16c784",
    }[val];
  }

  function formatDueDate(todo: TodoItem) {
    if (todo.completed) return "已完成";
    if (!todo.due_date) return "";

    const days = getDaysUntil(todo.due_date);
    if (days === null) return "今天";
    if (days === 0) return "今天";
    if (days === 1) return "明天";
    if (days > 1 && days < 7) return `还剩 ${days} 天`;
    if (days >= 7) return dateTimeFormat(todo.due_date).replaceAll("-", "/");
    return `已过期 ${Math.abs(days)} 天`;
  }

  const dueDateDays = computed(() => getDaysUntil(editingTodo.value.due_date));
  const dueDateDaysLabel = computed(() => {
    const days = dueDateDays.value;
    if (days === null) return "";
    if (days < 0) return `已过期 ${Math.abs(days)} 天`;
    if (days === 0) return "今天截止";
    if (days === 1) return "明天截止";
    return `还剩 ${days} 天`;
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
    formatDueDate,
    dueDateDays,
    dueDateDaysLabel,
    renderTags,
    tagList,
  };
};
