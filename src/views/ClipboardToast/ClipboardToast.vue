<template>
  <div
    v-if="visible"
    style="height: calc(100% + 1px)"
    :style="{
      background: `rgba(255, 255, 255, ${appConfigStore.portalOpacity / 100})`,
    }"
    data-tauri-drag-region
    class="relative box-border flex flex-col p-3 w-full overflow-hidden border border-black/5 bg-white/90 shadow-[0_6px_20px_rgba(0,0,0,.12)] dark:border-white/10 dark:bg-[#111827]/90"
  >
    <!-- content wrapper -->
    <div class="flex flex-1 flex-col gap-2">
      <!-- header -->
      <div class="flex items-start justify-between">
        <div class="flex items-center gap-2">
          <div class="flex h-8 w-8 items-center justify-center rounded-lg bg-black/5 dark:bg-white/10">
            <NIcon
              size="18"
              :color="themeColor?.light"
            >
              <FolderOutline v-if="isDirectory" />
              <LinkOutline v-else />
            </NIcon>
          </div>

          <div class="text-[15px] font-semibold text-[#111827] dark:text-white truncate">
            {{ info.title }}
          </div>
        </div>

        <NIcon
          class="cursor-pointer"
          size="14"
          @click="handleClose"
        >
          <CloseOutline />
        </NIcon>
      </div>

      <!-- content -->
      <div class="text-[12px] leading-5 text-gray-600 dark:text-white/75 truncate">
        <template v-if="appConfigStore.portalShowPath">
          {{ content || props.content }}
        </template>
      </div>
    </div>

    <!-- actions -->
    <div class="flex items-center gap-2">
      <NButton
        type="success"
        size="tiny"
        class="!h-8 !flex-1 !rounded-lg"
        @click="open"
      >
        <template #icon>
          <NIcon
            v-if="isDirectory"
            size="14"
            class="iconfont iconfont icon-wj-wjj"
          />
          <NIcon
            v-else
            size="14"
          >
            <OpenOutline />
          </NIcon>
        </template>

        {{ info.actionText }}
        <template v-if="appConfigStore.portalShowShortcut">(Home)</template>
      </NButton>

      <NButton
        v-if="isDirectory"
        color="#1766f7"
        size="tiny"
        class="!h-8 !flex-1 !rounded-lg"
        @click="openDirInManager"
      >
        <template #icon>
          <NIcon
            size="12"
            class="iconfont icon-dakaisuozaiwenjianjia"
          />
        </template>
        资源管理器中打开
        <template v-if="appConfigStore.portalShowShortcut">(PageUp)</template>
      </NButton>
    </div>

    <div
      v-if="appConfigStore.portalShowProgress"
      class="absolute bottom-0 left-0 h-[5px] w-full overflow-hidden bg-transparent"
    >
      <div
        :key="animationKey"
        :class="isDefaultModel ? 'timer-bar' : 'timer-bar-'"
        class="h-full"
        :style="{
          animationDuration: `${appConfigStore.portalDuration}ms`,
          background: `linear-gradient(90deg, ${themeColor.light}, ${themeColor.light}aa)`,
          boxShadow: `0 0 8px ${themeColor.light}`,
        }"
      ></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { listen } from '@tauri-apps/api/event';
import { TrayIcon } from '@tauri-apps/api/tray';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { CloseOutline, FolderOutline, LinkOutline, OpenOutline } from '@vicons/ionicons5';
import { NButton, NIcon } from 'naive-ui';
import { computed, onMounted, onUnmounted, ref } from 'vue';
import { exeCommand, openRevealManager, setDefaultTrayIcon } from '@/api';
import { useAppConfig } from '@/composables';
import { AppEvent } from '@/constant';
import { sleep } from '@/utils/delay';
import { EventBus } from '@/utils/eventBus';
import { register, unRegisterShortcutKey } from '@/utils/shortcutKey';

interface Props {
  model?: 'default' | 'setLocation' | 'demo';
  visible?: boolean;
  content?: string;
  type?: ClipboardContentType;
}

const props = withDefaults(defineProps<Props>(), { model: 'default', visible: false });

const { appConfigStore } = useAppConfig();
const type = ref<ClipboardContentType>(props.type || 'Url');
const content = ref<string>(props.content || '');
const visible = ref(props.visible);
const isDirectory = computed(() => type.value === 'Directory');
// const model = ref<Props['model']>();
const currentModel = ref<Props['model']>(props.model || 'default');
const isDefaultModel = computed(() => currentModel.value === 'default');

const themeColor = computed(() => {
  return isDirectory.value
    ? {
        light: '#8b5cf6',
        bg: 'from-violet-500/15 to-fuchsia-500/10',
        button: '#8b5cf6',
      }
    : {
        light: '#2563eb',
        bg: 'from-blue-500/15 to-cyan-500/10',
        button: '#2563eb',
      };
});

const info = computed(() => {
  if (currentModel.value === 'setLocation') {
    return {
      title: '设置窗口位置',
      actionText: '确认',
    };
  }
  return isDirectory.value
    ? {
        title: '目录已复制',
        actionText: '打开',
      }
    : {
        title: '链接已复制',
        actionText: '打开',
      };
});

const currentWindow = getCurrentWebviewWindow();

const timer = ref();

const OPEN_SHORTCUT_KEY = 'Home';
const OPEN_DIR_IN_MANAGER_SHORTCUT_KEY = 'PageUp';
const shortcutRegistered = ref(false);

