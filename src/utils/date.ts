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
