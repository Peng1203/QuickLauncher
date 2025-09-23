export const getShortcutKey = (
  e: KeyboardEvent,
  originValue: string = '',
  preventDefault: boolean = false
) => {
  if (preventDefault) e.preventDefault()

  const { code, key, keyCode, shiftKey, ctrlKey, metaKey, altKey } = e

  if (code === 'Escape' && keyCode === 27) return originValue
  if (code === 'Backspace' && keyCode === 8) return ''

  console.log(`%c e ----`, 'color: #fff;background-color: #000;font-size: 18px', e)

  const keys = []

  const CTRL_KEY = 'Ctrl + '
  const ALT_KEY = 'Alt + '
  const SHIFT_KEY = 'Shift + '
  // 添加组合按键
  if (ctrlKey && keyCode !== 17) keys.push(CTRL_KEY)
  if (altKey && keyCode !== 18) keys.push(ALT_KEY)
  if (shiftKey && keyCode !== 16) keys.push(SHIFT_KEY)
  // if (metaKey) keys.push('Win + ')

  // 添加多个组合按键
  if (ctrlKey && keyCode === 17) keys.push(CTRL_KEY)
  if (altKey && keyCode === 18) keys.push(ALT_KEY)
  if (shiftKey && keyCode === 16) keys.push(SHIFT_KEY)

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
