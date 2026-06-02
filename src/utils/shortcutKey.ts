import {
  register as _register,
  isRegistered,
  unregister,
} from "@tauri-apps/plugin-global-shortcut";
import { t } from "@/i18n";

const CTRL = "Ctrl";
const ALT = "Alt";
const SHIFT = "Shift";
const WIN = "⌘";

// 系统 / 浏览器 / IDE 高频保留快捷键（可持续扩展）
const SYSTEM_HOTKEYS = new Set([
  // 浏览器
  "Ctrl + L",
  "Ctrl + T",
  "Ctrl + W",
  "Ctrl + N",
  "Ctrl + R",
  "Ctrl + Shift + R",
  "Ctrl + Tab",

  // 编辑常用
  "Ctrl + C",
  "Ctrl + V",
  "Ctrl + X",
  "Ctrl + A",
  "Ctrl + Z",
  "Ctrl + Y",
  "Ctrl + S",
  "Ctrl + F",
  "Ctrl + P",

  // Windows 系统
  "Alt + Tab",
  "Alt + F4",
  "Win + D",
  "Win + L",

  // mac（你这里 WIN 实际是 ⌘）
  "⌘ + Space",
  "⌘ + Q",
  "⌘ + W",
  "⌘ + Tab",
  "⌘ + C",
  "⌘ + V",
  "⌘ + X",
  "⌘ + Z",
]);

/**
 * 获取输入的快捷键
 * @author Peng
 *
 * @param {KeyboardEvent} e
 * @param {string} [originValue]
 * @param {boolean} [preventDefault]
 * @returns {*}
 */
export function getShortcutKey(
  e: KeyboardEvent,
  originValue: string = "",
  preventDefault: boolean = false,
) {
  if (preventDefault) e.preventDefault();

  const { code, key, keyCode, shiftKey, ctrlKey, metaKey, altKey } = e;

  if (code === "Escape" && keyCode === 27) return originValue;
  if (code === "Backspace" && keyCode === 8) return "";

  const keys = [];

  const CTRL_KEY = `${CTRL} + `;
  const ALT_KEY = `${ALT} + `;
  const SHIFT_KEY = `${SHIFT} + `;
  const WIN_KEY = `${WIN} + `;

  // 添加组合按键
  if (ctrlKey && keyCode !== 17) keys.push(CTRL_KEY);
  if (altKey && keyCode !== 18) keys.push(ALT_KEY);
  if (shiftKey && keyCode !== 16) keys.push(SHIFT_KEY);
  if (metaKey && keyCode !== 91) keys.push(WIN_KEY);

  // 添加多个组合按键
  if (ctrlKey && keyCode === 17) keys.push(CTRL_KEY);
  if (altKey && keyCode === 18) keys.push(ALT_KEY);
  if (shiftKey && keyCode === 16) keys.push(SHIFT_KEY);
  if (metaKey && keyCode === 91) keys.push(WIN_KEY);

  // keys.push(code)
  let flag = false;
  let singleKey = "";

  // 字母A-Z处理
  if (keyCode >= 65 && keyCode <= 90) {
    flag = true;
    singleKey = key.toUpperCase();
  }
  // 处理小键盘 数字输入
  if (keyCode >= 96 && keyCode <= 105) {
    flag = true;
    singleKey = code;
  }
  // 特殊键映射处理
  switch (keyCode) {
    // Space 空格
    case 32:
      singleKey = "Space";
      break;
    // 方向键过滤
    case 37:
    case 38:
    case 39:
    case 40:
      singleKey = code.replace("Arrow", "");
      break;
    // 跳过组合键
    case 16:
    case 17:
    case 18:
    case 91:
      break;
    // + - 号
    case 107:
    case 109:
      singleKey = code;
      break;
    default:
      if (!flag) singleKey = key;
      // !flag && (singleKey = key);
      break;
  }

  keys.push(singleKey);

  // const order = { [ALT_KEY]: 1, [CTRL_KEY]: 2, [SHIFT_KEY]: 3 }
  // @ts-ignore
  // keys.sort((a, b) => order[a] - order[b])

  return keys.join("");
}

/**
 * 校验快捷键是否可以被注册
 * @author Peng
 *
 * @async
 * @param {string} shortcutKey
 * @param {?string} [oldShortcutKey]
 * @returns {Promise<{checked: boolean, message: string}>}
 */
export async function checkShortcutKey(
  shortcutKey: string,
  oldShortcutKey?: string,
): Promise<{ checked: boolean; message: string }> {
  try {
    if (oldShortcutKey && shortcutKey === oldShortcutKey) return { checked: true, message: "" };
    let flag = false;
    // 系统快捷键校验
    const isSystemHotkey = checkSystemHotkey(shortcutKey);
    if (!isSystemHotkey) return { checked: false, message: t("shortcutKeyInput.reserved") };

    // 检验快捷键是否在全局注册过
    const isReg = await isRegistered(shortcutKey);
    flag = !isReg;

    return { checked: flag, message: flag ? "" : t("shortcutKeyInput.occupied") };
  } catch (e) {
    console.log("e", e);
    return { checked: false, message: e as string };
  }
}

/**
 * 校验快捷键有效性
 * @author Peng
 *
 * @param {string} shortcutKey
 */
export function checkShortcutKeyComplete(shortcutKey: string): boolean {
  if (!shortcutKey.trim()) return false;

  const keys = shortcutKey
    .split("+")
    .map((k) => k.trim())
    .filter(Boolean);
  if (keys.length === 0) return false;

  let flag = false;
  const modifierKeys = [CTRL, ALT, SHIFT, WIN];
  for (const key of keys) {
    if (modifierKeys.includes(key)) continue;

    // 当存在组合修饰键以外的按键 则为合法
    if (key !== CTRL && key !== ALT && key !== SHIFT && key !== WIN) flag = true;
  }

  return flag;
}

export async function unRegisterShortcutKey(shortcutKey: string) {
  if (!shortcutKey.trim()) return;
  const isReg = await isRegistered(shortcutKey);
  if (isReg) await unregister(shortcutKey);
}

export async function register(shortcutKey: string, cb: () => any) {
  await _register(shortcutKey, async (e) => {
    if (e.state === "Released") {
      cb();
    }
  });
}

/**
 * 校验是否为系统常用快捷键
 * 命中则返回 false + console 提示
 */
export function checkSystemHotkey(shortcutKey: string): boolean {
  if (!shortcutKey) return true;

  const normalized = shortcutKey.trim();
  console.log(
    `%c shortcutKey ----`,
    "color: #fff;background-color: #000;font-size: 18px",
    shortcutKey,
  );
  console.log(
    `%c normalized ----`,
    "color: #fff;background-color: #000;font-size: 18px",
    normalized,
  );
  if (SYSTEM_HOTKEYS.has(normalized)) return false;

  return true;
}
