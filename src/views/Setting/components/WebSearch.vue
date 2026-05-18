<template>
  <div class="flex flex-col gap-4 p-4">
    <SettingGroup title="启用">
      <SettingSwitchItem
        v-model="appConfigStore.enableWebSearch"
        icon="icon-switch"
        title="启用网络搜索"
      >
        <OpenDemoVideo video-url="https://www.bilibili.com/video/BV1c7FKzKEc3" />
      </SettingSwitchItem>
    </SettingGroup>

    <SettingGroup title="特殊呼出">
      <n-select
        v-model:value="appConfigStore.webSearchOpenModel"
        size="small"
        :options="(options as any)"
      />
      <p
        v-show="appConfigStore.webSearchOpenModel !== WebSearchOpenModel.CLOSE"
        class="text-[12px] leading-3.5 text-gray-500 mt-1"
      >
        处于默认搜索模式下，输入
        <b class="text-black">
          {{ appConfigStore.webSearchOpenModel ? '(英文状态) 冒号 + 关键字 + 空格' : '关键字 + 空格' }}
        </b>
        使用网络搜索，例如使用谷歌搜索，输入
        <b class="text-black">
          {{ appConfigStore.webSearchOpenModel ? '":g"' : '"g"' }}
        </b>
        ，然后按下空格键，进入网络搜索模式。
      </p>
    </SettingGroup>

    <SettingGroup title="搜索源">
      <n-data-table
        size="small"
        max-height="160"
        :data="webSearchSourceList"
        :columns="columns"
        :pagination="false"
        :bordered="true"
        :single-line="true"
        :row-props="getRowProps"
        :row-class-name="({ id }) => (id === activeRowId ? '!bg-gray-200' : '')"
      />
      <div class="flex-sb-c">
        <n-button
          size="small"
          type="info"
          @click="handleAdd"
        >
          <template #icon>
            <Icon name="icon-xinzeng" />
          </template>
          新 增
        </n-button>

        <!-- v-if="activeRowId" -->
        <n-button
          :disabled="!activeRowId"
          size="small"
          type="error"
          @click="handleDel"
        >
          <template #icon>
            <Icon
              name="icon-shanchufenlei"
              size="14"
            />
          </template>
          删 除
        </n-button>

        <n-button
          size="small"
          type="default"
          @click="handleResetWebSource"
        >
          <template #icon>
            <Icon
              name="icon-shuaxin"
              size="14"
            />
          </template>
          重 置
        </n-button>
      </div>

      <div v-show="operationFormVisible">
        <h3>编辑</h3>
        <n-form
          ref="formRef"
          size="small"
          label-placement="left"
          :model="sourceForm"
          :label-width="60"
          :show-feedback="false"
        >
          <n-form-item
            label="图标"
            class="mt-1"
          >
            <div class="flex items-end gap-2">
              <n-avatar
                size="small"
                class="cursor-pointer"
                :src="sourceForm.icon"
                :class="[sourceForm.icon ? '!bg-transparent' : '']"
                @click="handleGetLocalFileIcon"
              />
              <!-- <n-input v-model:value="sourceForm.icon" placeholder="" /> -->
              <IconPicker v-model="sourceForm.icon!" />
            </div>
          </n-form-item>
          <n-form-item
            label="名称"
            class="mt-1"
          >
            <n-input
              v-model:value="sourceForm.name"
              placeholder=""
            />
          </n-form-item>

          <n-form-item
            label="关键字"
            class="mt-1"
          >
            <n-input
              v-model:value="sourceForm.keywords"
              placeholder=""
            />
          </n-form-item>

          <n-form-item
            label="网址"
            class="mt-1"
          >
            <n-input
              v-model:value="sourceForm.searchApi"
              placeholder="https://www.baidu.com/s?wd={w}"
            />
          </n-form-item>

          <n-form-item
            label="描述"
            class="mt-1"
          >
            <n-input
              v-model:value="sourceForm.desc"
              placeholder=""
            />
          </n-form-item>

          <div class="mt-3 flex-sb-c">
            <DescText>动态内容使用 {w} 替换</DescText>

            <div class="flex gap-1">
              <n-button
                size="small"
                type="info"
                @click="handleConfirm"
              >
                确 认
              </n-button>
              <n-button
                size="small"
                type="tertiary"
                @click="handleCancel"
              >
                取 消
              </n-button>
            </div>
          </div>
        </n-form>
      </div>
    </SettingGroup>
  </div>
</template>

<script setup lang="tsx">
import { open } from '@tauri-apps/plugin-dialog';
import { h } from 'vue';
import { getLocalIconBase64 } from '@/api';
import { useAppConfig, useNaiveUiApi } from '@/composables';
import { BASE_SOURCE, WebSearchOpenModel } from '@/constant';

