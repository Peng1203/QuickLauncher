<template>
  <div class="flex flex-col gap-4 p-4">
    <SettingGroup :title="t('quickSearch.groupEnable')">
      <SettingSwitchItem
        v-model="appConfigStore.enableTranslation"
        icon="icon-fanyi"
        :title="t('translation.enableTitle')"
        :description="t('translation.enableDesc')"
      >
        <OpenDemoVideo video-url="https://www.bilibili.com/video/BV1fnFKzAEC4" />
      </SettingSwitchItem>
    </SettingGroup>

    <SettingGroup :title="t('translation.groupTrigger')">
      <SettingItem
        :title="t('translation.triggerTitle')"
        :description="t('translation.triggerDesc')"
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
      :title="t('translation.baiduTranslate')"
      :description="t('translation.baiduDesc')"
    >
      <SettingSelectItem
        v-model="appConfigStore.BDTranslationTo"
        :title="t('translation.targetLanguage')"
        :description="t('translation.targetLanguageDesc')"
        :options="targetLanguageOptions"
      />

      <SettingItem
        icon="icon-hashjinghao"
        :title="`${t('translation.appId')} <span class='text-destructive'>*</span>`"
        :description="t('translation.appIdDesc')"
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
        :title="`${t('translation.secretKey')} <span class='text-destructive'>*</span>`"
        :description="t('translation.secretKeyDesc')"
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
          {{ t('translation.apiDoc') }}
        </a>
      </div>
    </SettingGroup>
  </div>
</template>

<script setup lang="ts">
import OpenDemoVideo from '@/components/OpenDemoVideo.vue';
import { useAppConfig } from '@/composables';
import { BAIDU_TRANSLATION_TO, TranslationOpenModel } from '@/constant';
import { t } from '@/i18n';

const { appConfigStore } = useAppConfig();
const options = computed<any[]>(() => [
  { label: t('translation.threeHits'), value: TranslationOpenModel.THREE_HITS_ON_SPACES },
  { label: t('webSearch.close'), value: TranslationOpenModel.CLOSE },
]);

const targetLanguageOptions = BAIDU_TRANSLATION_TO as any;
</script>
