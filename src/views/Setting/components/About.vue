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

    <!-- Update Info -->
    <div
      v-if="hasUpdate"
      class="flex flex-col gap-2 rounded border border-blue-200 bg-blue-50 p-3"
    >
      <div class="flex items-center gap-1.5">
        <n-icon
          size="16"
          color="#2563eb"
        >
          <CloudDownloadOutline />
        </n-icon>
        <span class="text-sm font-medium text-blue-700">发现新版本 v{{ updateInfo?.version }}</span>
      </div>
      <p
        v-if="updateInfo?.body"
        class="text-xs text-blue-600 whitespace-pre-wrap"
      >
        {{ updateInfo.body }}
      </p>
      <n-button
        size="small"
        type="primary"
        @click="downloadAndInstall"
      >
        下载并安装
      </n-button>
    </div>

    <!-- About -->
    <SettingGroup
      title="关于"
      description="应用相关信息与链接"
    >
      <SettingItem
        icon="icon-wangluo"
        title="官方网站"
        :description="GITHUB_URL"
      >
        <Icon
          name="icon-waibulianjie"
          class="cursor-pointer"
        />
      </SettingItem>

      <SettingItem
        icon="icon-github-fill"
        title="GitHub"
        :description="GITHUB_URL"
      >
        <Icon
          name="icon-waibulianjie"
          class="cursor-pointer"
          @click="handleOpenGitHub"
        />
      </SettingItem>

      <SettingItem
        icon="icon-kaiyuanxieyi"
        title="开源许可"
        description="MIT license"
      >
        <Icon
          name="icon-waibulianjie"
          class="cursor-pointer"
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
          name="icon-waibulianjie"
          class="cursor-pointer"
        />
      </SettingItem>

      <SettingItem
        icon="icon-banbengengxin"
        title="版本更新"
        :description="isChecking ? '正在检查更新...' : hasUpdate ? `发现新版本 v${updateInfo?.version}` : '检查是否有新版本可用'"
      >
        <n-button
          size="small"
          type="info"
          :loading="isChecking"
          @click="checkUpdate"
        >
          <template #icon>
            <Icon
              name="icon-jianchagengxin"
              size="16"
            />
          </template>
          {{ hasUpdate ? '重新检查' : '检查更新' }}
        </n-button>
      </SettingItem>
    </SettingGroup>
  </div>
</template>

<script setup lang="ts">
import { openUrl } from '@tauri-apps/plugin-opener';
import { CloudDownloadOutline } from '@vicons/ionicons5';
import { useAppVersion } from '@/composables';

const GITHUB_URL = 'https://github.com/Peng1203/QuickLauncher';

const { version, isChecking, updateInfo, hasUpdate, fetchVersion, checkUpdate, downloadAndInstall } = useAppVersion();

function handleOpenGitHub() {
  openUrl(GITHUB_URL);
}

onMounted(() => fetchVersion());
</script>
