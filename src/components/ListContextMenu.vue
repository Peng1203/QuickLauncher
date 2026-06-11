<template>
  <n-dropdown
    placement="bottom-start"
    trigger="manual"
    :x="position.x"
    :y="position.y"
    :options="menuOptions"
    :show="visible"
    :on-clickoutside="handleClose"
    @select="handleSelect"
  />
</template>

<script setup lang="tsx">
import { storeToRefs } from 'pinia';
import { useLayoutOrderMenu } from '@/composables';
import { AppEvent } from '@/constant';
import { t } from '@/i18n';
import { useStore } from '@/store/useStore';
import { EventBus } from '@/utils/eventBus';

defineProps<{ position: { x: number; y: number } }>();

export interface MenuAction {
  label: string;
  onClick: () => void;
}

const visible = defineModel<boolean>();

const store = useStore();
const { activeCategoryItem } = storeToRefs(store);
const { layoutMenu, orderMenu, handleLayoutOrderSelect } = useLayoutOrderMenu(activeCategoryItem, {
  showLaunchCount: false,
});

function handleClose() {
  visible.value = false;
}

// 默认菜单项
const menuOptions = computed(() => {
  const menuOptions = [layoutMenu.value, orderMenu.value]

  if (!activeCategoryItem.value?.association_directory) menuOptions.unshift({
    label: t('contextMenu.newLaunchItem'),
    key: 'add',
    icon: () => h(<i class="iconfont icon-xinjian" />),
  } as any)

  return menuOptions
});

async function handleSelect(key: string) {
  switch (key) {
    case 'add':
      EventBus.emit(AppEvent.OPEN_OPERATION_LAUNCH);
      break;
    default:
      handleLayoutOrderSelect(key);
      break;
  }

  handleClose();
}
</script>
