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
      <h3>启动</h3>
      <n-form-item>
        <n-checkbox
          v-model:checked="appConfigStore.enableWebSearch"
          size="small"
        >
          启用网络搜索
        </n-checkbox>
      </n-form-item>
      <h3>呼出方式</h3>
      <n-form-item>
        <div class="w-full">
          <!-- @ts-ignore -->
          <n-select
            v-model:value="appConfigStore.webSearchOpenModel"
            :options="(options as any)"
          />
          <p class="text-[12px] leading-3.5 text-gray-500 mt-1">
            输入
            <b class="text-black">
              {{
                appConfigStore.webSearchOpenModel
                  ? '(英文状态) 冒号 + 关键字 + 空格'
                  : '关键字 + 空格'
              }}
            </b>
            使用网络搜索，例如使用谷歌搜索，输入
            <b class="text-black">
              {{ appConfigStore.webSearchOpenModel ? '":g"' : '"g"' }}
            </b>
            ，然后按下空格键，进入网络搜索模式。
          </p>
        </div>
      </n-form-item>

      <h3 class="flex justify-between items-center">
        搜索源

        <n-icon
          title="重置搜索源"
          size="18"
          class="cursor-pointer mr-2"
          @click="handleResetWebSource"
        >
          <RefreshOutline />
        </n-icon>
      </h3>
      <n-form-item>
        <n-data-table
          size="small"
          max-height="160"
          :data="webSearchSourceList"
          :columns="columns"
          :pagination="false"
          :bordered="true"
          :single-line="true"
          :row-props="getRowProps"
          :row-class-name="
            ({ id }) => (id === activeRowId ? '!bg-gray-200' : '')
          "
        />
      </n-form-item>
      <div class="mt-1 flex gap-1">
        <n-button size="small" type="default" @click="handleAdd">
          新 增
        </n-button>
        <n-button
          v-if="activeRowId"
          size="small"
          type="error"
          @click="handleDel"
        >
          删 除
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
          <n-form-item label="图标" class="mt-1 w-[90%]">
            <div class="flex items-end gap-2">
              <n-avatar
                size="medium"
                class="cursor-pointer"
                :src="sourceForm.icon"
                :class="[sourceForm.icon ? '!bg-transparent' : '']"
                @click="handleGetLocalFileIcon"
              />
              <!-- <n-input v-model:value="sourceForm.icon" placeholder="" /> -->

              <n-button
                size="tiny"
                type="default"
                title="选择本地图标"
                @click="handleGetLocalFileIcon"
              >
                <template #icon>
                  <n-icon>
                    <ArrowUp />
                  </n-icon>
                </template>
              </n-button>

              <n-tooltip placement="bottom" trigger="click">
                <template #trigger>
                  <n-button size="tiny" type="default" title="网络图片">
                    <template #icon>
                      <n-icon>
                        <LinkOutline />
                      </n-icon>
                    </template>
                  </n-button>
                </template>
                <div class="text-gray-700">输入网络图片地址</div>

                <n-input-group>
                  <n-input v-model:value="onlineImgUrl" placeholder="" />
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

              <n-tooltip placement="bottom" trigger="click" title="网站图标">
                <template #trigger>
                  <n-button size="tiny" type="default">
                    <template #icon>
                      <n-icon>
                        <GlobeOutline />
                      </n-icon>
                    </template>
                  </n-button>
                </template>
                <div class="text-gray-700">输入网站地址</div>
                <n-input-group>
                  <n-input v-model:value="webSiteUrl" placeholder="" />
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

              <n-tooltip placement="top" trigger="click" title="SVG 图标">
                <template #trigger>
                  <n-button size="tiny" type="default">
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
            </div>
          </n-form-item>
          <n-form-item label="名称" class="mt-1 w-[90%]">
            <n-input v-model:value="sourceForm.name" placeholder="" />
          </n-form-item>

          <n-form-item label="关键字" class="mt-1 w-[90%]">
            <n-input v-model:value="sourceForm.keywords" placeholder="" />
          </n-form-item>

          <n-form-item label="网址" class="mt-1 w-[90%]">
            <n-input
              v-model:value="sourceForm.searchApi"
              placeholder="https://www.baidu.com/s?wd={w}"
            />
          </n-form-item>

          <n-form-item label="描述" class="mt-1 w-[90%]">
            <n-input v-model:value="sourceForm.desc" placeholder="" />
          </n-form-item>

          <div class="mt-1 flex gap-1">
            <n-button size="small" type="info" @click="handleConfirm">
              确 认
            </n-button>
            <n-button size="small" type="tertiary" @click="handleCancel">
              取 消
            </n-button>
          </div>
        </n-form>
      </div>
    </n-form>
  </div>
