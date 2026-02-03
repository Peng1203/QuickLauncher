<template>
  <div class="flex items-end gap-2">
    <n-button
      size="tiny"
      type="default"
      title="选择目录图标"
      @click="handleGetLocalDirIcon"
    >
      <template #icon>
        <n-icon class="iconfont icon-wj-wjj" />
      </template>
    </n-button>

    <n-button
      size="tiny"
      type="default"
      title="选择文件图标"
      @click="handleGetLocalFileIcon"
    >
      <template #icon>
        <n-icon class="iconfont icon-wenjian" />
      </template>
    </n-button>

    <n-tooltip
      placement="bottom"
      trigger="click"
      title="网络图片"
    >
      <template #trigger>
        <n-button
          size="tiny"
          type="default"
          title="网络图片"
        >
          <template #icon>
            <n-icon>
              <LinkOutline />
            </n-icon>
          </template>
        </n-button>
      </template>
      <div class="text-gray-700">输入网络图片地址</div>

      <n-input-group>
        <n-input
          v-model:value="onlineImgUrl"
          placeholder=""
        />
        <n-button
          type="info"
          :loading="onlineImgUrlLoading"
          :disabled="!onlineImgUrl.length"
          @click="handleGetOnlineImg"
        >
          获 取
        </n-button>
      </n-input-group>
    </n-tooltip>

    <n-tooltip
      placement="bottom"
      trigger="click"
      title="网站图标"
    >
      <template #trigger>
        <n-button
          size="tiny"
          type="default"
          title="网站图标"
        >
          <template #icon>
            <n-icon>
              <GlobeOutline />
            </n-icon>
          </template>
        </n-button>
      </template>
      <div class="text-gray-700">输入网站地址</div>
      <n-input-group>
        <n-input
          v-model:value="webSiteUrl"
          placeholder=""
        />
        <n-button
          type="info"
          :loading="webSiteUrlLoading"
          :disabled="!webSiteUrl.length"
          @click="handleGetWebSiteUrl"
        >
          获 取
        </n-button>
      </n-input-group>
    </n-tooltip>

    <n-tooltip
      placement="top"
      trigger="click"
      title="SVG 图标"
    >
      <template #trigger>
        <n-button
          size="tiny"
          type="default"
          title="SVG 图标"
        >
          <template #icon>
            <n-icon>
              <CodeOutline />
            </n-icon>
          </template>
        </n-button>
      </template>

      <div class="w-[200px]">
        <div class="text-gray-700">输入 SVG 代码</div>
        <n-input
          v-model:value="svgStr"
          type="textarea"
          placeholder=""
          :autosize="{ minRows: 3, maxRows: 5 }"
        />
        <n-button
          type="info"
          class="!mt-1"
          @click="handleGetSvgBase64"
        >
          获 取
        </n-button>
      </div>
    </n-tooltip>

    <n-button
      size="tiny"
      type="default"
      title="重置图标"
      @click="handleResetIcon"
    >
      <template #icon>
        <n-icon class="iconfont icon-zhongzhi" />
      </template>
    </n-button>
  </div>
</template>

<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog';
import { CodeOutline, GlobeOutline, LinkOutline } from '@vicons/ionicons5';
import { ref } from 'vue';
import { getLocalIconBase64, getOnlineImgBase64, getWebsiteInfo } from '@/api';
import { useNaiveUiApi } from '@/composables/useNaiveUiApi';

const { message } = useNaiveUiApi();

const iconValue = defineModel<string>();

async function handleGetLocalFileIcon() {
  const path = await open({
    title: '选择文件图标',
    multiple: false,
    directory: false,
  });
  if (!path) return;
  const base64 = await getLocalIconBase64(path);

  iconValue.value = base64;
}

async function handleGetLocalDirIcon() {
  const path = await open({
    title: '选择目录图标',
    multiple: false,
    directory: true,
  });
  if (!path) return;
  const base64 = await getLocalIconBase64(path);

  iconValue.value = base64;
}

const onlineImgUrl = ref<string>('');
const onlineImgUrlLoading = ref<boolean>(false);
async function handleGetOnlineImg() {
  try {
    onlineImgUrlLoading.value = true;
    if (!(onlineImgUrl.value.includes('http://') || onlineImgUrl.value.includes('https://'))) {
      onlineImgUrl.value = `https://${onlineImgUrl.value}`;
    }
    const base64 = await getOnlineImgBase64(onlineImgUrl.value);
    iconValue.value = base64;
  } catch (e) {
    message.error(e as string);
  } finally {
    onlineImgUrlLoading.value = false;
  }
}

const webSiteUrl = ref<string>('');
const webSiteUrlLoading = ref<boolean>(false);
async function handleGetWebSiteUrl() {
  try {
    webSiteUrlLoading.value = true;
    if (!(webSiteUrl.value.includes('http://') || webSiteUrl.value.includes('https://'))) {
      webSiteUrl.value = `https://${webSiteUrl.value}`;
    }
    const { icon }: any = await getWebsiteInfo(webSiteUrl.value);
    iconValue.value = icon;
  } catch (e) {
    message.error(e as string);
  } finally {
    webSiteUrlLoading.value = false;
  }
}

const svgStr = ref<string>('');
async function handleGetSvgBase64() {
  if (!svgStr.value.trim()) return;
  const base64 = btoa(unescape(encodeURIComponent(svgStr.value)));
  iconValue.value = `data:image/svg+xml;base64,${base64}`;
}

function handleResetIcon() {
  iconValue.value = '';
}
</script>
