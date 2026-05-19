<template>
  <div class="flex flex-col gap-4 p-4">
    <SettingGroup
      title="数据"
      description="备份应用配置、分类、启动项、历史记录等数据"
    >
      <SettingItem
        icon="icon-daochu"
        title="数据备份"
        description="导出当前所有数据到本地文件"
      >
        <n-button
          size="small"
          type="success"
          @click="handleExportBackup"
        >
          导出备份
        </n-button>
      </SettingItem>

      <SettingItem
        icon="icon-daoru"
        title="数据导入"
        description="从备份文件恢复应用数据，导入成功后应用会重启"
      >
        <n-button
          size="small"
          type="info"
          @click="handleImportBackup"
        >
          导入备份
        </n-button>
      </SettingItem>

      <SettingItem
        icon="icon-dakaiweizhi"
        title="数据目录"
        description="打开数据库与应用数据存放位置"
      >
        <n-button
          size="small"
          @click="handleOpenDbDirectory"
        >
          打开目录
        </n-button>
      </SettingItem>
    </SettingGroup>

    <SettingGroup title="危险操作">
      <SettingItem
        icon-color="red"
        icon="icon-shanchufenlei"
        title="<span style='color: #ef4444;'>重置数据</span>"
        description="清空所有本地数据，此操作不可恢复，操作前请确认已备份好数据"
      >
        <n-button
          type="error"
          size="small"
          @click="handleResetData"
        >
          重 置
        </n-button>
      </SettingItem>
    </SettingGroup>
  </div>
</template>

<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog';
import { backupDatabase, importDatabase, openAppDataDir, resetData } from '@/api';
import { useNaiveUiApi } from '@/composables';

const { notification } = useNaiveUiApi();

async function handleExportBackup() {
  // 选择保存位置
  const filePath = await open({
    title: '找个地儿 存住吧 😋',
    directory: true,
    multiple: false,
  });
  if (!filePath) return;
  const message = await backupDatabase(filePath);
  notification.success({ content: message, title: '备份成功', duration: 2000 });
}

async function handleImportBackup() {
  const filePath = await open({
    title: '回来吧 我最骄傲的数据 😭',
    directory: false,
    multiple: false,
    filters: [
      {
        name: 'Database',
        extensions: ['db'],
      },
    ],
  });
  if (!filePath) return;
  importDatabase(filePath);
}

function handleOpenDbDirectory() {
  openAppDataDir();
}

async function handleResetData() {
  await resetData();
}
</script>
