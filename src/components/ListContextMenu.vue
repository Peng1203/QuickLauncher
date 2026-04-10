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
import { updateCategory } from '@/api';
// import { useContextMenuClose } from '@/composables/useContextMenuClose';
import { AppEvent } from '@/constant';
import { useStore } from '@/store/useStore';
import { EventBus } from '@/utils/eventBus';

export interface MenuAction {
  label: string;
  onClick: () => void;
}

defineProps<{ position: { x: number; y: number } }>();
const visible = defineModel<boolean>();

const store = useStore();
const { activeCategoryItem } = storeToRefs(store);

function handleClose() {
  visible.value = false;
}

function renderIcon(icon: string) {
  return () => h(<i class={`iconfont ${icon}`} />);
}

// 默认菜单项
const menuOptions = computed(() => [
  {
    label: '新建启动项',
    key: 'add',
    icon: () => h(<i class="iconfont icon-xinjian" />),
  },
  {
    label: '布局展示',
    key: 'layout',
    icon: renderIcon('icon-buju'),
    children: [
      {
        label: activeCategoryItem.value?.layout === 'grid' ? '平铺 (✅)' : '平铺',
        key: 'layout-grid',
        props: {
          style: activeCategoryItem.value?.layout === 'grid' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
        },
        icon: renderIcon('icon-24gl-appsSmall'),
      },
      {
        label: activeCategoryItem.value?.layout === 'list' ? '列表 (✅)' : '列表',
        key: 'layout-list',
        props: {
          style: activeCategoryItem.value?.layout === 'list' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
        },
        icon: renderIcon('icon-liebiao'),
      },
    ],
  },
  {
    label: '排序方式',
    key: 'order',
    icon: renderIcon('icon-paixufangshi'),
    children: [
      {
        label: activeCategoryItem.value?.sort_by === 'name' ? '名称 (✅)' : '名称',
        key: 'order-name',
        props: {
          style: activeCategoryItem.value?.sort_by === 'name' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
        },
        icon: renderIcon('icon-mingchengpaixu'),
      },
      {
        label: activeCategoryItem.value?.sort_by === 'type' ? '类型 (✅)' : '类型',
        key: 'order-type',
        props: {
          style: activeCategoryItem.value?.sort_by === 'type' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
        },
        icon: renderIcon('icon-anleixingpaixu'),
      },
      {
        label: activeCategoryItem.value?.sort_by === 'time' ? '日期 (✅)' : '日期',
        key: 'order-time',
        props: {
          style: activeCategoryItem.value?.sort_by === 'time' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
        },
        icon: renderIcon('icon-anchuangjianshijianpaixu'),
      },
      {
        label: activeCategoryItem.value?.sort_by === 'order' ? '搜索优先级 (✅)' : '搜索优先级',
        key: 'order-index',
        props: {
          style: activeCategoryItem.value?.sort_by === 'order' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
        },
        icon: renderIcon('icon-youxianji'),
      },
      // {
      //   label: props.item?.layout === 'list' ? '大小 (✅)' : '大小',
      //   key: 'layout-list',
      //   icon: renderIcon('icon-liebiao'),
      // },
      {
        type: 'divider',
        key: 'd3',
      },
      {
        label: activeCategoryItem.value?.sort_order === 'asc' ? '升序 (✅)' : '升序',
        key: 'sort-asc',
        props: {
          style:
            activeCategoryItem.value?.sort_order === 'asc' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
        },
        icon: renderIcon('icon-shengxu'),
      },
      {
        label: activeCategoryItem.value?.sort_order === 'desc' ? '降序 (✅)' : '降序',
        key: 'sort-desc',
        props: {
          style:
            activeCategoryItem.value?.sort_order === 'desc' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
        },
        icon: renderIcon('icon-jiangxu'),
      },
    ],
  },
]);

async function handleSelect(key: string) {
  switch (key) {
    case 'add':
      EventBus.emit(AppEvent.OPEN_OPERATION_CATEGORY);
      break;
    case 'layout-grid':
      await handleLayoutOrderSortChange('grid', 'layout');
      break;
    case 'layout-list':
      await handleLayoutOrderSortChange('list', 'layout');
      break;
    case 'order-name':
      await handleLayoutOrderSortChange('name', 'sort_by', true);
      break;
    case 'order-type':
      await handleLayoutOrderSortChange('type', 'sort_by', true);
      break;
    case 'order-time':
      await handleLayoutOrderSortChange('time', 'sort_by', true);
      break;
    case 'order-index':
      await handleLayoutOrderSortChange('order', 'sort_by', true);
      break;
    case 'sort-asc':
      await handleLayoutOrderSortChange('asc', 'sort_order', true);
      break;
    case 'sort-desc':
      await handleLayoutOrderSortChange('desc', 'sort_order', true);
      break;
  }

  handleClose();
}

async function handleLayoutOrderSortChange<T extends LayoutType | SortByType | SortOrderType>(
  val: T,
  upKey: keyof CategoryItem,
  updateLaunchList: boolean = false,
) {
  const params = {
    ...activeCategoryItem.value,
    [upKey]: val,
  };
  await updateCategory(params);

  const upCategory = store.categoryData.find(item => item.id === activeCategoryItem.value.id);
  if (!upCategory) return;
  // @ts-ignore
  upCategory[upKey] = val;

  if (updateLaunchList) store.getLaunchData();
}
</script>
