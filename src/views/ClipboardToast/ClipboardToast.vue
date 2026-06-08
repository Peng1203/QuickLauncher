<template>
  <div
    v-if="visible"
    style="height: calc(100% + 1px)"
    :style="{
      background: `color-mix(in srgb, var(--card) ${appConfigStore.portalOpacity}%, transparent)`,
    }"
    data-tauri-drag-region
    class="relative box-border flex flex-col p-3 w-full overflow-hidden border border-border bg-card/90 shadow-[0_6px_20px_rgba(0,0,0,.12)]"
  >
    <!-- content wrapper -->
    <div class="flex flex-1 flex-col gap-2">
      <!-- header -->
      <div class="flex items-start justify-between">
        <div class="flex items-center gap-2">
          <Icon
            background
            :name="isDirectory ? 'icon-wj-wjj' : 'icon-url'"
            :color="themeColor?.light"
          />

          <div class="text-[15px] font-semibold text-foreground truncate">
            {{ info.title }}
          </div>
        </div>

        <Icon name="icon-guanbichuangkou" size="14" class="cursor-pointer" @click="handleClose" />
      </div>

      <!-- content -->
      <div class="text-[12px] leading-5 text-muted-foreground truncate">
        <template v-if="appConfigStore.portalShowPath">
          {{ content || props.content }}
        </template>
      </div>
    </div>

    <!-- actions -->
    <div class="flex items-center gap-2">
      <n-button type="success" size="tiny" class="h-8! flex-1! rounded-lg!" @click="open">
        <template #icon>
          <Icon :name="isDirectory ? 'icon-wj-wjj' : 'icon-waibulianjie'" size="14" />
        </template>

        {{ info.actionText }}
        <!-- <template v-if="appConfigStore.portalShowShortcut">(Home)</template> -->
      </n-button>

      <n-button
        v-if="isDirectory"
        type="primary"
        size="tiny"
        class="h-8! flex-1! rounded-lg!"
        @click="openDirInManager"
      >
        <template #icon>
          <Icon name="icon-dakaisuozaiwenjianjia" size="12" />
        </template>
        {{ t("clipboard.openInExplorer") }}
        <!-- <template v-if="appConfigStore.portalShowShortcut">(PageUp)</template> -->
      </n-button>

      <n-button
        v-if="isDirectory"
        color="#1F1F1F"
        text-color="#67C23A"
        size="tiny"
        class="h-8! flex-1! rounded-lg!"
        @click="openDirTerminal"
      >
        <template #icon>
          <Icon name="icon-minglinghang" size="12" />
        </template>
        {{ t("clipboard.openInTerminal") }}
        <!-- <template v-if="appConfigStore.portalShowShortcut">(PageUp)</template> -->
      </n-button>
    </div>
    <!-- <div class="flex items-center gap-2"></div> -->

    <div
      v-if="appConfigStore.portalShowProgress"
      class="absolute bottom-0 left-0 h-1.25 w-full overflow-hidden bg-transparent"
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
import { listen } from "@tauri-apps/api/event";
import { TrayIcon } from "@tauri-apps/api/tray";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { useDebounceFn, useTimeoutFn } from "@vueuse/core";
import { computed, onMounted, onUnmounted, ref } from "vue";
import { exeCommand, openDirInTerminal, openRevealManager, setDefaultTrayIcon } from "@/api";
import { useAppConfig } from "@/composables";
import { AppEvent, PortalNotifyMode } from "@/constant";
import { t } from "@/i18n";
import { sleep } from "@/utils/delay";
import { EventBus } from "@/utils/eventBus";
import { register, unRegisterShortcutKey } from "@/utils/shortcutKey";

interface Props {
  model?: "default" | "setLocation" | "demo";
  visible?: boolean;
  content?: string;
  type?: ClipboardContentType;
}

const props = withDefaults(defineProps<Props>(), { model: "default", visible: false });

const { appConfigStore } = useAppConfig();
const type = ref<ClipboardContentType>(props.type || "Url");
const content = ref<string>(props.content || "");
const visible = ref(props.visible);
const isDirectory = computed(() => type.value === "Directory");
// const model = ref<Props['model']>();
const currentModel = ref<Props["model"]>(props.model || "default");
const isDefaultModel = computed(() => currentModel.value === "default");

const themeColor = computed(() => {
  return isDirectory.value
    ? {
        light: "#8b5cf6",
        bg: "from-violet-500/15 to-fuchsia-500/10",
        button: "#8b5cf6",
      }
    : {
        light: "#2563eb",
        bg: "from-blue-500/15 to-cyan-500/10",
        button: "#2563eb",
      };
});

const info = computed(() => {
  if (currentModel.value === "setLocation") {
    return {
      title: t("clipboard.setWindowPosition"),
      actionText: t("clipboard.confirm"),
    };
  }
  return isDirectory.value
    ? {
        title: t("clipboard.dirCopied"),
        actionText: t("common.openPlain"),
      }
    : {
        title: t("clipboard.linkCopied"),
        actionText: t("common.openPlain"),
      };
});

const currentWindow = getCurrentWebviewWindow();
// const appConfigStore.portalOpenShortcutKey = 'Ctrl + Insert';
// const appConfigStore.portalOpenDirInManagerShortcutKey = 'Ctrl + Home';
// const appConfigStore.portalOpenDirInTerminalShortcutKey = 'Ctrl + PageUp';
const shortcutRegistered = ref(false);

const { start: startAutoClose, stop: stopAutoClose } = useTimeoutFn(
  handleClose,
  appConfigStore.portalDuration,
);

