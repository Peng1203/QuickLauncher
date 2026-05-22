<template>
  <div class="flex flex-col gap-4 p-4">
    <!-- Header -->
    <div class="flex flex-col items-center gap-3 py-4">
      <img
        src="@/assets/icon.png"
        alt="Quick Launcher"
        class="w-16 h-16"
      />
      <div class="flex flex-col items-center gap-0.5">
        <h2 class="text-base font-semibold text-gray-800">Quick Launcher</h2>
        <p class="text-xs text-muted-foreground">版本 v{{ version }}</p>
      </div>
    </div>

    <!-- About -->
    <SettingGroup
      title="关于"
      description="应用相关信息与链接"
    >
      <SettingItem
        icon="icon-wangluo"
        title="官方网站"
        description=""
      >
        <Icon
          link
          name="icon-waibulianjie"
          class="cursor-pointer"
        />
      </SettingItem>

      <SettingItem
        icon="icon-github-fill"
        title="GitHub"
      >
        <!-- :description="GITHUB_URL" -->
        <Icon
          link
          name="icon-waibulianjie"
          @click="openUrl('https://github.com/Peng1203/QuickLauncher')"
        />
      </SettingItem>

      <SettingItem
        icon="icon-kaiyuanxieyi"
        title="开源许可"
      >
        <!-- description="MIT license" -->
        <Icon
          link
          name="icon-waibulianjie"
          @click="openUrl('https://github.com/Peng1203/QuickLauncher?tab=License-1-ov-file')"
        />
      </SettingItem>

      <SettingItem
        icon="icon-wodefankui"
        title="功能/bug反馈"
      >
        <Icon
          link
          name="icon-waibulianjie"
          @click="openUrl('https://github.com/Peng1203/QuickLauncher/issues')"
        />
      </SettingItem>
    </SettingGroup>

    <SettingGroup title="更新">
      <SettingItem
        icon="icon-gengxinrizhi"
        title="更新日志"
        description="查看最新版本的更新内容"
      >
        <Icon
          link
          name="icon-waibulianjie"
          @click="openUrl('https://github.com/Peng1203/QuickLauncher/releases')"
        />
      </SettingItem>

      <SettingItem
        icon="icon-banbengengxin"
        title="版本更新"
        description="检查是否有新版本可用"
      >
        <!-- :loading="isChecking" -->
        <n-button
          size="small"
          type="info"
          :disabled="isChecking"
          @click="handleCheckUpdate"
        >
          <template #icon>
            <Icon
              :class="isChecking ? 'animate-spin' : ''"
              name="icon-jianchagengxin"
              size="16"
            />
          </template>
          {{ isChecking ? '检查中' : '检查更新' }}
        </n-button>
      </SettingItem>
    </SettingGroup>

    <div
      v-if="hasUpdate"
      class="flex flex-col gap-3 overflow-hidden rounded-2xl border bg-white p-5"
    >
      <!-- 更新信息 -->
      <div class="flex gap-4">
        <Icon
          background
          name="icon-xiazai"
          size="22"
          :color="UpdateSetupColorMap[updateSetup!]"
        />

        <div class="flex flex-col flex-1">
          <div class="flex-sb-c">
            <h3 class="text-[13px] font-semibold text-gray-900 tracking-tight">发现新版本</h3>

            <div class="flex-sb-c gap-2 text-gray-400">
              <Icon
                name="icon-faburiqi"
                size="12"
              />
              <div>{{ getFromNow(updateInfo?.date) }}</div>
            </div>
          </div>

          <p class="text-[12px] text-gray-400">
            <span>v{{ version }}</span>
            →
            <span class="font-semibold text-blue-500">v{{ updateInfo?.version }}</span>
          </p>
          <!-- <div class="flex-sb-c"></div> -->
        </div>
      </div>

      <!-- 下载更新进度 -->
      <template v-if="updateSetup === UpdateSetup.DOWNLOADING">
        <div class="flex flex-col gap-1">
          <div class="flex-sb-c">
            <div class="flex items-center justify-end gap-1">
              <span class="text-[11px] font-medium text-gray-400 tabular-nums">
                {{ formatBytes(downloadTotalBytes) }}
              </span>
              <span class="text-[11px] font-medium text-gray-400">/</span>
              <span class="text-[11px] font-medium text-gray-400 tabular-nums">
                {{ formatBytes(fileTotalBytes) }}
              </span>
            </div>

            <!-- 已下载百分百 -->
            <div class="text-[11px] font-medium text-blue-500">{{ Math.round(downloadProgress) }}%</div>
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
          <div class="flex-1 rounded-2xl border border-gray-100 bg-[#f8f8fb] px-3 py-2">
            <div class="mb-1 flex items-center gap-2 text-sm text-gray-500">
              <Icon
                size="16"
                name="icon-shandian"
                color="oklch(62.3% 0.214 259.815)"
              />
              <span>下载速度</span>
            </div>

            <div class="font-semibold text-black">
              {{ formatBytes(currentSpeedBytes) }}/s
              <!-- <span class="font-medium"></span> -->
            </div>
          </div>

          <div class="flex-1 rounded-2xl border border-gray-100 bg-[#f8f8fb] px-3 py-2">
            <div class="mb-1 flex items-center gap-2 text-sm text-gray-500">
              <Icon
                name="icon-panfu"
                color="oklch(62.7% 0.265 303.9)"
              />
              <span>剩余大小</span>
            </div>

            <div class="font-semibold text-black">
              {{ formatBytes(fileTotalBytes - downloadTotalBytes) }}
              <!-- <span class="font-medium">MB</span> -->
            </div>
          </div>

          <div class="flex-1 rounded-2xl border border-gray-100 bg-[#f8f8fb] px-3 py-2">
            <div class="mb-1 flex items-center gap-2 text-sm text-gray-500">
              <Icon
                size="16"
                name="icon-shijian"
                color="oklch(70.5% 0.213 47.604)"
              />
              <span>预计剩余</span>
            </div>

            <div class="font-semibold text-black">
              {{ currentSpeedBytes > 0 ? Math.ceil((fileTotalBytes - downloadTotalBytes) / currentSpeedBytes) : 0 }}
              <span class="font-medium">秒</span>
            </div>
          </div>
        </div>
      </template>

      <!-- 更新完成提示 -->
      <div
        v-if="isDownloaded"
        class="flex items-center gap-2 rounded-xl border border-emerald-200 bg-emerald-50 px-4 py-3 text-emerald-700 text-xs"
      >
        <Icon
          name="icon-xiazaiwancheng"
          size="14"
        />

        <span>文件已下载完成，点击“立即安装”以应用更新，应用将重新启动。</span>
      </div>

      <!-- 下载取消提示 -->
      <div
        v-if="isCancelled"
        class="flex items-center gap-2 rounded-xl border border-slate-200 bg-slate-50 px-4 py-3 text-slate-600 text-xs"
      >
        <Icon
          name="icon-gantanhao-xianxingyuankuang"
          size="15"
          class="text-slate-500"
        />

        <span>下载已被取消，如需重新下载，请点击重新下载按钮。</span>
      </div>

      <!-- 更新失败提示 -->
      <div
        v-if="isError"
        class="rounded-xl border border-red-200 bg-red-50 px-4 py-3 text-xs"
      >
        <!-- 第一行：图标 + 标题 -->
        <div class="flex items-center gap-2 text-red-700 font-medium">
          <Icon
            name="icon-sanjiaogantan"
            size="14"
            class="text-red-500"
          />
          <span>下载失败</span>
        </div>

        <!-- 第二行：中性错误说明 -->
        <div class="mt-1 text-slate-600 leading-5">网络连接中断或服务器响应异常，请检查网络后重试</div>

        <!-- 第三行：错误码 + 具体错误 -->
        <div class="mt-1 text-slate-500">
          <span>错误代码：</span>
          <span class="text-red-600 font-mono wrap-break-word">
            {{ errorMessage || 'UNKNOWN_ERROR' }}
          </span>
        </div>
      </div>

      <!-- 更新日志 -->
      <div v-if="updateInfo?.body">
        <div class="rounded-xl border-gray-100 bg-[#f8f8fb] p-3.5 max-h-32.5 overflow-y-auto">
          <p class="text-[11px] leading-relaxed text-gray-400 whitespace-pre-wrap">
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
          取消下载
        </n-button>

        <n-button
          v-show="isCancelled || isError"
          quaternary
          @click="() => initUpdateInfo()"
        >
          <template #icon>
            <Icon
              name="icon-guanbichuangkou"
              size="14"
            />
          </template>
          取消更新
        </n-button>

        <n-button
          v-show="isAvailable || isDownloading"
          :disabled="isDownloading"
          class="flex-1"
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

          {{ isDownloading ? `等待下载完成...${Math.round(downloadProgress)}%` : '立即下载' }}
        </n-button>

        <n-button
          v-show="isAvailable"
          quaternary
          @click="() => initUpdateInfo()"
        >
          <template #icon>
            <Icon
              name="icon-lingdang"
              size="14"
            />
          </template>
          稍后
        </n-button>

        <n-button
          v-show="isCancelled || isError"
          class="flex-1"
          :color="UpdateSetupColorMap[UpdateSetup.CANCELLED]"
          @click="handleDownload"
        >
          <template #icon>
            <Icon
              name="icon-zhongqi1"
              size="14"
            />
          </template>
          重新下载
        </n-button>

        <n-button
          v-show="isDownloaded"
          class="flex-1"
          :color="UpdateSetupColorMap[UpdateSetup.DOWNLOADED]"
          @click="handleInstall"
        >
          <template #icon>
            <Icon
              name="icon-xiazaiwancheng"
              size="14"
            />
          </template>
          立即安装
        </n-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Update } from '@tauri-apps/plugin-updater';