</template>

<script setup lang="tsx">
import { open } from '@tauri-apps/plugin-dialog';
import {
  ArrowUp,
  CodeOutline,
  GlobeOutline,
  LinkOutline,
  RefreshOutline,
} from '@vicons/ionicons5';
import { h } from 'vue';
import { getLocalIconBase64, getOnlineImgBase64, getWebsiteInfo } from '@/api';
import { useAppConfig } from '@/composables/useAppConfig';
import { useNaiveUiApi } from '@/composables/useNaiveUiApi';
import { BASE_SOURCE, WebSearchOpenModel } from '@/constant';

const { appConfigStore, webSearchSourceList } = useAppConfig();

const options: OptionItem[] = [
  { label: '关键字 + 空格', value: WebSearchOpenModel.KEY_SPACE },
  { label: '冒号 + 关键字 + 空格', value: WebSearchOpenModel.COLON_KEY_SPACE },
];

const columns = [
  {
    title: '图标',
    key: 'icon',
    // prettier-ignore
    width: 50,
    render: (row: WebSearchSource) =>
      h(<n-avatar class="!bg-transparent mt-2" size={22} src={row.icon} />),
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
    style: 'cursor: pointer; height: 40px',
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

const onlineImgUrl = ref<string>('');
const onlineImgUrlLoading = ref<boolean>(false);
async function handleGetOnlineImg() {
  try {
    onlineImgUrlLoading.value = true;
    if (
      !(
        onlineImgUrl.value.includes('http://') ||
        onlineImgUrl.value.includes('https://')
      )
    ) {
      onlineImgUrl.value = `https://${onlineImgUrl.value}`;
    }
    const base64 = await getOnlineImgBase64(onlineImgUrl.value);
    sourceForm.value.icon = base64;
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
    if (
      !(
        webSiteUrl.value.includes('http://') ||
        webSiteUrl.value.includes('https://')
      )
    ) {
      webSiteUrl.value = `https://${webSiteUrl.value}`;
    }
    const { icon, title }: any = await getWebsiteInfo(webSiteUrl.value);
    sourceForm.value.icon = icon;
    if (title && sourceForm.value.desc === '') sourceForm.value.desc = title;
  } catch (e) {
    message.error(e as string);
  } finally {
    webSiteUrlLoading.value = false;
  }
}
// const data = ref<WebSearchSource[]>([...webSearchSourceList.value]);

const svgStr = ref<string>('');
async function handleGetSvgBase64() {
  if (!svgStr.value.trim()) return;
  const base64 = btoa(unescape(encodeURIComponent(svgStr.value)));
  sourceForm.value.icon = `data:image/svg+xml;base64,${base64}`;
}

async function handleConfirm() {
  if (sourceForm.value.id) handleSaveEdit();
  else handleSaveAdd();

  operationFormVisible.value = false;
}

function handleSaveEdit() {
  const editSource = webSearchSourceList.value.find(
    item => item.id === sourceForm.value.id
  );
  if (!editSource) return;
  for (const key in sourceForm.value) {
    // @ts-expect-error
    editSource[key] = sourceForm.value[key];
  }

  handleCancel();
}
function handleSaveAdd() {
  const exists = webSearchSourceList.value.some(
    item => item.keywords === sourceForm.value.keywords
  );

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
  const index = webSearchSourceList.value.findIndex(
    item => item.id === sourceForm.value.id
  );

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

    onlineImgUrl.value = '';
    webSiteUrl.value = '';
  });
}
</script>

<style scoped>
.n-form-item {
  width: 90%;
  padding-left: 8px;
}
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
