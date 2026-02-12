<template>
  <!-- style="max-height: 300px; overflow-y: auto" -->
  <n-dropdown
    ref="menuRef"
    placement="bottom-start"
    trigger="manual"
    :x="position.x"
    :y="position.y"
    :options="(menuOptions as any)"
    :show="visible"
    :on-clickoutside="handleClose"
    @select="handleSelect"
  />
</template>

<script setup lang="tsx">
import { openPath } from '@/api';
import { useContextMenuClose } from '@/composables/useContextMenuClose';
import { AppEvent } from '@/constant';
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
    label: props.item?.exclude ? '搜索排除 (✅)' : '搜索排除',
    key: 'exclude',
    icon: renderIcon('icon-paichusousuo'),
    // show: item.
    props: {
      style: props.item?.exclude ? 'color: var(--n-color-danger);font-weight: bold;' : '',
    },
  },
  {
    label: props.item?.association_directory ? '关联目录 (✅)' : '关联目录',
    key: 'correlation',
    icon: renderIcon('icon-guanlian'),
    // show: !props.item?.association_directory,
    props: {
      style: props.item?.association_directory ? 'color: var(--n-color-danger);font-weight: bold;' : '',
    },
  },
  {
    label: '打开关联目录',
    key: 'openCorrelationDir',
    icon: renderIcon('icon-wj-wjj'),
    show: !!props.item?.association_directory,
    onClick: () => openPath(props.item!.association_directory!),
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
  // {
  //   label: '设置图标',
  //   key: 'set-icon',
  //   icon: renderIcon('icon-zhongmingming'),
  // },
  {
    label: '编 辑',
    key: 'edit',
    icon: renderIcon('icon-bianji'),
    onClick: () => EventBus.emit(AppEvent.OPEN_OPERATION_CATEGORY, props.item),
  },
  {
    label: '创建子分类',
    key: 'create-sub-category',
    icon: renderIcon('icon-bianji'),
  },
  {
    label: '删 除',
    key: 'delete',
    icon: renderIcon('icon-shanchufenlei'),
  },
  {
    type: 'divider',
    key: 'd2',
  },
  {
    label: '布局展示',
    key: 'layout',
    icon: renderIcon('icon-buju'),
    children: [
      {
        label: props.item?.layout === 'grid' ? '平铺 (✅)' : '平铺',
        key: 'layout-grid',
        icon: renderIcon('icon-24gl-appsSmall'),
      },
      {
        label: props.item?.layout === 'list' ? '列表 (✅)' : '列表',
        key: 'layout-list',
        icon: renderIcon('icon-liebiao'),
      },
    ],
  },
  {
    // TODO 排序
    label: '排序方式',
    key: 'order',
    icon: renderIcon('icon-buju'),
    children: [
      {
        label: props.item?.layout === 'grid' ? '名称 (✅)' : '名称',
        key: 'order-name',
        icon: renderIcon('icon-24gl-appsSmall'),
      },
      {
        label: props.item?.layout === 'list' ? '类型 (✅)' : '类型',
        key: 'layout-list',
        icon: renderIcon('icon-liebiao'),
      },
      {
        label: props.item?.layout === 'list' ? '大小 (✅)' : '大小',
        key: 'layout-list',
        icon: renderIcon('icon-liebiao'),
      },
      {
        type: 'divider',
        key: 'd3',
      },
      {
        label: props.item?.layout === 'list' ? '升序 (✅)' : '升序',
        key: 'layout-list',
        icon: renderIcon('icon-liebiao'),
      },
      {
        label: props.item?.layout === 'list' ? '降序 (✅)' : '降序',
        key: 'layout-list',
        icon: renderIcon('icon-liebiao'),
      },
    ],
  },
]);

function handleClose() {
  visible.value = false;
}

async function handleSelect(key: string) {
  // console.log('selected key:', key);
  // TODO: 根据 key 处理不同的操作
  const findRes = menuOptions.value.find(item => item.key === key);

  if (!findRes) return handleClose();
  findRes.onClick && (await findRes.onClick());
  // switch (key) {
  //   case 'rename':
  //     EventBus.emit(AppEvent.OPEN_OPERATION_CATEGORY, props.item);
  //     break;
  //   case 'delete':
  //     EventBus.emit(AppEvent.OPEN_OPERATION_CATEGORY, props.item);
  //     break;
  //   case 'edit':
  //     EventBus.emit(AppEvent.OPEN_OPERATION_CATEGORY, props.item);
  //     break;
  //   case 'layout':
  //     EventBus.emit(AppEvent.OPEN_OPERATION_CATEGORY, props.item);
  //     break;
  //   case 'exclude':
  //     EventBus.emit(AppEvent.OPEN_OPERATION_CATEGORY, props.item);
  //     break;
  //   default:
  //     break;
  // }
  handleClose();
}

useContextMenuClose(handleClose);
</script>
