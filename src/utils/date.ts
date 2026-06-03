import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
import "dayjs/locale/zh-cn.js";

dayjs.extend(relativeTime);

// 类型别名
type dateType = number | string | Date;

/**
 * 格式化时间的方法 YYYY-MM-DD
 */
export function dateFormat(date?: dateType, template: string = "YYYY-MM-DD"): string {
  return dayjs(date || new Date()).format(template);
}

/**
 * 格式化时间的方法 YYYY-MM-DD HH:mm:ss'
 */
export function dateTimeFormat(date?: dateType): string {
  return dayjs(date || new Date()).format("YYYY-MM-DD HH:mm:ss");
}

/**
 * 计算距离截止日期的天数
 * @param dueDate 截止日期（YYYY-MM-DD 或 Date）
 * @returns 正数=还有N天，0=今天，负数=已过期N天，null=无日期
 */
export function getDaysUntil(dueDate?: number | string | Date | null): number | null {
  if (!dueDate) return null;
  const today = dayjs().startOf("day");
  const due = dayjs(dueDate).startOf("day");
  return due.diff(today, "day");
}

export function getFromNow(date?: dateType, maxMonth = 6, lang = "zh-cn") {
  // return dayjs(date || new Date())
  //   .locale('zh-cn')
  //   .fromNow()

  const dateObj = dayjs(date || new Date()).locale(lang);
  const diffMonths = dayjs().diff(dateObj, "month");

  // 判断是否大于6个月
  return diffMonths >= maxMonth ? dateObj.format("YYYY-MM-DD") : dateObj.fromNow();
}
