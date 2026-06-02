<template>
  <div class="flex flex-col gap-4 p-4">
    <!-- Header -->
    <div class="flex flex-col items-center gap-3 py-4">
      <img src="@/assets/icon.png" alt="Quick Launcher" class="w-16 h-16" />
      <div class="flex flex-col items-center gap-0.5">
        <h2 class="text-base font-semibold text-foreground">Quick Launcher</h2>
        <p class="text-xs text-muted-foreground">版本 v{{ version }}</p>
      </div>
    </div>

    <!-- About -->
    <SettingGroup :title="t('about.groupAbout')" :description="t('about.aboutDesc')">
      <SettingItem icon="icon-wangluo" :title="t('about.officialSite')" description="">
        <Icon link name="icon-waibulianjie" class="cursor-pointer" />
      </SettingItem>

      <SettingItem icon="icon-github-fill" title="GitHub">
        <!-- :description="GITHUB_URL" -->
        <Icon
          link
          name="icon-waibulianjie"
          @click="openUrl('https://github.com/Peng1203/QuickLauncher')"
        />
      </SettingItem>

      <SettingItem icon="icon-kaiyuanxieyi" :title="t('about.openSourceLicense')">
        <!-- description="MIT license" -->
        <Icon
          link
          name="icon-waibulianjie"
          @click="openUrl('https://github.com/Peng1203/QuickLauncher?tab=License-1-ov-file')"
        />
      </SettingItem>

      <SettingItem icon="icon-wodefankui" :title="t('about.feedback')">
        <Icon
          link
          name="icon-waibulianjie"
          @click="openUrl('https://github.com/Peng1203/QuickLauncher/issues')"
        />
      </SettingItem>
    </SettingGroup>

    <SettingGroup :title="t('about.groupUpdate')">
      <SettingItem
        icon="icon-gengxinrizhi"
        :title="t('about.changelog')"
        :description="t('about.changelogDesc')"
      >
        <Icon
          link
          name="icon-waibulianjie"
          @click="openUrl('https://github.com/Peng1203/QuickLauncher/releases')"
        />
      </SettingItem>

      <SettingItem
        icon="icon-banbengengxin"
        :title="t('about.versionUpdate')"
        :description="t('about.versionUpdateDesc')"
      >
        <!-- :loading="isChecking" -->
        <n-button
          size="small"
          type="info"
          :disabled="isChecking || isDownloading"
          @click="handleCheckUpdate"
        >
          <template #icon>
            <Icon :class="isChecking ? 'animate-spin' : ''" name="icon-jianchagengxin" size="16" />
          </template>
          {{ isChecking ? t("about.checking") : t("about.checkUpdate") }}
        </n-button>
      </SettingItem>
    </SettingGroup>

    <div
      v-if="hasUpdate"
      class="flex flex-col gap-3 overflow-hidden rounded-2xl border bg-card border-border p-5"
    >
      <!-- 更新信息 -->
      <div class="flex gap-4">
        <Icon background name="icon-xiazai" size="22" :color="UpdateSetupColorMap[updateSetup!]" />

        <div class="flex flex-col flex-1">
          <div class="flex-sb-c">
            <h3 class="text-[13px] font-semibold text-foreground tracking-tight">
              {{ t("about.newVersion") }}
            </h3>

            <div class="flex-sb-c gap-2 text-muted-foreground">
              <Icon name="icon-faburiqi" size="12" />
              <div>{{ getFromNow(updateInfo?.date) }}</div>
            </div>
          </div>

          <p class="text-[12px] text-muted-foreground">
            <span>v{{ version }}</span>
            →
            <span class="font-semibold text-primary">v{{ updateInfo?.version }}</span>
          </p>
          <!-- <div class="flex-sb-c"></div> -->
        </div>
      </div>

      <!-- 下载更新进度 -->
      <template v-if="updateSetup === UpdateSetup.DOWNLOADING">
        <div class="flex flex-col gap-1">
          <div class="flex-sb-c">
            <div class="flex items-center justify-end gap-1">
              <span class="text-[11px] font-medium text-muted-foreground tabular-nums">
                {{ formatBytes(downloadTotalBytes) }}
              </span>
              <span class="text-[11px] font-medium text-muted-foreground">/</span>
              <span class="text-[11px] font-medium text-muted-foreground tabular-nums">
                {{ formatBytes(fileTotalBytes) }}
              </span>
            </div>

            <!-- 已下载百分百 -->
            <div class="text-[11px] font-medium text-primary">
              {{ Math.round(downloadProgress) }}%
            </div>
          </div>

          <n-progress
            type="line"
            :show-indicator="false"
            :percentage="downloadProgress"
            indicator-placement="outside"
            processing
          />
        </div>

        <!-- 下载信息 -->
        <div class="flex gap-4">
          <div class="flex-1 rounded-2xl border border-border bg-muted px-3 py-2">
            <div class="mb-1 flex items-center gap-2 text-sm text-muted-foreground">
              <Icon size="16" name="icon-shandian" color="oklch(62.3% 0.214 259.815)" />
              <span>{{ t("about.downloadSpeed") }}</span>
            </div>

            <div class="font-semibold text-foreground">
              {{ formatBytes(currentSpeedBytes) }}/s
              <!-- <span class="font-medium"></span> -->
            </div>
          </div>

          <div class="flex-1 rounded-2xl border border-border bg-muted px-3 py-2">
            <div class="mb-1 flex items-center gap-2 text-sm text-muted-foreground">
              <Icon name="icon-panfu" color="oklch(62.7% 0.265 303.9)" />
              <span>{{ t("about.remainingSize") }}</span>
            </div>

            <div class="font-semibold text-foreground">
              {{ formatBytes(fileTotalBytes - downloadTotalBytes) }}
              <!-- <span class="font-medium">MB</span> -->
            </div>
          </div>

          <div class="flex-1 rounded-2xl border border-border bg-muted px-3 py-2">
            <div class="mb-1 flex items-center gap-2 text-sm text-muted-foreground">
              <Icon size="16" name="icon-shijian" color="oklch(70.5% 0.213 47.604)" />
              <span>{{ t("about.estimatedTime") }}</span>
            </div>

            <div class="font-semibold text-foreground">
              <!-- {{ currentSpeedBytes > 0 ? Math.ceil((fileTotalBytes - downloadTotalBytes) / currentSpeedBytes) : 0 }} -->
              {{ formatDuration(remainingSeconds) }}
              <!-- <span class="font-medium">秒</span> -->
            </div>
          </div>
        </div>
      </template>

      <!-- 更新完成提示 -->
      <div
        v-if="isDownloaded"
        class="flex items-center gap-2 rounded-xl border border-primary/20 bg-primary/5 px-4 py-3 text-primary text-xs"
      >
        <Icon name="icon-xiazaiwancheng" size="14" />

        <span>{{ t("about.downloadComplete") }}</span>
      </div>

      <!-- 下载取消提示 -->
      <div
        v-if="isCancelled"
        class="flex items-center gap-2 rounded-xl border border-border bg-muted px-4 py-3 text-muted-foreground text-xs"
      >
        <Icon name="icon-gantanhao-xianxingyuankuang" size="15" class="text-muted-foreground" />

        <span>{{ t("about.downloadCancelled") }}</span>
      </div>

      <!-- 更新失败提示 -->
      <div
        v-if="isError"
        class="rounded-xl border border-destructive/20 bg-destructive/5 px-4 py-3 text-xs"
      >
        <!-- 第一行：图标 + 标题 -->
        <div class="flex items-center gap-2 text-destructive font-medium">
          <Icon name="icon-sanjiaogantan" size="14" class="text-destructive" />
          <span>{{ t("about.downloadFailed") }}</span>
        </div>

        <!-- 第二行：中性错误说明 -->
        <div class="mt-1 text-muted-foreground leading-5">{{ t("about.networkError") }}</div>

        <!-- 第三行：错误码 + 具体错误 -->
        <div class="mt-1 text-muted-foreground">
          <span>{{ t("about.errorCode") }}：</span>
          <span class="text-destructive font-mono wrap-break-word">
            {{ errorMessage || "UNKNOWN_ERROR" }}
          </span>
        </div>
      </div>

      <!-- 更新日志 -->
      <div v-if="updateInfo?.body">
        <div class="rounded-xl border border-border bg-muted p-3.5 max-h-32.5 overflow-y-auto">
          <p class="text-[11px] leading-relaxed text-muted-foreground whitespace-pre-wrap">
            {{ updateInfo.body }}
          </p>
        </div>
      </div>

      <!-- 操作按钮 -->
      <div class="flex-s-c gap-2 operations">
        <!-- v-if="!isDownloading" -->
        <!-- <template v-if="updateSetup === UpdateSetup.AVAILABLE"></template> -->
        <n-button
          v-show="isDownloading || isDownloaded"
          :disabled="isDownloaded"
          @click="handleCancelDownload"
        >
          <template #icon>
            <Icon name="icon-guanbichuangkou" />
          </template>
          {{ t("about.cancelDownload") }}
        </n-button>

        <n-button v-show="isCancelled || isError" quaternary @click="() => initUpdateInfo()">
          <template #icon>
            <Icon name="icon-guanbichuangkou" size="14" />
          </template>
          {{ t("about.cancelUpdate") }}
        </n-button>

        <n-button
          v-show="isAvailable || isDownloading"
          class="flex-1"
          :disabled="isDownloading"
          :color="UpdateSetupColorMap[UpdateSetup.AVAILABLE]"
          @click="handleDownload"
        >
          <template #icon>
            <Icon
              :class="isDownloading ? 'animate-spin' : ''"
              :name="isDownloading ? 'icon-zhongqi1' : 'icon-xiazai'"
              size="14"
            />
          </template>

          {{
            isDownloading
              ? `${t("about.downloadWaiting")}...${Math.round(downloadProgress)}%`
              : t("about.downloadNow")
          }}
        </n-button>

        <n-button v-show="isAvailable" quaternary @click="() => initUpdateInfo()">
          <template #icon>
            <Icon name="icon-lingdang" size="14" />
          </template>
          {{ t("about.later") }}
        </n-button>

        <n-button
          v-show="isCancelled || isError"
          class="flex-1"
          :color="UpdateSetupColorMap[UpdateSetup.CANCELLED]"
          @click="handleDownload"
        >
          <template #icon>
            <Icon name="icon-zhongqi1" size="14" />
          </template>
          {{ t("about.reDownload") }}
        </n-button>

        <n-button
          v-show="isDownloaded"
          class="flex-1"
          :color="UpdateSetupColorMap[UpdateSetup.DOWNLOADED]"
          @click="handleInstall"
        >
          <template #icon>
            <Icon name="icon-xiazaiwancheng" size="14" />
          </template>
          {{ t("about.installNow") }}
        </n-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Update } from "@tauri-apps/plugin-updater";