async function handleClose() {
  if (currentModel.value === "demo") return;
  if (currentModel.value === "setLocation") {
    await currentWindow?.hide();
    visible.value = false;
  } else if (appConfigStore.portalNotifyMode === "tray") {
    handleEndFlashTray();
  } else if (appConfigStore.portalNotifyMode === "window") {
    if (!visible.value) return;
    stopAutoClose();
    await currentWindow?.hide();
    visible.value = false;
  }

  if (shortcutRegistered.value) {
    unRegisterShortcutKey(appConfigStore.portalOpenShortcutKey);
    unRegisterShortcutKey(appConfigStore.portalOpenDirInManagerShortcutKey);
    unRegisterShortcutKey(appConfigStore.portalOpenDirInTerminalShortcutKey);
    shortcutRegistered.value = false;
  }
}

const animationKey = ref(0);
async function handleShowWindow() {
  animationKey.value++;
  await currentWindow?.show();
  await handleRegisterShortcutKey();
  stopAutoClose();
  startAutoClose();
  visible.value = true;
}

const flashFlag = ref(false);
const { start: startFlashTimeout, stop: stopFlashTimeout } = useTimeoutFn(
  handleEndFlashTray,
  appConfigStore.portalDuration,
);
const flashLock = ref(false);
// 托盘闪烁通知
async function handleFlashTray() {
  const tray = await TrayIcon.getById("tray");
  flashFlag.value = true;
  stopFlashTimeout();
  startFlashTimeout();

  handleRegisterShortcutKey();
  if (flashLock.value) return;
  while (flashFlag.value) {
    flashLock.value = true;
    tray?.setIcon(null);
    await sleep(500);
    await setDefaultTrayIcon();
    tray?.setTooltip(info.value.title);
    await sleep(500);
  }
}
// 托盘通知完毕
async function handleEndFlashTray() {
  flashFlag.value = false;
  stopFlashTimeout();
  setDefaultTrayIcon();
  const tray = await TrayIcon.getById("tray");
  tray?.setTooltip(appConfigStore.title);
  flashLock.value = false;
}

async function handleRegisterShortcutKey() {
  if (shortcutRegistered.value || !appConfigStore.portalEnableShortcut) return;
  await Promise.all([
    unRegisterShortcutKey(appConfigStore.portalOpenShortcutKey),
    unRegisterShortcutKey(appConfigStore.portalOpenDirInManagerShortcutKey),
    unRegisterShortcutKey(appConfigStore.portalOpenDirInTerminalShortcutKey),
  ]);

  register(appConfigStore.portalOpenShortcutKey, open);

  if (isDirectory.value) {
    register(appConfigStore.portalOpenDirInManagerShortcutKey, openDirInManager);
    register(appConfigStore.portalOpenDirInTerminalShortcutKey, openDirTerminal);
  }
  shortcutRegistered.value = true;
}

async function open() {
  if (currentModel.value === "setLocation") return handleClose();
  if (!content.value) return;
  if (!isDefaultModel.value) return;
  const openUrl = appConfigStore.portalBrowser
    ? `${appConfigStore.portalBrowser} ${content.value}`
    : content.value;
  await exeCommand(openUrl);
  handleClose();
}
async function openDirInManager() {
  if (!isDirectory.value) return;
  if (!isDefaultModel.value) return;
  await openRevealManager(content.value);
  handleClose();
}

async function openDirTerminal() {
  if (!isDirectory.value) return;
  if (!isDefaultModel.value) return;
  // const command = `cdr ${content.value}`;
  // const command = `start cmd /k "cd /d ${content.value}"`;
  // await exeCommand(command);
  await openDirInTerminal(content.value);
  handleClose();
}

let unlistenClipboard: (() => void) | null = null;

const savePortalPosition = useDebounceFn((position: { x: number; y: number }) => {
  appConfigStore.portalWindowPositionX = position.x;
  appConfigStore.portalWindowPositionY = position.y;
}, 100);

currentWindow.onMoved(({ payload: position }) => savePortalPosition(position));
EventBus.listen(AppEvent.OPEN_CLIPBOARD_WINDOW_BY_SET_LOCATION_MODAL, async () => {
  currentModel.value = "setLocation";
  content.value = t("clipboard.dragToSetPosition");
  visible.value = true;
  await currentWindow?.show();
});

onMounted(async () => {
  if (isDefaultModel.value) {
    unlistenClipboard = await listen(
      AppEvent.CLIPBOARD,
      async ({ payload }: { payload: ClipboardPayload }) => {
        if (!appConfigStore.portalEnabled) return;
        // 判断 portalNotifyMode 是什么方式
        const { content: str, content_type } = payload;
        if (content_type === "Unknown") return;
        currentModel.value = "default";
        content.value = str;
        type.value = content_type;

        switch (appConfigStore.portalNotifyMode) {
          case PortalNotifyMode.WINDOW:
            handleShowWindow();
            break;
          case PortalNotifyMode.TRAY:
            handleFlashTray();
            break;
          case PortalNotifyMode.SILENT:
            handleRegisterShortcutKey();
            break;
        }
      },
    );
  }
});

onUnmounted(() => {
  if (isDefaultModel.value) {
    unRegisterShortcutKey(appConfigStore.portalOpenShortcutKey);
    unRegisterShortcutKey(appConfigStore.portalOpenDirInManagerShortcutKey);
    unRegisterShortcutKey(appConfigStore.portalOpenDirInTerminalShortcutKey);
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
