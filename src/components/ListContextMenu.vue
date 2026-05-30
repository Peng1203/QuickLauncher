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
import { useCategorySort } from '@/composables';
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
const { handleLayoutOrderSortChange } = useCategorySort(activeCategoryItem);

function handleClose() {
  visible.value = false;
}

function renderIcon(icon: string) {
  return () => h(<i class={`iconfont ${icon}`} />);
}

// 默认菜单项
const menuOptions = computed(() => [
  {
    label: t('contextMenu.newLaunchItem'),
    key: 'add',
    icon: () => h(<i class="iconfont icon-xinjian" />),
  },
  {
    label: t('common.layout'),
    key: 'layout',
    icon: renderIcon('icon-buju'),
    children: [
      {
        label: activeCategoryItem.value?.layout === 'grid' ? `${t('common.tile')} (✅)` : t('common.tile'),
        key: 'layout-grid',
        props: {
          style: activeCategoryItem.value?.layout === 'grid' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
        },
        icon: renderIcon('icon-24gl-appsSmall'),
      },
      {
        label: activeCategoryItem.value?.layout === 'list' ? `${t('common.list')} (✅)` : t('common.list'),
        key: 'layout-list',
        props: {
          style: activeCategoryItem.value?.layout === 'list' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
        },
        icon: renderIcon('icon-liebiao'),
      },
    ],
  },
  {
    label: t('common.sortOrder'),
    key: 'order',
    icon: renderIcon('icon-paixufangshi'),
    children: [
      {
        label: activeCategoryItem.value?.sort_by === 'name' ? `${t('common.name')} (✅)` : t('common.name'),
        key: 'order-name',
        props: {
          style: activeCategoryItem.value?.sort_by === 'name' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
        },
        icon: renderIcon('icon-mingchengpaixu'),
      },
      {
        label: activeCategoryItem.value?.sort_by === 'type' ? `${t('common.type')} (✅)` : t('common.type'),
        key: 'order-type',
        props: {
          style: activeCategoryItem.value?.sort_by === 'type' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
        },
        icon: renderIcon('icon-anleixingpaixu'),
      },
      {
        label: activeCategoryItem.value?.sort_by === 'time' ? `${t('common.date')} (✅)` : t('common.date'),
        key: 'order-time',
        props: {
          style: activeCategoryItem.value?.sort_by === 'time' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
        },
        icon: renderIcon('icon-anchuangjianshijianpaixu'),
      },
      {
        label:
          activeCategoryItem.value?.sort_by === 'order'
            ? `${t('common.searchPriority')} (✅)`
            : t('common.searchPriority'),
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
        label: activeCategoryItem.value?.sort_order === 'asc' ? `${t('common.ascending')} (✅)` : t('common.ascending'),
        key: 'sort-asc',
        props: {
          style:
            activeCategoryItem.value?.sort_order === 'asc' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
        },
        icon: renderIcon('icon-shengxu'),
      },
      {
        label:
          activeCategoryItem.value?.sort_order === 'desc' ? `${t('common.descending')} (✅)` : t('common.descending'),
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
      EventBus.emit(AppEvent.OPEN_OPERATION_LAUNCH);
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
</script>
