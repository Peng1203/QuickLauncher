<template>
  <!-- style="max-height: 300px; overflow-y: auto" -->
  <n-dropdown
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
import { storeToRefs } from 'pinia';
import { h } from 'vue';
import {
  deleteCategory,
  deleteLaunchByCategory,
  openPath,
  updateCategory,
  updateCategoryAssDir,
  updateLaunchEnabledByCategory,
} from '@/api';
import { useCategoryCorrelationDir, useLayoutOrderMenu, useNaiveUiApi } from '@/composables';
import { AppEvent } from '@/constant';
import { t } from '@/i18n';
import { useStore } from '@/store/useStore';
import { EventBus } from '@/utils/eventBus';

const props = defineProps<{
  position: { x: number; y: number };
  item: CategoryItem;
}>();

const visible = defineModel<boolean>();

const store = useStore();
const { dialog } = useNaiveUiApi();

const { handleCreateLaunchFromCategoryDir, registerAllCategoryDirWatch, removeCategoryDirWatch } =
  useCategoryCorrelationDir();
const itemRef = toRefs(props).item;
const { layoutMenu, orderMenu, handleLayoutOrderSelect } = useLayoutOrderMenu(itemRef);

const { activeCategoryItem, defaultCategory } = storeToRefs(store);

const isAssociationDirectory = computed(() => !!props.item?.association_directory);
const isDefaultCategory = computed(() => props.item?.id !== defaultCategory.value?.id);
const isCurrentSelected = computed(() => store.activeCategory === props.item.id);

function renderIcon(icon: string) {
  return () => h(<i class={`iconfont ${icon}`} />);
}
const sync_clear = ref<boolean>(false);
function CancelWaring() {
  return h(
    <>
      <p>{t('category.cancelAssociate')}</p>
      {/* class="text-[12px] text-gray-500" */}
      <n-checkbox
        v-model:checked={sync_clear.value}
        class="text-[12px] text-muted-foreground"
        size="small"
        label={t('category.alsoDeleteItems')}
      />
    </>,
  );
}

function renderLabel(prop: keyof CategoryItem, label: string, value?: string) {
  if(value) return props.item?.[prop] === value ? `${label} (✅)` : label;
  return props.item?.[prop] ? `${label} (✅)` : label;
}

function renderMenuProps(prop: keyof CategoryItem, value?: string) {
  if (!value) {
    return {
      class: props.item?.[prop] ? 'font-bold text-[var(--n-color-danger)]' : '',
    }
  }
  return {
    class: props.item?.[prop] === value ? 'font-bold text-[var(--n-color-danger)]' : '',
  }
}


// 默认菜单项
const menuOptions = computed(() => [
  {
    // TODO
    // label: props.item?.exclude ? `${t('common.searchExclude')} (✅)` : t('common.searchExclude'),
    label: renderLabel('exclude', t('common.searchExclude')),
    key: 'exclude',
    icon: renderIcon('icon-paichusousuo'),
    show: isDefaultCategory.value,
    props: {
      style: renderMenuProps('exclude'),
    },
  },
  {
    // label: isAssociationDirectory.value ? `${t('common.associateDir')} (✅)` : t('common.associateDir'),
    label: renderLabel('association_directory', t('common.associateDir')),
    key: 'correlation',
    icon: renderIcon('icon-guanlian'),
    show: isDefaultCategory.value,
    props: {
      // style: isAssociationDirectory.value ? 'color: var(--n-color-danger);font-weight: bold;' : '',
      style: renderMenuProps('association_directory'),
    },
  },
  {
    label: t('common.openAssociatedDir'),
    key: 'openCorrelationDir',
    icon: renderIcon('icon-wj-wjj'),
    show: isAssociationDirectory.value,
  },
  {
    type: 'divider',
    key: 'd1',
    show: isDefaultCategory.value,
  },
  {
    label: t('common.rename'),
    key: 'rename',
    icon: renderIcon('icon-zhongmingming'),
  },
  // {
  //   label: '设置图标',
  //   key: 'set-icon',
  //   icon: renderIcon('icon-zhongmingming'),
  // },
  {
    label: t('common.edit'),
    key: 'edit',
    icon: renderIcon('icon-bianji'),
  },
  {
    // TODO
    label: t('common.createSubcategory'),
    key: 'create-sub-category',
    icon: renderIcon('icon-tianjiazifenlei'),
    show: isDefaultCategory.value,
  },
  {
    label: t('common.delete'),
    key: 'delete',
    icon: renderIcon('icon-shanchu'),
    show: isDefaultCategory.value,
  },
  {
    type: 'divider',
    key: 'd2',
  },
  layoutMenu.value,
  orderMenu.value,
]);

async function handleAssDir() {
  // 选择关联目录
  const association_directory = await open({
    title: t('category.selectDir'),
    multiple: false,
    directory: true,
  });
  if (!association_directory) return;
  // 查询是否有其他分类关联过当前目录
  const hasAssDir = store.categoryData.find(item => item.association_directory === association_directory);
  if (hasAssDir) {
    const isConfirm = await new Promise(resolve => {
      dialog.warning({
        title: t('common.tip'),
        content: `${t('category.dirConflict')}(${hasAssDir.name}) ${t('category.dirContinue')}`,
        positiveText: t('common.confirmDelete'),
        negativeText: t('common.cancel'),
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
      EventBus.emit(AppEvent.CATEGORY_RENAME);
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
    default:
      if (handleLayoutOrderSelect(key)) break;
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
    title: isAssociationDirectory.value ? t('common.tip') : t('common.warning'),
    content: isAssociationDirectory.value ? CancelWaring : t('category.dirClearWarning'),
    positiveText: t('common.confirmDelete'),
    negativeText: t('common.cancel'),
    draggable: true,
    onPositiveClick: async () => {
      // message.success('确定');
      if (isAssociationDirectory.value) handleCancelAssDir()
      else handleAssDir()
    },
    onNegativeClick: () => {
      // message.info('取消');
    },
  });
}

async function handleDelete() {
  return new Promise(resolve => {
    dialog.warning({
      title: t('common.tip'),
      content: `${t('category.deleteConfirm')} ${props.item?.name || activeCategoryItem.value.name} ${t('category.deleteCategory')}`,
      positiveText: t('common.confirmDelete'),
      negativeText: t('common.cancel'),
      draggable: true,
      onPositiveClick: async () => {
        const id = props.item?.id || activeCategoryItem.value.id;
        await deleteCategory(id);
        // 当删除的分类为当前选择的分类时 重置到默认分类
        if (id === store.activeCategory) {
          // store.activeCategory = defaultCategory.value.id;
          await store.handleChangeCategory(defaultCategory.value.id);
        }
        await store.getCategoryData();
        resolve(true);
      },
      onNegativeClick: () => resolve(true),
    });
  });
}

EventBus.listen(AppEvent.DELETE_CATEGORY, handleDelete);
</script>