const { appConfigStore, webSearchSourceList } = useAppConfig();

const options: OptionItem[] = [
  { label: '关键字 + 空格', value: WebSearchOpenModel.KEY_SPACE },
  { label: '冒号 + 关键字 + 空格', value: WebSearchOpenModel.COLON_KEY_SPACE },
  { label: '关闭', value: WebSearchOpenModel.CLOSE },
];

const columns = [
  {
    title: '图标',
    key: 'icon',
    // prettier-ignore
    width: 50,
    render: (row: WebSearchSource) =>
      h(
        <n-avatar
          class="!bg-transparent mt-2"
          size={22}
          src={row.icon}
        />,
      ),
  },
  { title: '名称', key: 'name', width: 80, ellipsis: true },
  { title: '关键字', key: 'keywords', width: 100, ellipsis: true },
  { title: '描述', key: 'desc', width: 180, ellipsis: true },
  // { title: '搜索建议', key: 'suggestion' },
  // TODO 使用指定浏览器打开
];

const operationFormVisible = ref<boolean>(false);
const sourceForm = ref<WebSearchSource>({
  id: 0,
  icon: '',
  name: '',
  keywords: '',
  searchApi: '',
  desc: '',
  suggestion: '',
  suggestionApi: '',
});

const activeRowId = ref<number>(0);
function getRowProps(row: WebSearchSource) {
  return {
    style: 'cursor: pointer; height: 34px',
    onClick: () => {
      activeRowId.value = row.id;

      sourceForm.value.id = row.id;
      sourceForm.value.icon = row.icon;
      sourceForm.value.name = row.name;
      sourceForm.value.keywords = row.keywords;
      sourceForm.value.desc = row.desc;
      sourceForm.value.searchApi = row.searchApi;

      operationFormVisible.value = true;
    },
  };
}

function handleAdd() {
  formInit();
  activeRowId.value = 0;
  operationFormVisible.value = true;
}

async function handleGetLocalFileIcon() {
  const path = await open({
    title: '选择图标',
    multiple: false,
    directory: false,
  });
  if (!path) return;
  const base64 = await getLocalIconBase64(path);
  sourceForm.value.icon = base64;
}
const { message } = useNaiveUiApi();

// const data = ref<WebSearchSource[]>([...webSearchSourceList.value]);

async function handleConfirm() {
  if (sourceForm.value.id) handleSaveEdit();
  else handleSaveAdd();

  operationFormVisible.value = false;
}

function handleSaveEdit() {
  const editSource = webSearchSourceList.value.find(item => item.id === sourceForm.value.id);
  if (!editSource) return;
  for (const key in sourceForm.value) {
    // @ts-expect-error
    editSource[key] = sourceForm.value[key];
  }

  handleCancel();
}
function handleSaveAdd() {
  const exists = webSearchSourceList.value.some(item => item.keywords === sourceForm.value.keywords);

  if (exists) return message.warning('已存在相同关键字');
  webSearchSourceList.value.push({
    ...sourceForm.value,
    id: Date.now(),
  });

  handleCancel();
}

function handleCancel() {
  formInit();
  activeRowId.value = 0;
  operationFormVisible.value = false;
}

function handleDel() {
  const index = webSearchSourceList.value.findIndex(item => item.id === sourceForm.value.id);

  if (index === -1) return;
  webSearchSourceList.value.splice(index, 1);

  handleCancel();
}

function handleResetWebSource() {
  handleCancel();
  webSearchSourceList.value = JSON.parse(JSON.stringify(BASE_SOURCE));
}

function formInit() {
  nextTick(() => {
    sourceForm.value.id = 0;
    sourceForm.value.icon = '';
    sourceForm.value.name = '';
    sourceForm.value.keywords = '';
    sourceForm.value.desc = '';
    sourceForm.value.searchApi = '';
    sourceForm.value.suggestion = '';
    sourceForm.value.suggestionApi = '';
  });
}
</script>

<style scoped>
::v-deep(.n-data-table .n-data-table-td) {
  background-color: initial !important;
}
::v-deep(.n-form-item-label__text) {
  text-align: left;
}
/* ::v-deep(td[data-col-key='icon']) { */
::v-deep(.n-data-table-td) {
  /* display: flex; */
  padding-top: 0;
  padding-bottom: 0;
}

/* ::v-deep(.n-data-table-th) {
  padding-left: 0 !important;
  padding-right: 0 !important;
} */
</style>

<style>
.n-popover {
  --n-text-color: #364153;
  --n-color: #fff !important;
}
</style>
