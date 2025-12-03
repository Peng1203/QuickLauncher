<template>
  <div
    v-if="visible"
    ref="menuRef"
    class="fixed z-50 rounded-lg shadow-lg bg-white border border-gray-200"
    :style="{
      top: `${calcPosition.y}px`,
      left: `${calcPosition.x}px`,
      width: `${MENU_WIDTH}px`,
    }"
    @click.stop
  >
    <ul class="text-sm text-gray-700">
      <template v-for="menu in menuItems" :key="menu.label">
        <li
          v-if="menu?.itemVisible === undefined ? true : menu.itemVisible()"
          class="px-4 py-2 hover:bg-gray-100 cursor-pointer"
          @click="handleClick(menu)"
        >
          {{ menu.label }}
        </li>
      </template>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { ask } from '@tauri-apps/plugin-dialog';
import { useMessage } from 'naive-ui';
import { onMounted, onUnmounted, ref } from 'vue';
import { deleteLaunch, openPath, runLaunchAsAdmin } from '@/api';
import { useAppConfig } from '@/composables/useAppConfig';
import { AppEvent, MENU_WIDTH } from '@/constant';
import { EventBus } from '@/utils/eventBus';

export interface MenuAction {
  label: string;
  onClick: () => void;
  itemVisible?: () => void;
}

// visible: boolean
const props = defineProps<{
  position: { x: number; y: number };
  item: LaunchItem;
  itemPath: string;
  itemName: string;
  selectedIds: number[];
  extraItems?: MenuAction[];
}>();

const emit = defineEmits(['close', 'rename']);

const visible = defineModel<boolean>();

const message = useMessage();
const { appConfigStore } = useAppConfig();
const selected = computed(() => !!((props.selectedIds?.length || 0) > 1));
function handleClick(item: MenuAction) {
  item.onClick();
  handleCloseMenu();
}

// 默认菜单项
const menuItems = ref<MenuAction[]>([
  {
    label: '以管理员身份运行',
    onClick: () => runLaunchAsAdmin(props.item.id),
    itemVisible: () =>
      !selected.value && ['exe'].includes(props.item?.extension || ''),
  },
  {
    label: '打开所在位置',
    onClick: async () => openPath(props.item.path),
    itemVisible: () =>
      !selected.value && ['file', 'directory'].includes(props.item.type),
  },
  {
    label: '复制路径',
    onClick: async () => {
      await writeText(props.itemPath);
      message.success('复制成功');
    },
    itemVisible: () => !selected.value,
  },
  {
    label: '重命名',
    onClick: () => emit('rename'),
    itemVisible: () => !selected.value,
  },
  {
    label: '删除',
    onClick: async () => {
      if (appConfigStore.confirmBeforeDelete) {
        const tip = selected.value
          ? `是否批量删除选中的 ${props.selectedIds.length} 个启动项 ?`
          : `是否删除 ${props.itemName} ?`;
        const answer = await ask(tip, {
          title: '删 除',
          kind: 'warning',
        });
        if (!answer) return;
      }
      await Promise.all(props.selectedIds.map(id => deleteLaunch(id)));
      EventBus.emit(AppEvent.UPDATE_LAUNCH_LIST);
    },
  },
  {
    label: '编辑',
    onClick: () => {
      EventBus.emit(AppEvent.OPEN_OPERATION_LAUNCH, props.item);
    },
    itemVisible: () => !selected.value,
  },
  // {
  //   label: '设置开机启动',
  //   onClick: () => {
  //     console.log('设置开机启动')
  //   },
  // },
  // {
  //   label: '移除启动项',
  //   onClick: () => {
  //     console.log('移除启动项')
  //   },
  // },
]);

// 自动监听点击窗口其他地方关闭菜单
function handleCloseMenu() {
  visible.value = false;
}

const menuRef = useTemplateRef('menuRef');

function handleOutsideClick(e: MouseEvent) {
  if (menuRef.value && !menuRef.value.contains(e.target as Node)) {
    handleCloseMenu();
  }
}

// 菜单距离窗口边的距离
const VIEWPORT_MARGIN = 5;

// 计算出菜单出现的 x y 位置
const calcPosition = computed(() => {
  let x = props.position.x;
  let y = props.position.y;

  if (props.position.x + MENU_WIDTH > window.innerWidth) {
    x = window.innerWidth - MENU_WIDTH - VIEWPORT_MARGIN;
  }

  if (menuRef.value) {
    if (props.position.y + menuRef.value.offsetHeight > window.innerHeight) {
      y = window.innerHeight - menuRef.value.offsetHeight - VIEWPORT_MARGIN;
    }
  }

  return { x, y };
});

onMounted(() => {
  EventBus.listen(AppEvent.CLOSE_CONTEXT_MENU, handleCloseMenu);

  window.addEventListener('click', handleOutsideClick);
  window.addEventListener('contextmenu', handleOutsideClick);
  window.addEventListener('scroll', handleCloseMenu, true);
});
onUnmounted(() => {
  window.removeEventListener('click', handleOutsideClick);
  window.removeEventListener('contextmenu', handleOutsideClick);
  window.removeEventListener('scroll', handleCloseMenu, true);
});
</script>
