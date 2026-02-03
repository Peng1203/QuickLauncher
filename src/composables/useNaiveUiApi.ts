import { useMessage, useModal, useNotification } from 'naive-ui';

export function useNaiveUiApi() {
  const message = useMessage();
  const modal = useModal();
  const notification = useNotification();

  return {
    message,
    modal,
    notification,
  };
}
