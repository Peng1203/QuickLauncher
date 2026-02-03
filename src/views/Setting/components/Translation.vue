<template>
  <div>
    <n-form
      ref="formRef"
      size="small"
      label-placement="left"
      :model="appConfigStore"
      :label-width="160"
      :show-feedback="false"
    >
      <div class="flex items-center justify-between">
        <h3 class="!mt-[0]">启动</h3>

        <OpenDemoVideo
          video-url="https://www.bilibili.com/video/BV1fnFKzAEC4"
        />
      </div>
      <n-form-item>
        <n-checkbox
          v-model:checked="appConfigStore.enableTranslation"
          size="small"
        >
          启用翻译
        </n-checkbox>
      </n-form-item>
      <h3>呼出方式</h3>
      <n-form-item>
        <div class="w-full">
          <!-- @ts-ignore -->
          <n-select v-model:value="openModel" :options="options as any" />
        </div>
      </n-form-item>

      <h3>百度翻译</h3>
      <n-form
        ref="formRef"
        size="small"
        label-placement="left"
        :model="appConfigStore"
        :label-width="70"
        :show-feedback="false"
      >
        <n-form-item label="目标语言" class="w-[90%]">
          <n-select
            v-model:value="appConfigStore.BDTranslationTo"
            placeholder=""
            :options="BAIDU_TRANSLATION_TO as any"
          />
        </n-form-item>

        <n-form-item label="APP ID" class="mt-1 w-[90%]" required>
          <n-input
            v-model:value="appConfigStore.BDTranslationAppid"
            show-password-toggle
            type="password"
            placeholder="通过百度Api文档教程注册获取"
          />
        </n-form-item>

        <n-form-item label="密钥" class="mt-1 w-[90%]" required>
          <n-input
            v-model:value="appConfigStore.BDTranslationKey"
            show-password-toggle
            type="password"
            placeholder="通过百度Api文档教程注册获取"
          />
        </n-form-item>

        <div class="mt-1 flex gap-1">
          <n-button size="small" type="default" @click="handleOpenBDApiDoc">
            Api 文档
          </n-button>
        </div>
      </n-form>
    </n-form>
  </div>
</template>

<script setup lang="ts">
import { exeCommand } from '@/api';
import OpenDemoVideo from '@/components/OpenDemoVideo.vue';
import { useAppConfig } from '@/composables/useAppConfig';
import { BAIDU_TRANSLATION_TO } from '@/constant';

const { appConfigStore } = useAppConfig();
const openModel = ref<number>(0);
const options: OptionItem[] = [{ label: '快速点击 3 次空格', value: 0 }];

function handleOpenBDApiDoc() {
  exeCommand('https://fanyi-api.baidu.com/product/113');
}
</script>

<style scoped lang="scss">
.n-form-item {
  width: 90%;
  padding-left: 8px;
}
</style>
