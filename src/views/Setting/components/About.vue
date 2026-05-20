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
          name="icon-waibulianjie"
          class="cursor-pointer"
          @click="handleOpenGitHub"
        />
      </SettingItem>

      <SettingItem
        icon="icon-kaiyuanxieyi"
        title="开源许可"
      >
        <!-- description="MIT license" -->
        <Icon
          name="icon-waibulianjie"
          class="cursor-pointer"
          @click="openUrl('https://github.com/Peng1203/QuickLauncher?tab=License-1-ov-file')"
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

    <!-- Update Info -->
    <div
      v-if="hasUpdate"
      ref="updateInfoRef"
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

const updateInfoRef = useTemplateRef('updateInfoRef');

async function handleCheckUpdate() {
  await checkUpdate();
  await nextTick();
  updateInfoRef.value?.scrollIntoView({ behavior: 'smooth', block: 'center' });
}

onMounted(() => fetchVersion());
</script>
