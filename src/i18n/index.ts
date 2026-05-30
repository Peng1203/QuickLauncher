import { createI18n } from 'vue-i18n';
import { LANG_INDEX, messages } from './lang/message';

type LangIndex = (typeof LANG_INDEX)[keyof typeof LANG_INDEX];

/**
 * 将数组格式的翻译数据转换为 vue-i18n 需要的嵌套对象格式
 * 输入: { common: { confirm: ['确认', '確認', 'Confirm', '確認'] } }
 * 输出: { 'zh-CN': { common: { confirm: '确认' } }, 'zh-HK': { ... }, ... }
 */
function buildLocaleMessages(arrayMessages: Record<string, Record<string, string[]>>) {
  const locales: Record<string, Record<string, string>> = {};

  for (const lang of Object.keys(LANG_INDEX)) {
    locales[lang] = {};
  }

  for (const [namespace, keys] of Object.entries(arrayMessages)) {
    for (const lang of Object.keys(LANG_INDEX)) {
      const idx = LANG_INDEX[lang as keyof typeof LANG_INDEX] as LangIndex;
      // @ts-ignore
      locales[lang][namespace] = {};
      for (const [key, values] of Object.entries(keys)) {
        // @ts-ignore
        locales[lang][namespace][key] = values[idx] ?? values[0];
      }
    }
  }

  // 转换为 vue-i18n 的嵌套格式
  const result: Record<string, any> = {};
  for (const lang of Object.keys(LANG_INDEX)) {
    result[lang] = {};
    for (const [namespace, keys] of Object.entries(locales[lang])) {
      result[lang][namespace] = keys;
    }
  }

  return result;
}

const localeMessages = buildLocaleMessages(messages as any);

export const i18n = createI18n({
  legacy: false,
  locale: 'zh-CN',
  fallbackLocale: 'zh-CN',
  messages: localeMessages,
});

// export const t = i18n.global.t;
export const t = (key: NestedKeys<typeof messages>) => i18n.global.t(key);

// type Messages = typeof messages;
// type DeepKey<T> = {
//   [K in keyof T & string]: T[K] extends object ? `${K}` | `${K}.${DeepKey<T[K]>}` : `${K}`;
// }[keyof T & string];
// type DeepValue<T, K extends string> = K extends `${infer P}.${infer R}`
//   ? P extends keyof T
//     ? DeepValue<T[P], R>
//     : never
//   : K extends keyof T
//     ? T[K]
//     : never;
// export function t<K extends DeepKey<Messages>>(key: K): DeepValue<Messages, K> {
//   return i18n.global.t(key) as any;
// }

export { LANG_INDEX };
