import { useMessage, useModal } from 'naive-ui';

export function useNaiveUiApi() {
  const message = useMessage();
  const modal = useModal();

  return {
    message,
    modal,
  };
}
