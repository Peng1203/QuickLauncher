<template>
  <div class="con">
    <n-dynamic-tags
      v-model:value="browserOptions as any"
      size="small"
      :render-tag="handleRenderBrowserTag"
      @create="handleCreateBrowserOption"
    />

    <n-button
      class="!mt-1"
      size="tiny"
      type="info"
      title="重置默认选项"
      @click="handleSetDefaultBrowserOptions"
    >
      <n-icon size="16">
        <RefreshOutline />
      </n-icon>
    </n-button>
  </div>
</template>

<script setup lang="tsx">
import { LogoChrome, LogoEdge, LogoFirefox, RefreshOutline } from '@vicons/ionicons5';
import { ref } from 'vue';
import { useNaiveUiApi } from '@/composables/useNaiveUiApi';

const { message } = useNaiveUiApi();

const activeValue = defineModel<string>();

const defaultBrowserOptions = ref([
  {
    label: '默认',
    value: '',
  },
  {
    label: 'Chrome',
    value: 'chrome',
  },
  {
    label: 'Edge',
    value: 'msedge',
  },
  {
    label: 'Firefox',
    value: 'firefox',
  },
]);

const browserIcons: Record<string, any> = {
  chrome: LogoChrome,
  msedge: LogoEdge,
  firefox: LogoFirefox,
};

const LOCAL_BROWSER_KEY = 'local_browser_key';

// 浏览器选择
const baseBrowserOptions = ref<OptionItem[]>(
  localStorage.getItem(LOCAL_BROWSER_KEY)
    ? JSON.parse(localStorage.getItem(LOCAL_BROWSER_KEY) as any)
    : defaultBrowserOptions.value,
);

const browserOptions = computed<OptionItem[]>({
  get: () => baseBrowserOptions.value,
  set: val => (baseBrowserOptions.value = val.filter(item => item.value !== undefined)),
});

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

function saveBrowserOption() {
  nextTick(() => {
    localStorage.setItem(LOCAL_BROWSER_KEY, JSON.stringify(baseBrowserOptions.value));
  });
}

function handleCreateBrowserOption(newTag: string) {
  const [label, value] = newTag.split('=');

  if (!value || !label) message.warning('输入信息有误');
  else saveBrowserOption();

  return {
    label,
    value,
  };
}

function handleDeleteBrowserOption(item: OptionItem) {
  // 当删除的是当前选中的浏览器 则重置为默认
  if (activeValue.value === item.value) activeValue.value = '';
  // baseBrowserOptions.value
  const delIndex = baseBrowserOptions.value.findIndex(tag => tag.value === item.value);
  baseBrowserOptions.value.splice(delIndex, 1);
  // 持久化保存到本地
  saveBrowserOption();
}

function handleSetDefaultBrowserOptions() {
  baseBrowserOptions.value = JSON.parse(JSON.stringify(defaultBrowserOptions.value));
  saveBrowserOption();
}
</script>

<style scoped lang="scss">
.con {
  // display: flex;
  // flex-wrap: wrap;
}
</style>
