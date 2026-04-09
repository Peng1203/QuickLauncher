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
import { open } from '@tauri-apps/plugin-dialog';
import { computed, h, ref } from 'vue';
import {
  deleteCategory,
  deleteLaunchByCategory,
  openPath,
  updateCategory,
  updateCategoryAssDir,
  updateLaunchEnabledByCategory,
} from '@/api';
import { useCategoryCorrelationDir } from '@/composables/useCategoryCorrelationDir';
import { useContextMenuClose } from '@/composables/useContextMenuClose';
import { useNaiveUiApi } from '@/composables/useNaiveUiApi';
import { AppEvent } from '@/constant';
import { useStore } from '@/store/useStore';
import { EventBus } from '@/utils/eventBus';

const props = defineProps<{
  position: { x: number; y: number };
  item: CategoryItem;
}>();
const { handleCreateLaunchFromCategoryDir, registerAllCategoryDirWatch, removeCategoryDirWatch } =
  useCategoryCorrelationDir();
const store = useStore();

const visible = defineModel<boolean>();

const { dialog } = useNaiveUiApi();

const isAssociationDirectory = computed(() => !!props.item?.association_directory);
const isCurrentSelected = computed(() => store.activeCategory === props.item.id);

function renderIcon(icon: string) {
  return () => h(<i class={`iconfont ${icon}`} />);
}
const sync_clear = ref<boolean>(false);
function CancelWaring() {
  return h(
    <>
      <p>是否取消关联目录?</p>
      {/* class="text-[12px] text-gray-500" */}
      <n-checkbox
        v-model:checked={sync_clear.value}
        style="font-size: 12px; color: #6a7282 !important"
        size="small"
        label="同时删除该分类下所有启动项?"
      />
    </>,
  );
}

// 默认菜单项
const menuOptions = computed(() => [
  {
    // TODO
    label: props.item?.exclude ? '搜索排除 (✅)' : '搜索排除',
    key: 'exclude',
    icon: renderIcon('icon-paichusousuo'),
    // show: item.
    props: {
      style: props.item?.exclude ? 'color: var(--n-color-danger);font-weight: bold;' : '',
    },
  },
  {
    label: isAssociationDirectory.value ? '关联目录 (✅)' : '关联目录',
    key: 'correlation',
    icon: renderIcon('icon-guanlian'),
    // show: !isAssociationDirectory.value,
    props: {
      style: isAssociationDirectory.value ? 'color: var(--n-color-danger);font-weight: bold;' : '',
    },
  },
  {
    label: '打开关联目录',
    key: 'openCorrelationDir',
    icon: renderIcon('icon-wj-wjj'),
    show: isAssociationDirectory.value,
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
  },
  {
    // TODO
    label: '创建子分类',
    key: 'create-sub-category',
    icon: renderIcon('icon-tianjiazifenlei'),
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
        props: {
          style: props.item?.layout === 'grid' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
        },
        icon: renderIcon('icon-24gl-appsSmall'),
      },
      {
        label: props.item?.layout === 'list' ? '列表 (✅)' : '列表',
        key: 'layout-list',
        props: {
          style: props.item?.layout === 'list' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
        },
        icon: renderIcon('icon-liebiao'),
      },
    ],
  },
  {
    // TODO 排序
    label: '排序方式',
    key: 'order',
    icon: renderIcon('icon-paixufangshi'),
    children: [
      {
        label: props.item?.sort_by === 'name' ? '名称 (✅)' : '名称',
        key: 'order-name',
        props: {
          style: props.item?.sort_by === 'name' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
        },
        icon: renderIcon('icon-mingchengpaixu'),
      },
      {
        label: props.item?.sort_by === 'type' ? '类型 (✅)' : '类型',
        key: 'order-type',
        props: {
          style: props.item?.sort_by === 'type' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
        },
        icon: renderIcon('icon-anleixingpaixu'),
      },
      {
        label: props.item?.sort_by === 'time' ? '日期 (✅)' : '日期',
        key: 'order-time',
        props: {
          style: props.item?.sort_by === 'time' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
        },
        icon: renderIcon('icon-anchuangjianshijianpaixu'),
      },
      {
        label: props.item?.sort_by === 'order' ? '搜索优先级 (✅)' : '搜索优先级',
        key: 'order-index',
        props: {
          style: props.item?.sort_by === 'order' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
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
        label: props.item?.sort_order === 'asc' ? '升序 (✅)' : '升序',
        key: 'sort-asc',
        props: {
          style: props.item?.sort_order === 'asc' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
        },
        icon: renderIcon('icon-shengxu'),
      },
      {
        label: props.item?.sort_order === 'desc' ? '降序 (✅)' : '降序',
        key: 'sort-desc',
        props: {
          style: props.item?.sort_order === 'desc' ? 'color: var(--n-color-danger);font-weight: bold;' : '',
        },
        icon: renderIcon('icon-jiangxu'),
      },
    ],
  },
]);

