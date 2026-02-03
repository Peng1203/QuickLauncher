<template>
  <n-dropdown
    ref="menuRef"
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
import { WineOutline as WineIcon } from '@vicons/ionicons5';
import { NIcon } from 'naive-ui';
import { ref } from 'vue';
import { AppEvent, MENU_WIDTH } from '@/constant';
import { EventBus } from '@/utils/eventBus';

const props = defineProps<{
  position: { x: number; y: number };
  item: CategoryItem;
}>();

const visible = defineModel<boolean>();

function renderIcon(icon: string) {
  return () => h(<i class={`iconfont ${icon}`} />);
}

// 默认菜单项
const menuOptions = computed(() => [
  {
    label: '搜索排除',
    key: 'exclude',
    icon: renderIcon('icon-paichusousuo'),
    // show: item.
  },
  {
    label: '关联目录',
    key: 'correlation',
    icon: renderIcon('icon-guanlian'),
  },
  {
    type: 'divider',
    key: 'd1',
  },
  {
    label: '重命名',
    key: 'rename',
    icon: renderIcon('icon-zhongmingming'),
  },
  {
    label: '删除分类',
    key: 'delete',
    icon: renderIcon('icon-shanchufenlei'),
  },
  {
    label: '编辑分类',
    key: 'edit',
    icon: renderIcon('icon-bianji'),
  },
  {
    type: 'divider',
    key: 'd1',
  },
  {
    label: '布局展示',
    key: 'layout',
    icon: renderIcon('icon-buju'),
    children: [
      {
        label: '平铺',
        key: 'layout-grid',
        icon: renderIcon('icon-24gl-appsSmall'),
      },
      {
        label: '列表',
        key: 'layout-list',
        icon: renderIcon('icon-liebiao'),
      },
    ],
  },
]);

function handleClose() {
  visible.value = false;
}

function handleSelect(key: string) {
  // console.log('selected key:', key);
  switch (key) {
    case 'rename':
      EventBus.emit(AppEvent.OPEN_OPERATION_CATEGORY, props.item);
      break;
    case 'delete':
      EventBus.emit(AppEvent.OPEN_OPERATION_CATEGORY, props.item);
      break;
    case 'edit':
      EventBus.emit(AppEvent.OPEN_OPERATION_CATEGORY, props.item);
      break;
    case 'layout':
      EventBus.emit(AppEvent.OPEN_OPERATION_CATEGORY, props.item);
      break;
    default:
      break;
  }
  handleClose();
}
</script>
