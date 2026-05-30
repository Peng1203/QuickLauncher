<template>
  <n-dropdown
    placement="bottom-start"
    trigger="manual"
    :show="visible"
    :x="position.x"
    :y="position.y"
    :options="menuOptions"
    :on-clickoutside="handleClose"
    @select="handleSelect"
  />
</template>

<script setup lang="tsx">
import { AppEvent } from '@/constant';
import { t } from '@/i18n';
import { EventBus } from '@/utils/eventBus';

defineProps<{ position: { x: number; y: number } }>();

export interface MenuAction {
  label: string;
  onClick: () => void;
}

const visible = defineModel<boolean>();

function handleClose() {
  visible.value = false;
}

// 默认菜单项
const menuOptions = computed(() => [
  {
    label: t('contextMenu.newCategory'),
    key: 'add',
    icon: () => h(<i class="iconfont icon-xinjian" />),
  },
]);

function handleSelect(key: string) {
  switch (key) {
    case 'add':
      EventBus.emit(AppEvent.OPEN_OPERATION_CATEGORY);
      break;
  }

  handleClose();
}
</script>