async function handleAssDir() {
  // 选择关联目录
  const association_directory = await open({
    title: '选择关联目录',
    multiple: false,
    directory: true,
  });
  if (!association_directory) return;
  // 查询是否有其他分类关联过当前目录
  const hasAssDir = store.categoryData.find(item => item.association_directory === association_directory);
  if (hasAssDir) {
    const isConfirm = await new Promise(resolve => {
      dialog.warning({
        title: '提示',
        content: `选中目录已存在关联的分类(${hasAssDir.name}) 是否继续关联?`,
        positiveText: '确 定',
        negativeText: '取 消',
        draggable: true,
        onPositiveClick: () => resolve(true),
        onNegativeClick: () => resolve(false),
      });
    });
    if (!isConfirm) return;
  }

  // 清除分类下的所有启动项
  await deleteLaunchByCategory(props.item.id);

  // 更新分类 关联目录
  await updateCategoryAssDir({ id: props.item.id, association_directory });
  await store.getCategoryData();
  // 添加关联分类
  await handleCreateLaunchFromCategoryDir({ ...props.item, association_directory });
  await registerAllCategoryDirWatch();

  if (isCurrentSelected.value) await store.getLaunchData(props.item.id);
}

async function handleCancelAssDir() {
  if (sync_clear.value) await deleteLaunchByCategory(props.item.id);
  await updateCategoryAssDir({ id: props.item.id, association_directory: '' });
  await store.getCategoryData();
  if (sync_clear.value && isCurrentSelected.value) await store.getLaunchData(props.item.id);
  await removeCategoryDirWatch(props.item.id);
  sync_clear.value = false;
}

function handleClose() {
  visible.value = false;
}

async function handleSelect(key: string) {
  console.log('selected key:', key);
  // const findRes = menuOptions.value.find(item => item.key === key);

  // if (!findRes) return handleClose();
  // findRes.onClick && (await findRes.onClick());
  // 子菜单 click 事件处理
  switch (key) {
    case 'exclude':
      await handleToggleQueryEnable();
      break;
    case 'correlation':
      await handleToggleAssDir();
      break;
    case 'openCorrelationDir':
      openPath(props.item!.association_directory!);
      break;
    case 'rename':
      EventBus.emit(AppEvent.CATEGORY_RENAME, props.item);
      break;
    case 'edit':
      EventBus.emit(AppEvent.OPEN_OPERATION_CATEGORY, props.item);
      break;
    case 'create-sub-category':
      // EventBus.emit(AppEvent.CATEGORY_RENAME, props.item);
      break;
    case 'delete':
      await handleDelete();
      break;
    case 'layout-grid':
      await handleLayoutChange('grid');
      break;
    case 'layout-list':
      await handleLayoutChange('list');
      break;
    case 'order-name':
      await handleOrderChange('name');
      break;
    case 'order-type':
      await handleOrderChange('type');
      break;
    case 'order-time':
      await handleOrderChange('time');
      break;
    case 'order-index':
      await handleOrderChange('order');
      break;
    case 'sort-asc':
      await handleSortChange('asc');
      break;
    case 'sort-desc':
      await handleSortChange('desc');
      break;
    default:
      break;
  }
  handleClose();
}

async function handleToggleQueryEnable() {
  const { id, exclude } = props.item;
  const newExclude = !exclude;
  const params = {
    ...props.item,
    exclude: newExclude,
  };
  await updateCategory(params);
  const upCategory = store.categoryData.find(item => item.id === props.item.id);
  if (!upCategory) return;
  upCategory.exclude = newExclude;

  await updateLaunchEnabledByCategory(id, newExclude !== true);
}

async function handleToggleAssDir() {
  if (!store.launchData.length) return handleAssDir();

  dialog.warning({
    title: isAssociationDirectory.value ? '提示' : '警告',
    content: isAssociationDirectory.value ? CancelWaring : '关联目录将清空该分类下的所有启动项 是否继续关联?',
    positiveText: '确 定',
    negativeText: '取 消',
    draggable: true,
    onPositiveClick: async () => {
      // message.success('确定');
      isAssociationDirectory.value ? handleCancelAssDir() : handleAssDir();
    },
    onNegativeClick: () => {
      // message.info('取消');
    },
  });
}

async function handleDelete() {
  return new Promise(resolve => {
    dialog.warning({
      title: '提示',
      content: `是否删除 ${props.item.name} 分类?`,
      positiveText: '确 定',
      negativeText: '取 消',
      draggable: true,
      onPositiveClick: async () => {
        await deleteCategory(props.item.id);
        // 当删除的分类为当前选择的分类时 重置到默认分类
        if (props.item.id === store.activeCategory) {
          store.activeCategory = -1;
          await store.getLaunchData();
        }
        await store.getCategoryData();
        resolve(true);
      },
      onNegativeClick: () => resolve(true),
    });
  });
}

async function handleLayoutChange(val: LayoutType) {
  const params = {
    ...props.item,
    layout: val,
  };
  await updateCategory(params);

  const upCategory = store.categoryData.find(item => item.id === props.item.id);
  if (!upCategory) return;
  upCategory.layout = val;
}
async function handleOrderChange(val: SortByType) {
  const params = {
    ...props.item,
    sort_by: val,
  };
  await updateCategory(params);
  const upCategory = store.categoryData.find(item => item.id === props.item.id);
  if (!upCategory) return;
  upCategory.sort_by = val;
  if (isCurrentSelected.value) store.getLaunchData();
}
async function handleSortChange(val: SortOrderType) {
  const params = {
    ...props.item,
    sort_order: val,
  };
  await updateCategory(params);
  const upCategory = store.categoryData.find(item => item.id === props.item.id);
  if (!upCategory) return;
  upCategory.sort_order = val;
  if (isCurrentSelected.value) store.getLaunchData();
}

useContextMenuClose(handleClose);
</script>
