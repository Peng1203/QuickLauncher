import { AppEvent } from '@/constant';
import { useStore } from '@/store/useStore';
import { EventBus } from '@/utils/eventBus';
import { isEmpty } from 'lodash-es';
import { storeToRefs } from 'pinia';

export function useMainWindowShortcut() {
  const store = useStore();
  const { activeCategoryItem, activeLaunchItem } = storeToRefs(store);

  const EVENT = 'keydown';

  const handler = (e: KeyboardEvent) => {
    const { key, ctrlKey, shiftKey, altKey } = e;
    // const el = e.target;
    // console.log(`%c key ----`, 'color: #fff;background-color: #000;font-size: 18px', key);
    // console.log(`%c ctrlKey ----`, 'color: #fff;background-color: #000;font-size: 18px', ctrlKey);
    // console.log(`%c shiftKey ----`, 'color: #fff;background-color: #000;font-size: 18px', shiftKey);
    // console.log(`%c altKey ----`, 'color: #fff;background-color: #000;font-size: 18px', altKey);

    switch (key) {
      case 'F2': // 重命名
        rename(e);
        break;
      case 'F4': // 编辑
        edit();
        break;
      case 'N': // 新建启动项
        ctrlKey && shiftKey && addLaunch();
        break;
      case 'C': // 新建分类
        ctrlKey && shiftKey && addCategory();
        break;
      case 'Delete': // 删除
        handleDelete();
        break;

      default:
        break;
    }
    e.preventDefault();
  };

  /**
   * 重命名 分为分类重命名和启动项重命名
   * 分类重命名组件内部 使用自己的键盘事件
   * 当触发全局的重命名操作时如果 document.activeElement 选中的元素为侧边栏元素
   * 则触发分类重命名 其他情况触发 启动项重命名
   *
   * 分类 src\views\Main\components\Sidebar.vue
   */
  function rename(e: KeyboardEvent) {
    const siderEl = document.querySelector('#layoutSider');
    // const el = e.target as Node;
    // console.log(`%c siderEl ----`, 'color: #fff;background-color: #000;font-size: 18px', siderEl);
    // console.log(`%c el ----`, 'color: #fff;background-color: #000;font-size: 18px', el);
    // if (siderEl?.isSameNode(el) || siderEl?.contains(el)) { }
    // 使用 Sidbar 组件内部键盘事件处理
    if (siderEl?.contains(document.activeElement) && siderEl?.matches(':hover')) return;

    EventBus.emit(AppEvent.LAUNCH_RENAME);
  }

  /**
   * 编辑 当有选中启动项是 打开启动项编辑 当没有选中启动项时 打开分类编辑
   */
  function edit() {
    const isEmp = isEmpty(activeLaunchItem.value);
    isEmp
      ? EventBus.emit(AppEvent.OPEN_OPERATION_CATEGORY, activeCategoryItem.value)
      : EventBus.emit(AppEvent.OPEN_OPERATION_LAUNCH, activeLaunchItem.value);
  }

  function addLaunch() {
    EventBus.emit(AppEvent.OPEN_OPERATION_LAUNCH);
  }
  function addCategory() {
    EventBus.emit(AppEvent.OPEN_OPERATION_CATEGORY);
  }

  function handleDelete() {
    const isEmp = isEmpty(activeLaunchItem.value);
    console.log(`%c isEmp ----`, 'color: #fff;background-color: #000;font-size: 18px', isEmp);
    isEmp ? EventBus.emit(AppEvent.DELETE_CATEGORY) : EventBus.emit(AppEvent.DELETE_LAUNCH);
  }

  onMounted(() => window.addEventListener(EVENT, handler, true));
  onUnmounted(() => window.removeEventListener(EVENT, handler));
}
