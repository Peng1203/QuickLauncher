import { AppEvent } from '@/constant';
import { useStore } from '@/store/useStore';
import { EventBus } from '@/utils/eventBus';
import { isEmpty } from 'lodash-es';
import { storeToRefs } from 'pinia';

export function useMainWindowShortcut() {
  const store = useStore();
  const { activeCategoryItem, activeLaunchItem, activeCursorX, activeCursorY, launchData } = storeToRefs(store);

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
      case 'ArrowUp':
      case 'ArrowRight':
      case 'ArrowDown':
      case 'ArrowLeft':
        handleMoveSelection(key);
        break;

      default:
        break;
    }
    // e.preventDefault();
  };

  const gridRowMaxItem = computed(() => (activeCategoryItem.value?.layout === 'list' ? 1 : 6));
  const rowTotal = computed(() => Math.ceil((launchData.value.length + 1) / gridRowMaxItem.value));
  const navigationMap = computed(() => {
    const map = new Map();
    launchData.value.forEach((item, i) => {
      const posX = Math.ceil((i + 1) / gridRowMaxItem.value);
      const posY = (i + 1) % gridRowMaxItem.value || gridRowMaxItem.value;
      map.set(`${posX},${posY}`, item);
    });
    return map;
  });
  function handleMoveSelection(key: string) {
    // 当前没有选中 列表元素时执行分类上下切换的快捷键

    // if (!launchData.value.length) return;

    // 初始坐标可以通过右方向键选中
    if (key === 'ArrowRight' && launchData.value.length && !activeCursorX.value && !activeCursorY.value) {
      activeCursorX.value = 1;
      activeCursorY.value = 1;
    } else {
      switch (key) {
        case 'ArrowUp': {
          const nextX = activeCursorX.value - 1;
          if (nextX < 1) return;

          let nextY = activeCursorY.value;

          while (nextY > 0) {
            if (navigationMap.value.has(`${nextX},${nextY}`)) {
              activeCursorX.value = nextX;
              activeCursorY.value = nextY;
              break;
            }
            nextY--;
          }

          break;
        }

        case 'ArrowDown': {
          const nextX = activeCursorX.value + 1;
          if (nextX > rowTotal.value) return;

          let nextY = activeCursorY.value;

          while (nextY > 0) {
            if (navigationMap.value.has(`${nextX},${nextY}`)) {
              activeCursorX.value = nextX;
              activeCursorY.value = nextY;
              break;
            }
            nextY--;
          }

          break;
        }
        case 'ArrowLeft': {
          // 如果处于第一个选中的启动项 再次按下时 回到初始状态 取消选中元素
          if (activeCursorX.value === 1 && activeCursorY.value === 1) {
            activeCursorX.value = 0;
            activeCursorY.value = 0;
            // document.body.focus();
            const siderEl = document.querySelector('#layoutSider') as any;
            siderEl?.focus();
          } else {
            const nextY = activeCursorY.value - 1;
            // 当前行正常左移
            if (nextY >= 1 && navigationMap.value.has(`${activeCursorX.value},${nextY}`)) {
              activeCursorY.value = nextY;
              break;
            }

            // 当前已经是第一个元素 -> 跳上一行最后一个
            const prevX = activeCursorX.value - 1;
            if (prevX < 1) return;

            let lastY = gridRowMaxItem.value;

            while (lastY > 0) {
              if (navigationMap.value.has(`${prevX},${lastY}`)) {
                activeCursorX.value = prevX;
                activeCursorY.value = lastY;
                break;
              }
              lastY--;
            }
          }

          break;
        }

        case 'ArrowRight': {
          const nextY = activeCursorY.value + 1;

          // 当前行正常右移
          if (navigationMap.value.has(`${activeCursorX.value},${nextY}`)) {
            activeCursorY.value = nextY;
            break;
          }

          // 当前已经是最后一个元素 -> 跳下一行第一个
          const nextX = activeCursorX.value + 1;
          if (nextX > rowTotal.value) return;

          let firstY = 1;

          while (firstY <= gridRowMaxItem.value) {
            if (navigationMap.value.has(`${nextX},${firstY}`)) {
              activeCursorX.value = nextX;
              activeCursorY.value = firstY;
              break;
            }
            firstY++;
          }

          break;
        }
      }
    }

    const position = `${activeCursorX.value},${activeCursorY.value}`;
    const nextItem = navigationMap.value.get(position);
    if (!nextItem) return (activeLaunchItem.value = null);
    activeLaunchItem.value = nextItem;
  }

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
    isEmp ? EventBus.emit(AppEvent.DELETE_CATEGORY) : EventBus.emit(AppEvent.DELETE_LAUNCH);
  }

  onMounted(() => window.addEventListener(EVENT, handler, true));
  onUnmounted(() => window.removeEventListener(EVENT, handler));
}
