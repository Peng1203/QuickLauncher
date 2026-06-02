<template>
  <div class="flex flex-col gap-4 p-4">
    <SettingGroup :title="t('quickSearch.groupEnable')">
      <SettingSwitchItem
        v-model="appConfigStore.enableCommandAlias"
        icon="icon-minglinghangchaxun"
        :title="t('commandAlias.enableTitle')"
        :description="t('commandAlias.enableDesc')"
      >
        <!-- <OpenDemoVideo video-url="https://www.bilibili.com/video/BV1c7FKzKEc3" /> -->
      </SettingSwitchItem>
    </SettingGroup>

    <SettingGroup :title="t('commandAlias.groupList')">
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
            <Icon name="icon-xinzeng" size="16" />
          </template>
          {{ t("common.add") }}
        </n-button>
        <n-button type="primary" size="tiny" :disabled="changed" @click="handleSave">
          <template #icon>
            <Icon name="icon-baocun" size="14" />
          </template>
          {{ t("common.save") }}
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
        <span class="flex-s-c gap-0.5 text-[10px] text-muted-foreground">
          <Icon name="icon-tip" size="12" />
          <span>{{ t("commandAlias.aliasPinyinTip") }}</span>
        </span>
        <n-button type="tertiary" size="tiny" @click="setDefaultData">
          <template #icon>
            <Icon name="icon-shuaxin" size="14" />
          </template>
          {{ t("commandAlias.defaultData") }}
        </n-button>
      </div>
    </SettingGroup>
  </div>
</template>

<script setup lang="tsx">
import { isEqual } from 'lodash-es';
import { NInput, NSwitch } from 'naive-ui';
import { ref } from 'vue';
import { addLaunch, deleteLaunch, exeCommand, getAliasLaunch, updateLaunch } from '@/api';
import Icon from '@/components/ui/Icon.vue';
import { useAppConfig } from '@/composables';
import { defaultCommandAlias } from '@/constant/data';
import { t } from '@/i18n';

const { appConfigStore } = useAppConfig();
const dataList = ref<LaunchItem[]>([]);
const originData = ref<LaunchItem[]>([]);
const changed = computed(() => isEqual(originData.value, dataList.value));
const columns = computed(() => [
  {
    title: t('commandAlias.alias'),
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
    title: t('commandAlias.command'),
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
    title: t('common.operation'),
    key: '',
    width: 85,
    render(row: LaunchItem) {
      return h(
        <div class="flex gap-2">
          <span
            title={t('commandAlias.run')}
            class="cursor-pointer"
            onClick={() => handleRun(row)}
          >
            <Icon
              name="icon-yunhang"
              size="16"
            />
          </span>

          <span
            title={t('common.deletePlain')}
            class="cursor-pointer"
            onClick={() => handleDelete(row)}
          >
            <Icon
              name="icon-shanchufenlei"
              size="16"
            />
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
]);

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
      if(isChanged)  changedRecord.push(item);
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