import { openUrl } from "@tauri-apps/plugin-opener";
import { check } from "@tauri-apps/plugin-updater";
import { useAppVersion, useNaiveUiApi } from "@/composables";
import { t } from "@/i18n";
import { getFromNow } from "@/utils/date";
import { formatBytes, formatDuration } from "@/utils/format";

const { version, isChecking, updateInfo, hasUpdate, fetchVersion, checkUpdate } = useAppVersion();
const { notification } = useNaiveUiApi();

const updateSetup = ref<UpdateSetup | null>(null);

enum UpdateSetup {
  AVAILABLE = "available", // 检查到新版本 展示新版本相关页面

  DOWNLOADING = "downloading", // 下载中 展示下载进度等相关页面

  CANCELLED = "cancelled", // 下载过程 点击了取消下载 展示重新下载相关页面

  DOWNLOADED = "downloaded", // 下载完成 展示安装相关页面

  ERROR = "error", // 下载过程中发生错误 展示重新下载相关页面
}

const UpdateSetupColorMap = {
  // [UpdateSetup.AVAILABLE]: 'var(--info)',
  // [UpdateSetup.DOWNLOADING]: 'var(--info)',
  // [UpdateSetup.CANCELLED]: 'var(--muted-foreground)',
  // [UpdateSetup.DOWNLOADED]: 'var(--success)',
  // [UpdateSetup.ERROR]: 'var(--destructive)',

  [UpdateSetup.AVAILABLE]: "#155dfc",
  [UpdateSetup.DOWNLOADING]: "#155dfc",
  [UpdateSetup.CANCELLED]: "#6b7280",
  [UpdateSetup.DOWNLOADED]: "#009966",
  [UpdateSetup.ERROR]: "#f14444",
};
// const UpdateSetupIconMap = {
//   [UpdateSetup.AVAILABLE]: 'icon-xiazai',
//   [UpdateSetup.DOWNLOADING]: 'icon-xiazai',
//   [UpdateSetup.CANCELLED]: 'icon-xiazai',
//   [UpdateSetup.DOWNLOADED]: 'icon-xiazai',
//   [UpdateSetup.ERROR]: 'icon-xiazai',
// };
const isAvailable = computed(() => updateSetup.value === UpdateSetup.AVAILABLE);
const isDownloaded = computed(() => updateSetup.value === UpdateSetup.DOWNLOADED);
const isCancelled = computed(() => updateSetup.value === UpdateSetup.CANCELLED);
const isError = computed(() => updateSetup.value === UpdateSetup.ERROR);
const isDownloading = computed(() => updateSetup.value === UpdateSetup.DOWNLOADING);

