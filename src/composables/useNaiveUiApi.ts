import { useMessage, useModal, useNotification, useDialog } from 'naive-ui';

export function useNaiveUiApi() {
  const message = useMessage();
  const modal = useModal();
  const notification = useNotification();
  const dialog = useDialog();

  return {
    modal,
    dialog,
    message,
    notification,
  };
}
