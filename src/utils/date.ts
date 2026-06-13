import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
import "dayjs/locale/zh-cn.js";
import "dayjs/locale/zh-hk.js";
import "dayjs/locale/en.js";
import "dayjs/locale/ja.js";

dayjs.extend(relativeTime);

// 类型别名
type dateType = number | string | Date;

let currentLang: LanguageType = "zh-CN";

// export type Lang = "zh-cn" | "zh-hk" | "en" | "ja";

export function setDayjsLang(lang: LanguageType) {
  let newLang = lang.toLocaleUpperCase();
  currentLang = lang;
  dayjs.locale(newLang);
}

export function getDayjsLang() {
  return currentLang;
}

export function dateFormat(date?: dateType, template = "YYYY-MM-DD"): string {
  return dayjs(date || new Date()).format(template);
}

export function dateTimeFormat(date?: dateType): string {
  return dayjs(date || new Date()).format("YYYY-MM-DD HH:mm:ss");
}

export function getDaysUntil(dueDate?: dateType | null): number | null {
  if (!dueDate) return null;

  const today = dayjs().startOf("day");
  const due = dayjs(dueDate).startOf("day");

  return due.diff(today, "day");
}

export function getFromNow(date?: dateType, maxMonth = 6) {
  const dateObj = dayjs(date || new Date());
  const diffMonths = dayjs().diff(dateObj, "month");

  return diffMonths >= maxMonth ? dateObj.format("YYYY-MM-DD") : dateObj.fromNow();
}

/**
 * 格式化提醒时间的相对展示
 * 规则：
 * - 已过期 → "已提醒"
 * - 1小时内 → "xx分钟后"
 * - 6小时内 → "xx小时后"
 * - 当天超过6小时 → "今天 HH:MM"
 * - 明天 → "明天 HH:MM"
 * - 2~6天 → "还剩 x 天 HH:MM"
 * - 7天及以上 → 完整日期
 * - 已过期（昨天及更早）→ "已过期 x 天 HH:MM"
 */
export function formatRelativeReminder(reminderAt: number, t: (key: any) => string): string {
  const now = dayjs();
  const target = dayjs(reminderAt);
  const diffMs = reminderAt - now.valueOf();

  if (diffMs <= 0) return t("todo.reminded");

  const diffMin = Math.floor(diffMs / 60000);
  const diffHour = Math.floor(diffMs / 3600000);
  const timeStr = target.format("HH:mm");

  if (diffMin < 60) return `${diffMin}${t("todo.minutesLater")}`;
  if (diffHour < 6) return `${diffHour}${t("todo.hoursLater")}`;

  const diffDays = target.startOf("day").diff(now.startOf("day"), "day");

  if (diffDays === 0) return `${t("todo.today")} ${timeStr}`;
  if (diffDays === 1) return `${t("todo.tomorrow")} ${timeStr}`;
  if (diffDays > 1 && diffDays < 7)
    return `${t("todo.remaining")} ${diffDays} ${t("todo.dayUnit")} ${timeStr}`;
  if (diffDays >= 7) return target.format("YYYY/MM/DD HH:mm");
  return `${t("todo.overdue")} ${Math.abs(diffDays)} ${t("todo.dayUnit")} ${timeStr}`;
}
