<template>
  <div class="flex flex-col gap-4 p-4">
    <SettingGroup title="启用">
      <SettingSwitchItem
        v-model="appConfigStore.enableTranslation"
        icon="icon-fanyi"
        title="启用翻译"
        description="开启快捷翻译功能"
      >
        <OpenDemoVideo video-url="https://www.bilibili.com/video/BV1fnFKzAEC4" />
      </SettingSwitchItem>
    </SettingGroup>

    <SettingGroup title="特殊呼出">
      <SettingItem
        title="触发方式"
        description="处于默认搜索模式下，快速唤起翻译的操作方式"
      >
        <n-select
          v-model:value="appConfigStore.translationOpenModel"
          size="small"
          :options="options"
          :consistent-menu-width="false"
        />
      </SettingItem>
    </SettingGroup>

    <SettingGroup
      title="百度翻译"
      description="配置百度翻译 API 凭证"
    >
      <SettingItem
        title="目标语言"
        description="翻译结果的目标语言"
      >
        <n-select
          v-model:value="appConfigStore.BDTranslationTo"
          size="small"
          :consistent-menu-width="false"
          :options="targetLanguageOptions"
        />
      </SettingItem>

      <SettingItem
        icon="icon-hashjinghao"
        title="APP ID <span class='text-destructive'>*</span>"
        description="百度翻译应用 ID"
      >
        <n-input
          v-model:value="appConfigStore.BDTranslationAppid"
          size="small"
          type="password"
          show-password-toggle
          placeholder=""
        />
      </SettingItem>

      <SettingItem
        icon="icon-miyue"
        title="密钥 <span class='text-destructive'>*</span>"
        description="百度翻译应用密钥"
      >
        <n-input
          v-model:value="appConfigStore.BDTranslationKey"
          size="small"
          type="password"
          show-password-toggle
          placeholder=""
        />
      </SettingItem>

      <div class="flex-s-c gap-0.5">
        <Icon
          name="icon-waibulianjie"
          size="12"
          class="text-primary"
        />
        <a
          target="_blank"
          href="https://fanyi-api.baidu.com/product/113"
          class="inline-flex items-center gap-1 text-[10px] text-primary hover:underline"
        >
          API 文档
        </a>
      </div>
    </SettingGroup>
  </div>
</template>

<script setup lang="ts">
import OpenDemoVideo from '@/components/OpenDemoVideo.vue';
import { useAppConfig } from '@/composables';
import { BAIDU_TRANSLATION_TO, TranslationOpenModel } from '@/constant';

const { appConfigStore } = useAppConfig();
const options: any[] = [
  { label: '快速点击 3 次空格', value: TranslationOpenModel.THREE_HITS_ON_SPACES },
  { label: '关闭', value: TranslationOpenModel.CLOSE },
];

const targetLanguageOptions = BAIDU_TRANSLATION_TO as any;
</script>
