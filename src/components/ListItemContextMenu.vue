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
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { useMessage } from 'naive-ui';
import { openRevealManager, runLaunchAsAdmin } from '@/api';
import { AppEvent } from '@/constant';
import { t } from '@/i18n';
import { EventBus } from '@/utils/eventBus';

export interface MenuAction {
  label: string;
  onClick: () => void;
  itemVisible?: () => void;
}

// visible: boolean
const props = withDefaults(
  defineProps<{
    position: { x: number; y: number };
    item: LaunchItem;
    itemPath: string;
    itemName: string;
    categoryItem?: CategoryItem | null;
    type?: 'LaunchList' | 'SearchLaunchList';
    liStyle?: string;
  }>(),
  {
    type: 'LaunchList',
    categoryItem: null,
  },
);

const emit = defineEmits(['close', 'rename', 'delete']);

const visible = defineModel<boolean>();

const message = useMessage();

function renderIcon(icon: string) {
  return () => h(<i class={`iconfont ${icon}`} />);
}

const isLaunchList = computed(() => props.type === 'LaunchList');
const isSearchLaunchList = computed(() => props.type === 'SearchLaunchList');

// 默认菜单项
const menuOptions = computed(() => {
  const menus = [
    {
      label: t('common.adminRun'),
      key: 'runAsAdmin',
      icon: renderIcon('icon-guanliyuan_jiaoseguanli'),
      itemVisible: ['exe'].includes(props.item?.extension || ''),
    },
    {
      label: t('common.openInExplorer'),
      key: 'openRevalPath',
      icon: renderIcon('icon-dakaisuozaiwenjianjia'),
      itemVisible: ['file', 'directory'].includes(props.item.type),
    },
    {
      label: t('common.copyPath'),
      key: 'copyPath',
      icon: renderIcon('icon-fuzhilujing'),
    },
    {
      label: t('common.rename'),
      key: 'rename',
      icon: renderIcon('icon-zhongmingming'),
      itemVisible: isLaunchList.value,
    },
    {
      label: t('common.delete'),
      key: 'delete',
      icon: renderIcon('icon-shanchufenlei'),
      itemVisible: isLaunchList.value,
    },
    {
      label: t('common.edit'),
      key: 'edit',
      icon: renderIcon('icon-bianji'),
      itemVisible: props.categoryItem === null ? true : !props.categoryItem?.association_directory,
    },
    {
      label: t('common.locate'),
      key: 'position',
      icon: renderIcon('icon-address'),
      itemVisible: isSearchLaunchList.value,
    },
    {
      label: t('common.increasePriority'),
      key: 'increasePriority',
      icon: renderIcon('icon-youxianji1'),
      itemVisible: isSearchLaunchList.value,
    },
  ];

  return menus.filter(item => item.itemVisible !== false);
});

// 自动监听点击窗口其他地方关闭菜单
function handleClose() {
  visible.value = false;
}

async function handleSelect(key: string) {
  switch (key) {
    case 'runAsAdmin':
      runLaunchAsAdmin(props.item.id);
      break;
    case 'openRevalPath':
      openRevealManager(props.item.path);
      break;
    case 'copyPath':
      await writeText(props.itemPath);
      message.success(t('common.copySuccess'));
      break;
    case 'rename':
      emit('rename');
      break;
    case 'delete':
      emit('delete');
      break;
    case 'edit':
      EventBus.emit(AppEvent.OPEN_OPERATION_LAUNCH, props.item);
      break;
    case 'position':
      // TODO 定位
      EventBus.emit(AppEvent.LAUNCH_POSITION, props.item);
      break;
    case 'increasePriority':
      EventBus.emit(AppEvent.INCREASE_PRIORITY, props.item);
      break;
  }

  handleClose();
}
</script>
