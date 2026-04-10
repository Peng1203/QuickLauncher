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
import { EventBus } from '@/utils/eventBus';

export interface MenuAction {
  label: string;
  onClick: () => void;
}

defineProps<{ position: { x: number; y: number } }>();
const visible = defineModel<boolean>();

function handleClose() {
  visible.value = false;
}

// 默认菜单项
const menuOptions = computed(() => [
  {
    label: '新建分类',
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
