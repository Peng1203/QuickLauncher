<template>
  <div class="flex items-end gap-2">
    <n-button
      size="tiny"
      type="default"
      :title="t('iconPicker.selectDirIcon')"
      @click="handleGetLocalDirIcon"
    >
      <template #icon>
        <n-icon class="iconfont icon-wj-wjj" />
      </template>
    </n-button>

    <n-button
      size="tiny"
      type="default"
      :title="t('iconPicker.selectFileIcon')"
      @click="handleGetLocalFileIcon"
    >
      <template #icon>
        <n-icon class="iconfont icon-wenjian" />
      </template>
    </n-button>

    <n-tooltip placement="bottom" trigger="click" :title="t('iconPicker.networkImage')">
      <template #trigger>
        <n-button size="tiny" type="default" :title="t('iconPicker.networkImage')">
          <template #icon>
            <n-icon>
              <LinkOutline />
            </n-icon>
          </template>
        </n-button>
      </template>
      <div class="text-foreground">输入网络图片地址</div>

      <n-input-group>
        <n-input v-model:value="onlineImgUrl" placeholder="" />
        <n-button
          type="info"
          :loading="onlineImgUrlLoading"
          :disabled="!onlineImgUrl.length"
          @click="handleGetOnlineImg"
        >
          {{ t("iconPicker.get") }}
        </n-button>
      </n-input-group>
    </n-tooltip>

    <n-tooltip placement="bottom" trigger="click" :title="t('iconPicker.websiteIcon')">
      <template #trigger>
        <n-button size="tiny" type="default" :title="t('iconPicker.websiteIcon')">
          <template #icon>
            <n-icon>
              <GlobeOutline />
            </n-icon>
          </template>
        </n-button>
      </template>
      <div class="text-foreground">输入网站地址</div>
      <n-input-group>
        <n-input v-model:value="webSiteUrl" placeholder="" />
        <n-button
          type="info"
          :loading="webSiteUrlLoading"
          :disabled="!webSiteUrl.length"
          @click="handleGetWebSiteUrl"
        >
          {{ t("iconPicker.get") }}
        </n-button>
      </n-input-group>
    </n-tooltip>

    <n-tooltip placement="top" trigger="click" :title="t('iconPicker.svgIcon')">
      <template #trigger>
        <n-button size="tiny" type="default" :title="t('iconPicker.svgIcon')">
          <template #icon>
            <n-icon>
              <CodeOutline />
            </n-icon>
          </template>
        </n-button>
      </template>

      <div class="w-[200px]">
        <div class="text-foreground">输入 SVG 代码</div>
        <n-input
          v-model:value="svgStr"
          type="textarea"
          placeholder=""
          :autosize="{ minRows: 3, maxRows: 5 }"
        />
        <n-button type="info" class="!mt-1" @click="handleGetSvgBase64">
          {{ t("iconPicker.get") }}
        </n-button>
      </div>
    </n-tooltip>

    <n-button
      size="tiny"
      type="default"
      :title="t('iconPicker.resetIcon')"
      @click="handleResetIcon"
    >
      <template #icon>
        <n-icon class="iconfont icon-zhongzhi" />
      </template>
    </n-button>
  </div>
</template>

<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";
import { CodeOutline, GlobeOutline, LinkOutline } from "@vicons/ionicons5";
import { ref } from "vue";
import { getLocalIconBase64, getOnlineImgBase64, getWebsiteInfo } from "@/api";
import { useNaiveUiApi } from "@/composables";
import { t } from "@/i18n";

const { message } = useNaiveUiApi();

const iconValue = defineModel<string>();

async function handleGetLocalFileIcon() {
  const path = await open({
    title: t("iconPicker.selectFileIcon"),
    multiple: false,
    directory: false,
  });
  if (!path) return;
  const base64 = await getLocalIconBase64(path);

  iconValue.value = base64;
}

async function handleGetLocalDirIcon() {
  const path = await open({
    title: t("iconPicker.selectDirIcon"),
    multiple: false,
    directory: true,
  });
  if (!path) return;
  const base64 = await getLocalIconBase64(path);

  iconValue.value = base64;
}

const onlineImgUrl = ref<string>("");
const onlineImgUrlLoading = ref<boolean>(false);
async function handleGetOnlineImg() {
  try {
    onlineImgUrlLoading.value = true;
    if (!(onlineImgUrl.value.includes("http://") || onlineImgUrl.value.includes("https://"))) {
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

const webSiteUrl = ref<string>("");
const webSiteUrlLoading = ref<boolean>(false);
async function handleGetWebSiteUrl() {
  try {
    webSiteUrlLoading.value = true;
    if (!(webSiteUrl.value.includes("http://") || webSiteUrl.value.includes("https://"))) {
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

const svgStr = ref<string>("");
async function handleGetSvgBase64() {
  if (!svgStr.value.trim()) return;
  const base64 = btoa(unescape(encodeURIComponent(svgStr.value)));
  iconValue.value = `data:image/svg+xml;base64,${base64}`;
}

function handleResetIcon() {
  iconValue.value = "";
}
</script>
