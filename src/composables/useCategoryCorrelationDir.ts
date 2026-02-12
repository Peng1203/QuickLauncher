import { useStore } from '@/store/useStore';
import { storeToRefs } from 'pinia';
import { watch as watchDir, readDir, exists } from '@tauri-apps/plugin-fs';
import { addLaunch, deleteLaunch, getFileInfo, getLaunchByNameAndCategory, updateLaunch } from '@/api';
import { batchRequest } from '@/utils/concurRequest';
import { differenceBy } from 'lodash-es';
import { useNaiveUiApi } from './useNaiveUiApi';
import { VNode } from 'vue';

/** 监听分类关联目录 */
export const useCategoryCorrelationDir = () => {
  const { dialog, message } = useNaiveUiApi();

  const store = useStore();
  const { activeCategory, categoryData } = storeToRefs(store);
  const activeCategoryItem = computed(() => {
    if (activeCategory.value === -1) return null;
    return categoryData.value.find(item => item.id === activeCategory.value);
  });

  const watchMap = new Map();

  const isConrrelationDir = computed(() => !!activeCategoryItem.value?.association_directory);

  /**
   * 获取到所有分类数据 注册分类目录监听
   */
  async function registerAllCategoryDirWatch() {
    // 判断到关联目录已经被删除时 归为失效分类
    const failureCateggorys = [];

    if (!categoryData.value.length) return;

    for await (const category of categoryData.value) {
      if (!category.association_directory) continue;
      if (watchMap.has(category.id)) continue;
      if (!(await exists(category.association_directory).catch(() => false))) {
        failureCateggorys.push(category.id);
        continue;
      }

      const unWatch = await watchDir(
        category.association_directory,
        async event => {
          const { paths, type } = event;
          const { create = '', remove = '', modify = '' } = type as any;

          if (create) await handleWatchCreate(paths, category);
          else if (remove) await handleWatchRemove(paths, category);
          else if (modify && modify.kind === 'rename') handleWatchRename(paths, category);

          store.getLaunchData();
        },
        { delayMs: 300, recursive: false },
      );
      watchMap.set(category.id, unWatch);
    }

    // 处理失效分类
  }

  const handleWatchCreate = async (paths: string[], category: CategoryItem) => {
    const fullPath = paths[0];
    return await getFileInfoAndCreateLaunch(fullPath, category, true);
  };
  const handleWatchRemove = async (paths: string[], category: CategoryItem) => {
    const removeFullPath = paths[0];
    const launchItem = await getLaunchByNameAndCategory(removeFullPath.split('\\').pop()!, category.id);
    if (!launchItem) return;
    await deleteLaunch(launchItem.id);
  };
  const handleWatchRename = async (paths: string[], category: CategoryItem) => {
    // 重命名操作 先删除旧的启动项 再添加新的启动项
    const [oldFullPath, newFullPath] = paths;
    const newName = newFullPath.split('\\').pop()!;

    const launchItem = await getLaunchByNameAndCategory(oldFullPath.split('\\').pop()!, category.id);
    if (!launchItem) return;
    const form = {
      ...launchItem,
      name: newName,
      path: newFullPath,
    };
    await updateLaunch(form);
  };

  /**
   * 删除指定的分类目录 watch
   */
  function removeCategoryDirWatch(id: number) {
    const unWatch = watchMap.get(id);
    if (!unWatch) return;
    unWatch();
    watchMap.delete(id);
  }

  /**
   * 分类创建成功时 创建关联目录中的所有启动项
   */
  async function handleCreateLaunchFromCategoryDir(category: CategoryItem) {
    const { association_directory } = category;
    if (!association_directory) return;
    const files = await readDir(association_directory);

    const tasks = files.map(file => () => getFileInfoAndCreateLaunch(file.name, category));
    await batchRequest(tasks);
  }

  /**
   * 根据目录中文件信息 创建启动项
   */
  const getFileInfoAndCreateLaunch = async (entryName: string, category: CategoryItem, isFullPath: boolean = false) => {
    const fullPath = isFullPath ? entryName : category.association_directory + '\\' + entryName;

    const fileInfo = await getFileInfo(fullPath);
    if (!fileInfo) return;
    const item: NewLaunchItem = {
      name: fileInfo.name,
      path: fileInfo.path,
      type: fileInfo.type,
      icon: fileInfo.icon,
      // category_id: null,
      hotkey: '',
      hotkey_global: 0,
      keywords: '',
      start_dir: fileInfo.start_dir,
      remarks: fileInfo.remarks || '',
      args: fileInfo.args || '',
      run_as_admin: 0,
      order_index: 0,
      enabled: category.exclude === 1 ? 0 : 1,
      category_id: category.id,
      subcategory_id: null,
      extension: fileInfo.extension,
    };
    // // 添加记录
    await addLaunch(item);
  };

  // 检查分类关联目录与启动项的同步情况
  const checkCategoryDirAndLaunchSync = async () => {
    const category = activeCategoryItem.value;
    if (!category) return;
    const { association_directory } = category;
    if (!association_directory) return;
    // 判断目录是否存在 不存在则返回提示
    if (!(await exists(association_directory).catch(() => false))) {
      dialog.warning({
        title: '提示',
        content: () => h('div', ['关联目录已被删除,是否删除当前分类？', h('br'), association_directory]),
        positiveText: '确 定',
        negativeText: '取 消',
        draggable: true,
        onPositiveClick: () => {
          // TODO 删除分类
          message.success('确定');
        },
      });
      return;
    }

    // 读取目录文件
    const files = await readDir(association_directory);
    // 获取该分类下的所有启动项
    if (files.length === store.launchData.length) return;
    const launchData = store.launchData.map(item => ({
      ...item,
      name: item.extension ? `${item.name}.${item.extension}` : item.name,
    }));
    // 批量处理任务
    let tasks: (() => Promise<any>)[];
    if (files.length > store.launchData.length) {
      // 目录中新添加了文件 向启动项中添加缺失的启动项
      const diffValue = differenceBy(files, launchData, 'name');
      tasks = diffValue.map(file => () => getFileInfoAndCreateLaunch(file.name, category));
    } else {
      // 目录中文件被删除了 从启动项中删除多余的启动项
      const diffValue = differenceBy(launchData, files, 'name');
      tasks = diffValue.map(launch => () => deleteLaunch(launch.id));
    }

    await batchRequest(tasks);
    // 更新分类列表
    store.getLaunchData(category.id);
  };

  return {
    activeCategoryItem,
    isConrrelationDir,
    registerAllCategoryDirWatch,
    removeCategoryDirWatch,
    handleCreateLaunchFromCategoryDir,
    checkCategoryDirAndLaunchSync,
  };
};
