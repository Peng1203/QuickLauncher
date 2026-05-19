<template>
  <div class="flex flex-col gap-4 p-4">
    <SettingGroup title="启用">
      <SettingSwitchItem
        v-model="appConfigStore.enableCommandAlias"
        icon="icon-minglinghangchaxun"
        title="启用命令别名"
        description="通过简短别名快速执行系统命令"
      >
        <!-- <OpenDemoVideo video-url="https://www.bilibili.com/video/BV1c7FKzKEc3" /> -->
      </SettingSwitchItem>
    </SettingGroup>

    <SettingGroup title="映射列表">
      <div class="flex justify-end gap-2">
        <!-- {{ changed }} -->
        <n-button
          type="success"
          size="tiny"
          @click="
            dataList.unshift({
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
          <template #icon>
            <Icon
              name="icon-xinzeng"
              size="16"
            />
          </template>
          新 增
        </n-button>
        <n-button
          color="#1989fa"
          size="tiny"
          :disabled="changed"
          @click="handleSave"
        >
          <template #icon>
            <Icon
              name="icon-baocun"
              size="14"
            />
          </template>
          保 存
        </n-button>
      </div>

      <n-data-table
        size="small"
        max-height="300"
        :data="dataList"
        :columns="columns"
        :pagination="false"
        :bordered="false"
        :single-line="true"
        :row-props="() => ({ style: 'height: 40px' })"
      />

      <div class="flex-1 flex-sb-c">
        <span class="flex-s-c gap-0.5 text-[10px] text-gray-500">
          <Icon
            name="icon-tip"
            size="12"
          />
          <span>别名支持拼音搜索</span>
        </span>
        <n-button
          type="tertiary"
          size="tiny"
          @click="setDefaultData"
        >
          <template #icon>
            <Icon
              name="icon-shuaxin"
              size="14"
            />
          </template>
          默认数据
        </n-button>
      </div>
    </SettingGroup>
  </div>
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
  originData.value = structuredClone(data);
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