// 下载总大小
const fileTotalBytes = ref<number>(0);
// 已下载字节
const downloadTotalBytes = ref<number>(0);

// 当前下载速度（字节/秒）
const currentSpeedBytes = ref<number>(0);

const downloadProgress = computed(() => {
  if (fileTotalBytes.value === 0) return 0;
  return (downloadTotalBytes.value / fileTotalBytes.value) * 100;
});

// 更新信息 放ref里 方法无法调用
let currentUpdate: Update | null = null;

async function handleCheckUpdate() {
  const hasUp = await checkUpdate();
  if (hasUp === false) return;
  await nextTick();
  scrollToBottom();

  updateSetup.value = UpdateSetup.AVAILABLE;
}

const startTime = ref(0);

const errorMessage = ref("");

const remainingSeconds = computed(() => {
  if (!currentSpeedBytes.value) {
    return 0;
  }

  return Math.ceil((fileTotalBytes.value - downloadTotalBytes.value) / currentSpeedBytes.value);
});

const runTaskId = ref("");
const timeoutTimer = ref();
async function handleDownload() {
  try {
    await initUpdateInfo(true);
    const currentTaskId = crypto.randomUUID();
    runTaskId.value = currentTaskId;
    currentUpdate = await check();
    if (!currentUpdate) return;
    updateSetup.value = UpdateSetup.DOWNLOADING;
    scrollToBottom();

    currentUpdate
      .download(
        (e) => {
          // 当用户在下载过程中点了取消操作 修改 runTaskId 防止后续操作影响状态
          if (currentTaskId !== runTaskId.value) return;
          // 下载进度
          const { event } = e;
          switch (event) {
            case "Started":
              startTime.value = Date.now();
              fileTotalBytes.value = e.data.contentLength!;
              break;
            case "Progress":
              {
                downloadTotalBytes.value += e.data.chunkLength;

                const elapsed = (Date.now() - startTime.value) / 1000;
                currentSpeedBytes.value = downloadTotalBytes.value / elapsed;
              }

              break;
            case "Finished":
              // 由于下载无法中断 所以即使用户点击了取消下载 当判断到下载时用户点击了取消下载 在下载完成时不触发下载完成状态
              if (updateSetup.value !== UpdateSetup.CANCELLED)
                updateSetup.value = UpdateSetup.DOWNLOADED;
              break;
          }
        },
        // 设置下载超时时间为1分钟
        { timeout: 1000 * 60 },
      )
      .catch((err) => {
        if (currentTaskId !== runTaskId.value) return;
        // 设置下载状态为错误
        updateSetup.value = UpdateSetup.ERROR;
        errorMessage.value = err;
      });

    // 手动添加定时器
    timeoutTimer.value = setTimeout(() => {
      // 设置下载状态为错误
      updateSetup.value = UpdateSetup.ERROR;
      errorMessage.value = "Download timeout.";
    }, 1000 * 60);
  } catch (e) {
    notification.error({
      title: t("about.downloadUpdateFailed"),
      description: `${e}`,
      duration: 5000,
    });
    updateSetup.value = UpdateSetup.ERROR;
  }
}

function handleInstall() {
  currentUpdate?.install();
}

function handleCancelDownload() {
  if (!currentUpdate) return;
  currentUpdate?.close();
  currentUpdate = null;
  runTaskId.value = "";
  updateSetup.value = UpdateSetup.CANCELLED;
}

async function initUpdateInfo(downloadBeforeInit = false) {
  await currentUpdate?.close();
  currentUpdate = null;

  if (!downloadBeforeInit) {
    updateSetup.value = null;
    updateInfo.value = null;
  }
  runTaskId.value = "";
  if (timeoutTimer.value) clearTimeout(timeoutTimer.value);
  timeoutTimer.value = null;
  fileTotalBytes.value = 0;
  downloadTotalBytes.value = 0;
  currentSpeedBytes.value = 0;
  errorMessage.value = "";
}

// 滚动到最底部
function scrollToBottom() {
  requestAnimationFrame(() => {
    const el = document.querySelector(".n-tab-pane");
    if (!el) return;
    el.scrollTo({
      top: 9999999,
      behavior: "smooth",
    });
  });
}

onMounted(() => fetchVersion());

onUnmounted(() => initUpdateInfo());
</script>

<style lang="scss" scoped>
.operations {
  .n-button {
    border-radius: 8px;
  }
}
</style>
