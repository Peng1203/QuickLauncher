import { isRegistered, unregister } from '@tauri-apps/plugin-global-shortcut'

const CTRL = 'Ctrl'
const ALT = 'Alt'
const SHIFT = 'Shift'
const WIN = 'Super'

/**
 * 获取输入的快捷键
 * @author Peng
 *
 * @param {KeyboardEvent} e
 * @param {string} [originValue='']
 * @param {boolean} [preventDefault=false]
 * @returns {*}
 */
export const getShortcutKey = (
  e: KeyboardEvent,
  originValue: string = '',
  preventDefault: boolean = false
) => {
  if (preventDefault) e.preventDefault()

  const { code, key, keyCode, shiftKey, ctrlKey, metaKey, altKey } = e

  if (code === 'Escape' && keyCode === 27) return originValue
  if (code === 'Backspace' && keyCode === 8) return ''

  const keys = []

  const CTRL_KEY = `${CTRL} + `
  const ALT_KEY = `${ALT} + `
  const SHIFT_KEY = `${SHIFT} + `
  const WIN_KEY = `${WIN} + `

  // 添加组合按键
  if (ctrlKey && keyCode !== 17) keys.push(CTRL_KEY)
  if (altKey && keyCode !== 18) keys.push(ALT_KEY)
  if (shiftKey && keyCode !== 16) keys.push(SHIFT_KEY)
  if (metaKey && keyCode !== 91) keys.push(WIN_KEY)

  // 添加多个组合按键
  if (ctrlKey && keyCode === 17) keys.push(CTRL_KEY)
  if (altKey && keyCode === 18) keys.push(ALT_KEY)
  if (shiftKey && keyCode === 16) keys.push(SHIFT_KEY)
  if (metaKey && keyCode === 91) keys.push(WIN_KEY)

  // keys.push(code)
  let flag = false
  let singleKey = ''

  // 字母A-Z处理
  if (keyCode >= 65 && keyCode <= 90) {
    flag = true
    singleKey = key.toUpperCase()
  }
  // 处理小键盘 数字输入
  if (keyCode >= 96 && keyCode <= 105) {
    flag = true
    singleKey = code
  }
  // 特殊键映射处理
  switch (keyCode) {
    // Space 空格
    case 32:
      singleKey = 'Space'
      break
    // 方向键过滤
    case 37:
    case 38:
    case 39:
    case 40:
      singleKey = code.replace('Arrow', '')
      break
    // 跳过组合键
    case 16:
    case 17:
    case 18:
    case 91:
      break
    // + - 号
    case 107:
    case 109:
      singleKey = code
      break
    default:
      !flag && (singleKey = key)
      break
  }

  keys.push(singleKey)

  // const order = { [ALT_KEY]: 1, [CTRL_KEY]: 2, [SHIFT_KEY]: 3 }
  // @ts-ignore
  // keys.sort((a, b) => order[a] - order[b])

  return keys.join('')
}

/**
 * 校验快捷键是否可以被注册
 * @author Peng
 *
 * @async
 * @param {string} shortcutKey
 * @param {?string} [oldShortcutKey]
 * @returns {Promise<boolean>}
 */
export const checkShortcutKey = async (
  shortcutKey: string,
  oldShortcutKey?: string
): Promise<boolean> => {
  try {
    if (oldShortcutKey && shortcutKey === oldShortcutKey) return true
    let flag = false
    // 检验快捷键是否在全局注册过
    const isReg = await isRegistered(shortcutKey)
    flag = !isReg
    // TODO 校验快捷键是否在应用内其他地方被注册
    return flag
  } catch (e) {
    console.log('e', e)
    return false
  }
}

/**
 * 校验快捷键有效性
 * @author Peng
 *
 * @param {string} shortcutKey
 */
export const checkShortcutKeyComplete = (shortcutKey: string): boolean => {
  if (!shortcutKey.trim()) return false

  const keys = shortcutKey
    .split('+')
    .map(k => k.trim())
    .filter(Boolean)
  if (keys.length === 0) return false

  let flag = false
  const modifierKeys = [CTRL, ALT, SHIFT, WIN]
  for (const key of keys) {
    if (modifierKeys.includes(key)) continue

    // 当存在组合修饰键以外的按键 则为合法
    if (key !== CTRL && key !== ALT && key !== SHIFT && key !== WIN) flag = true
  }

  return flag
}

export const unRegisterShortcutKey = async (shortcutKey: string) => {
  if (!shortcutKey.trim()) return
  const isReg = await isRegistered(shortcutKey)
  isReg && (await unregister(shortcutKey))
}