async function handleClose() {
  if (currentModel.value === 'demo') return;
  // 当处于设置位置模式时 直接关闭
  if (currentModel.value === 'setLocation') {
    await currentWindow?.hide();
    visible.value = false;
  } else if (appConfigStore.portalNotifyMode === 'tray') {
    handleEndFlashTray();
  } else if (appConfigStore.portalNotifyMode === 'window') {
    if (!visible.value) return;
    timer.value && clearTimeout(timer.value);
    await currentWindow?.hide();
    visible.value = false;
  }

  // 解除注册的快捷键打开
  if (shortcutRegistered.value) {
    unRegisterShortcutKey(OPEN_SHORTCUT_KEY);
    unRegisterShortcutKey(OPEN_DIR_IN_MANAGER_SHORTCUT_KEY);

    shortcutRegistered.value = false;
  }
}

const animationKey = ref(0);
// 弹窗通知
async function handleShowWindow() {
  animationKey.value++;
  await currentWindow?.show();
  await handleRegisterShortcutKey();
  timer.value && clearTimeout(timer.value);
  timer.value = setTimeout(handleClose, appConfigStore.portalDuration);

  visible.value = true;
}

const flashFlag = ref(false);
const flashTimer = ref();
// 闪烁锁
const flashLock = ref(false);
// 托盘闪烁通知
async function handleFlashTray() {
  const tray = await TrayIcon.getById('tray');
  flashFlag.value = true;
  flashTimer.value && clearTimeout(flashTimer.value);
  flashTimer.value = setTimeout(handleEndFlashTray, appConfigStore.portalDuration);

  handleRegisterShortcutKey();
  if (flashLock.value) return;
  while (flashFlag.value) {
    // console.log(`%c 托盘闪烁 ----`, 'color: #fff;background-color: #000;font-size: 18px');
    flashLock.value = true;
    tray?.setIcon(null);
    await sleep(500);

    // tray?.setIcon('tray/32x32.png');
    await setDefaultTrayIcon();
    tray?.setTooltip(info.value.title);
    await sleep(500);
  }
}
// 托盘通知完毕
async function handleEndFlashTray() {
  flashFlag.value = false;
  flashTimer.value && clearTimeout(flashTimer.value);
  setDefaultTrayIcon();
  const tray = await TrayIcon.getById('tray');
  tray?.setTooltip(appConfigStore.title);
  flashLock.value = false;
}

async function handleRegisterShortcutKey() {
  if (shortcutRegistered.value || !appConfigStore.portalEnableShortcut) return;
  await Promise.all([
    unRegisterShortcutKey(OPEN_SHORTCUT_KEY),
    unRegisterShortcutKey(OPEN_DIR_IN_MANAGER_SHORTCUT_KEY),
  ]);

  register(OPEN_SHORTCUT_KEY, open);
  isDirectory.value && register(OPEN_DIR_IN_MANAGER_SHORTCUT_KEY, openDirInManager);
  shortcutRegistered.value = true;
}

async function open() {
  if (currentModel.value === 'setLocation') return handleClose();
  if (!content.value) return;
  if (!isDefaultModel.value) return;
  await exeCommand(content.value);
  handleClose();
}
async function openDirInManager() {
  if (!isDirectory.value) return;
  if (!isDefaultModel.value) return;
  await openRevealManager(content.value);
  handleClose();
}

let unlistenClipboard: (() => void) | null = null;

const timer2 = ref();
currentWindow.onMoved(({ payload: position }) => {
  clearTimeout(timer2.value);
  timer2.value = setTimeout(() => {
    appConfigStore.portalWindowPositionX = position.x;
    appConfigStore.portalWindowPositionY = position.y;
  }, 100);
});
EventBus.listen(AppEvent.OPEN_CLIPBOARD_WINDOW_BY_SET_LOCATION_MODAL, async () => {
  currentModel.value = 'setLocation';
  content.value = '拖动窗口设置位置';
  visible.value = true;
  await currentWindow?.show();
});

onMounted(async () => {
  if (isDefaultModel.value) {
    unlistenClipboard = await listen(AppEvent.CLIPBOARD, async ({ payload }: { payload: ClipboardPayload }) => {
      if (!appConfigStore.portalEnabled) return;
      // 判断 portalNotifyMode 是什么方式
      const { content: str, content_type } = payload;
      if (content_type === 'Unknown') return;
      currentModel.value = 'default';
      content.value = str;
      type.value = content_type;

      switch (appConfigStore.portalNotifyMode) {
        case 'window':
          handleShowWindow();
          break;
        case 'tray':
          handleFlashTray();
          break;
        case 'silent':
          handleRegisterShortcutKey();
          break;
      }
    });
  }
});

onUnmounted(() => {
  if (isDefaultModel.value) {
    timer.value && clearTimeout(timer.value);
    flashTimer.value && clearTimeout(flashTimer.value);
    unRegisterShortcutKey(OPEN_SHORTCUT_KEY);
    unRegisterShortcutKey(OPEN_DIR_IN_MANAGER_SHORTCUT_KEY);
    unlistenClipboard?.();
  }
});
</script>

<style scoped lang="scss">
.timer-bar {
  z-index: 9;
  width: 100%;
  transform-origin: left;
  animation: timer linear forwards;
}
.timer-bar- {
  z-index: 9;
  width: 100%;
  transform-origin: left;
}

@keyframes timer {
  from {
    transform: scaleX(1);
  }

  to {
    transform: scaleX(0);
  }
}

.toast-enter-active,
.toast-leave-active {
  transition:
    opacity 0.25s ease,
    transform 0.25s ease;
}

.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translateY(20px) scale(0.96);
}
</style>
