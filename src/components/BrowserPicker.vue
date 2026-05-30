<template>
  <div class="con">
    <!-- @ts-ignore -->
    <n-dynamic-tags
      v-model:value="browserOptions"
      size="small"
      :render-tag="handleRenderBrowserTag"
      @create="handleCreateBrowserOption"
    />

    <n-button
      class="!mt-1"
      size="tiny"
      type="info"
      :title="t('browserPicker.resetDefault')"
      @click="handleSetDefaultBrowserOptions"
    >
      <n-icon size="16">
        <RefreshOutline />
      </n-icon>
    </n-button>
  </div>
</template>

<script setup lang="tsx">
import type { DynamicTagsOption } from 'naive-ui';
import { LogoChrome, LogoEdge, LogoFirefox, RefreshOutline } from '@vicons/ionicons5';
import { useStorage } from '@vueuse/core';
import { useNaiveUiApi } from '@/composables';
import { t } from '@/i18n';

const { message } = useNaiveUiApi();

const activeValue = defineModel<string>();

const defaultBrowserOptions: OptionItem[] = [
  { label: '默认', value: '' },
  { label: 'Chrome', value: 'chrome' },
  { label: 'Edge', value: 'msedge' },
  { label: 'Firefox', value: 'firefox' },
];

const browserIcons: Record<string, any> = {
  chrome: LogoChrome,
  msedge: LogoEdge,
  firefox: LogoFirefox,
};

const LOCAL_BROWSER_KEY = 'local_browser_key';

const baseBrowserOptions = useStorage<OptionItem[]>(LOCAL_BROWSER_KEY, defaultBrowserOptions);

const browserOptions = computed<OptionItem[]>({
  get: () => baseBrowserOptions.value,
  set: val => (baseBrowserOptions.value = val.filter(item => item.value !== undefined)),
}) as unknown as DynamicTagsOption as any;

function handleRenderBrowserTag(tag: OptionItem, index: number) {
  const IconComponent = browserIcons[tag.value as string];

  const NIcon = (
    <n-icon
      size="16"
      style={{ paddingTop: '1px' }}
    >
      <IconComponent />
    </n-icon>
  );

  return (
    <n-tag
      size="small"
      closable={index !== 0}
      style="cursor: pointer;"
      title={tag.value}
      type={activeValue.value === tag.value ? 'info' : ''}
      onClick={() => (activeValue.value = tag.value as any)}
      onClose={() => handleDeleteBrowserOption(tag)}
    >
      {IconComponent ? NIcon : tag.label}
    </n-tag>
  );
}

function handleCreateBrowserOption(newTag: string) {
  const [label, value] = newTag.split('=');

  if (!value || !label) message.warning(t('browserPicker.inputError'));

  return {
    label,
    value,
  };
}

function handleDeleteBrowserOption(item: OptionItem) {
  if (activeValue.value === item.value) activeValue.value = '';
  const delIndex = baseBrowserOptions.value.findIndex(tag => tag.value === item.value);
  baseBrowserOptions.value.splice(delIndex, 1);
}

function handleSetDefaultBrowserOptions() {
  baseBrowserOptions.value = [...defaultBrowserOptions];
}
</script>

<style scoped lang="scss">
.con {
  // display: flex;
  // flex-wrap: wrap;
}
</style>
