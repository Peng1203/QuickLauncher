<template>
  <div class="flex flex-col gap-4 p-4">
    <SettingGroup :title="t('quickSearch.groupEnable')">
      <SettingSwitchItem
        v-model="appConfigStore.enableWebSearch"
        icon="icon-wangluosousuo"
        :title="t('webSearch.enableTitle')"
      >
        <OpenDemoVideo video-url="https://www.bilibili.com/video/BV1c7FKzKEc3" />
      </SettingSwitchItem>
    </SettingGroup>

    <SettingGroup :title="t('webSearch.groupSpecial')">
      <n-select
        v-model:value="appConfigStore.webSearchOpenModel"
        size="small"
        :options="(options as any)"
      />
      <p
        v-show="appConfigStore.webSearchOpenModel !== WebSearchOpenModel.CLOSE"
        class="text-[12px] leading-3.5 text-muted-foreground"
      >
        {{ t("webSearchExtra.searchModeHint") }}
        <b class="text-foreground">
          {{
            appConfigStore.webSearchOpenModel
              ? t("webSearch.colonKeySpace")
              : t("webSearch.keySpace")
          }}
        </b>
        {{ t("webSearchExtra.searchModeHint2") }}
        <b class="text-foreground">
          {{ appConfigStore.webSearchOpenModel ? '":g"' : '"g"' }}
        </b>
        {{ t("webSearchExtra.searchModeHint3") }}
      </p>
    </SettingGroup>

    <SettingGroup :title="t('webSearch.groupSource')">
      <SettingSelectItem
        v-model:value="appConfigStore.webSearchDefaultSourceId"
        :options="defaultSourceOptions"
        :title="t('webSearch.defaultSource')"
        :description="t('webSearch.defaultWebSourceDesc')"
        :placeholder="t('webSearch.defaultSourcePlaceholder')"
      />

      <n-data-table
        size="small"
        max-height="160"
        :data="webSearchSourceList"
        :columns="columns"
        :pagination="false"
        :bordered="true"
        :single-line="true"
        :row-props="getRowProps"
        :row-class-name="({ id }) => (id === activeRowId ? '!bg-muted' : '')"
      />
      <div class="flex-sb-c">
        <n-button dashed size="small" type="info" @click="handleAdd">
          <template #icon>
            <Icon name="icon-xinzeng" />
          </template>
          {{ t("common.add") }}
        </n-button>

        <!-- v-if="activeRowId" -->
        <n-button :disabled="!activeRowId" size="small" type="error" @click="handleDel">
          <template #icon>
            <Icon name="icon-shanchu" size="14" />
          </template>
          {{ t("common.delete") }}
        </n-button>

        <n-button size="small" type="default" @click="handleResetWebSource">
          <template #icon>
            <Icon name="icon-shuaxin" size="14" />
          </template>
          {{ t("common.reset") }}
        </n-button>
      </div>

      <div v-show="operationFormVisible">
        <h3>{{ t("webSearch.edit") }}</h3>
        <n-form
          ref="formRef"
          size="small"
          label-placement="left"
          :model="sourceForm"
          :label-width="60"
          :show-feedback="false"
        >
          <n-form-item :label="t('common.icon')" class="mt-1">
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
          <n-form-item :label="t('common.name')" class="mt-1">
            <n-input v-model:value="sourceForm.name" placeholder="" />
          </n-form-item>

          <n-form-item :label="t('common.keywords')" class="mt-1">
            <n-input v-model:value="sourceForm.keywords" placeholder="" />
          </n-form-item>

          <n-form-item :label="t('common.url')" class="mt-1">
            <n-input
              v-model:value="sourceForm.searchApi"
              placeholder="https://www.baidu.com/s?wd={w}"
            />
          </n-form-item>

          <n-form-item :label="t('common.description')" class="mt-1">
            <n-input v-model:value="sourceForm.desc" placeholder="" />
          </n-form-item>

          <div class="mt-3 flex-sb-c">
            <DescText>{{ t("webSearch.dynamicContent") }}</DescText>

            <div class="flex gap-1">
              <n-button size="small" type="info" @click="handleConfirm">
                {{ t("common.confirm") }}
              </n-button>
              <n-button size="small" type="tertiary" @click="handleCancel">
                {{ t("common.cancel") }}
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
import { t } from '@/i18n';

const { appConfigStore, webSearchSourceList } = useAppConfig();

const options = computed<OptionItem[]>(() => [
  { label: t('webSearch.keySpace'), value: WebSearchOpenModel.KEY_SPACE },
  { label: t('webSearch.colonKeySpace'), value: WebSearchOpenModel.COLON_KEY_SPACE },
  { label: t('webSearch.close'), value: WebSearchOpenModel.CLOSE },
]);

const defaultSourceOptions = computed(() =>
  webSearchSourceList.value.map((item) => ({
    label: item.name,
    value: item.id,
  })),
);

const columns = computed(() => [
  {
    title: t('common.icon'),
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
  { title: t('common.name'), key: 'name', width: 80, ellipsis: true },
  { title: t('common.keywords'), key: 'keywords', width: 100, ellipsis: true },
  { title: t('common.description'), key: 'desc', width: 180, ellipsis: true },
  // { title: '搜索建议', key: 'suggestion' },
  // TODO 使用指定浏览器打开
]);

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
    title: t('webSearch.selectIcon'),
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

  if (exists) return message.warning(t('webSearch.duplicateKeyword'));

  const newId = Date.now();
  webSearchSourceList.value.push({
    ...sourceForm.value,
    id: newId,
  });

  // 如果是第一个搜索源，设为默认
  if (webSearchSourceList.value.length === 1) {
    appConfigStore.webSearchDefaultSourceId = newId;
  }

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

  // 如果删除的是默认搜索引擎，或列表为空，清空默认值
  if (appConfigStore.webSearchDefaultSourceId === sourceForm.value.id || webSearchSourceList.value.length === 0) {
    appConfigStore.webSearchDefaultSourceId = null;
  }

  handleCancel();
}

function handleResetWebSource() {
  handleCancel();
  webSearchSourceList.value = JSON.parse(JSON.stringify(BASE_SOURCE));
  // 重置后设第一个为默认
  appConfigStore.webSearchDefaultSourceId = webSearchSourceList.value[0]?.id ?? null;
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
  --n-text-color: var(--foreground);
  --n-color: var(--card) !important;
}
</style>
