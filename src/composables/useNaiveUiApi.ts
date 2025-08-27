import { useMessage, useModal } from 'naive-ui'

export const useNaiveUiApi = () => {
  const message = useMessage()
  const modal = useModal()

  return {
    message,
    modal,
  }
}
