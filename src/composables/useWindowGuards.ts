export const useWindowGuards = () => {
  const isDev = import.meta.env.DEV
  if (isDev) return
  // 生产环境禁止鼠标右键
  window.addEventListener('contextmenu', e => e.preventDefault())

  window.addEventListener('keydown', e => {
    if (e.key === 'F5' || (e.ctrlKey && e.key.toLowerCase() === 'r')) {
      e.preventDefault()
    }
  })

  window.addEventListener('keydown', e => {
    if (e.ctrlKey && e.shiftKey && e.key.toLowerCase() === 'i') {
      e.preventDefault()
    }
  })
}
