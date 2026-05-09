<template>
  <n-form
    ref="formRef"
    size="small"
    label-placement="left"
    :model="appConfigStore"
    :label-width="160"
    :show-feedback="false"
  >
    <div class="flex items-center justify-between">
      <h3 class="!mt-[0]">启用</h3>
      <!-- <OpenDemoVideo video-url="https://www.bilibili.com/video/BV1c7FKzKEc3" /> -->
    </div>

    <n-form-item>
      <n-checkbox
        v-model:checked="appConfigStore.enableCommandAlias"
        size="small"
      >
        启用命令别名
      </n-checkbox>
    </n-form-item>

    <h3 class="flex justify-between items-center">
      <span>列表</span>

      <div class="flex gap-2 mr-3">
        <!-- {{ changed }} -->
        <n-button
          type="success"
          size="tiny"
          @click="
            dataList.push({
              id: 0,
              name: '',
              path: '',
              type: 'alias',
              created_at: '',
              updated_at: '',
              launch_count: 0,
              failure_count: 0,
              run_as_admin: false,
              order_index: 0,
              enabled: true,
              pinyin_full: '',
              pinyin_abbr: '',
            })
          "
        >
          新 增
        </n-button>
        <n-button
          color="#1989fa"
          size="tiny"
          :disabled="changed"
          @click="handleSave"
        >
          保 存
        </n-button>
      </div>
    </h3>
    <n-form-item>
      <n-data-table
        size="small"
        max-height="300"
        :data="dataList"
        :columns="columns"
        :pagination="false"
        :bordered="true"
        :single-line="true"
        :row-props="() => ({ style: 'height: 40px' })"
      />
    </n-form-item>
    <n-form-item>
      <div class="flex-1 flex-sb-c">
        <span class="text-[12px] text-gray-500">别名支持拼音搜索</span>
        <n-button
          type="tertiary"
          size="tiny"
          @click="setDefaultData"
        >
          默认数据
        </n-button>
      </div>
    </n-form-item>
  </n-form>
</template>

<script setup lang="tsx">
import { isEqual } from 'lodash-es';
import { NIcon, NInput, NSwitch } from 'naive-ui';
import { ref } from 'vue';
import { addLaunch, deleteLaunch, exeCommand, getAliasLaunch, updateLaunch } from '@/api';
import { useAppConfig } from '@/composables';
import { defaultCommandAlias } from '@/constant/data';

const { appConfigStore } = useAppConfig();
const dataList = ref<LaunchItem[]>([]);
const originData = ref<LaunchItem[]>([]);
const changed = computed(() => isEqual(originData.value, dataList.value));
const columns = [
  {
    title: '别名',
    key: 'name',
    width: 150,
    render(row: LaunchItem, i: number) {
      return h(NInput, {
        size: 'small',
        placeholder: '',
        value: row.name,
        onUpdateValue(v) {
          dataList.value[i].name = v;
        },
      });
    },
    ellipsis: true,
  },
  {
    title: '命令',
    key: 'path',
    width: 150,
    render(row: any, i: number) {
      return h(NInput, {
        size: 'small',
        placeholder: '',
        value: row.path,
        onUpdateValue(v) {
          dataList.value[i].path = v;
        },
      });
    },
    ellipsis: true,
  },
  {
    title: '操作',
    key: '',
    width: 85,
    render(row: LaunchItem) {
      return h(
        <div class="flex gap-2">
          <span
            title="运行"
            class="cursor-pointer"
            onClick={() => handleRun(row)}
          >
            <NIcon class="iconfont icon-yunhang" />
          </span>

          <span
            title="删除"
            class="cursor-pointer"
            onClick={() => handleDelete(row)}
          >
            <NIcon class="iconfont icon-shanchufenlei " />
          </span>
          <NSwitch
            size="small"
            default-value={row.enabled}
            on-update:value={(val: boolean) => (row.enabled = val)}
          />
        </div>,
      );
    },
  },
];

async function getData() {
  const data = await getAliasLaunch();
  originData.value = JSON.parse(JSON.stringify(data));
  dataList.value = data;
}
getData();

/**
 * 保存分为两种情况
 *  已有修改的
 *  新增的
 */
async function handleSave() {
  const changedRecord: LaunchItem[] = [];
  const newRecord: LaunchItem[] = [];
  dataList.value.forEach(item => {
    if (!item.name || !item.path) return;
    if (item.id) {
      const originItem = originData.value.find(i => i.id === item.id);
      const isChanged = !isEqual(originItem, item);
      isChanged && changedRecord.push(item);
    } else {
      newRecord.push(item);
    }
  });

  await Promise.all([...changedRecord.map(item => updateLaunch(item)), ...newRecord.map(item => addLaunch(item))]);
  getData();
}

function handleRun(row: LaunchItem) {
  if (!row.path) return;
  exeCommand(row.path);
}

async function handleDelete(row: LaunchItem) {
  const cb = (item: LaunchItem) => item.id === row.id;

  const index = dataList.value.findIndex(cb);
  dataList.value.splice(index, 1);
  if (row.id) {
    // 删除已有记录
    const i = originData.value.findIndex(cb);
    originData.value.splice(i, 1);

    await deleteLaunch(row.id);
    // getData();
  }
}

async function setDefaultData() {
  const defaultData: LaunchItem[] = defaultCommandAlias.map(item => ({
    id: 0,
    type: 'alias',
    created_at: '',
    updated_at: '',
    launch_count: 0,
    failure_count: 0,
    run_as_admin: false,
    order_index: 0,
    enabled: true,
    pinyin_full: '',
    pinyin_abbr: '',
    ...item,
  }));

  dataList.value = [...dataList.value, ...defaultData];
}
</script>

<style scoped lang="scss">
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
</style>