import { openUrl } from '@tauri-apps/plugin-opener';
import { check } from '@tauri-apps/plugin-updater';
import { useAppVersion, useNaiveUiApi } from '@/composables';
import { getFromNow } from '@/utils/date';
import { formatBytes } from '@/utils/formatBytes';

const { version, isChecking, updateInfo, hasUpdate, fetchVersion, checkUpdate } = useAppVersion();
const { notification } = useNaiveUiApi();

const updateSetup = ref<UpdateSetup | null>(null);

enum UpdateSetup {
  // eslint-disable-next-line no-unused-vars
  AVAILABLE = 'available', // 检查到新版本 展示新版本相关页面
  // eslint-disable-next-line no-unused-vars
  DOWNLOADING = 'downloading', // 下载中 展示下载进度等相关页面
  // eslint-disable-next-line no-unused-vars
  CANCELLED = 'cancelled', // 下载过程 点击了取消下载 展示重新下载相关页面
  // eslint-disable-next-line no-unused-vars
  DOWNLOADED = 'downloaded', // 下载完成 展示安装相关页面
  // eslint-disable-next-line no-unused-vars
  ERROR = 'error', // 下载过程中发生错误 展示重新下载相关页面
}

const UpdateSetupColorMap = {
  [UpdateSetup.AVAILABLE]: '#155dfc',
  [UpdateSetup.DOWNLOADING]: '#f97316',
  [UpdateSetup.CANCELLED]: '#6b7280',
  [UpdateSetup.DOWNLOADED]: '#009966',
  [UpdateSetup.ERROR]: '#f14444',
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

const errorMessage = ref('');

async function handleDownload() {
  try {
    initUpdateInfo(true);
    currentUpdate = await check();
    if (!currentUpdate) return;
    updateSetup.value = UpdateSetup.DOWNLOADING;
    scrollToBottom();

    currentUpdate
      .download(
        e => {
          // console.log('download e', { ...e });
          // 下载进度
          const { event } = e;
          switch (event) {
            case 'Started':
              fileTotalBytes.value = e.data.contentLength!;
              break;
            case 'Progress':
              currentSpeedBytes.value = e.data.chunkLength * 1024;
              downloadTotalBytes.value += e.data.chunkLength;
              break;
            case 'Finished':
              // 由于下载无法中断 所以即使用户点击了取消下载 当判断到下载时用户点击了取消下载 在下载完成时不触发下载完成状态
              if (updateSetup.value !== UpdateSetup.CANCELLED) updateSetup.value = UpdateSetup.DOWNLOADED;
              break;
          }
        },
        // 设置下载超时时间为3分钟
        { timeout: 1000 * 60 * 3 },
      )
      .catch(err => {
        // 设置下载状态为错误
        updateSetup.value = UpdateSetup.ERROR;
        errorMessage.value = err;
      });
  } catch (e) {
    notification.error({
      title: '下载更新失败',
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
  updateSetup.value = UpdateSetup.CANCELLED;
}

function initUpdateInfo(downloadBeforeInit = false) {
  currentUpdate?.close();
  currentUpdate = null;

  if (!downloadBeforeInit) {
    updateSetup.value = null;
    updateInfo.value = null;
  }
  fileTotalBytes.value = 0;
  downloadTotalBytes.value = 0;
  currentSpeedBytes.value = 0;
  errorMessage.value = '';
}

// 滚动到最底部
function scrollToBottom() {
  requestAnimationFrame(() => {
    const el = document.querySelector('.n-tab-pane');
    if (!el) return;
    el.scrollTo({
      top: 9999999,
      behavior: 'smooth',
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
