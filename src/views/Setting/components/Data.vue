<template>
  <div class="mr-3">
    <!-- 数据 -->
    <h3 class="!mt-[0]">数据</h3>

    <n-space
      vertical
      :size="12"
    >
      <n-alert
        type="info"
        size="small"
        :show-icon="false"
      >
        备份应用配置、分类、启动项、历史记录等数据
      </n-alert>

      <!-- 备份 -->
      <n-card
        size="small"
        embedded
      >
        <div class="flex items-center justify-between gap-4">
          <div>
            <div class="text-[14px] font-medium">数据备份</div>
            <div class="mt-1 text-[12px] text-gray-500">导出当前所有数据到本地文件</div>
          </div>

          <n-button
            type="primary"
            secondary
            @click="handleExportBackup"
          >
            导出备份
          </n-button>
        </div>
      </n-card>

      <!-- 导入 -->
      <n-card
        size="small"
        embedded
      >
        <div class="flex items-center justify-between gap-4">
          <div>
            <div class="text-[14px] font-medium">数据导入</div>
            <div class="mt-1 text-[12px] text-gray-500">从备份文件恢复应用数据，导入成功后应用会重启</div>
          </div>

          <n-button
            type="warning"
            secondary
            @click="handleImportBackup"
          >
            导入备份
          </n-button>
        </div>
      </n-card>

      <!-- 数据目录 -->
      <n-card
        size="small"
        embedded
      >
        <div class="flex items-center justify-between gap-4">
          <div>
            <div class="text-[14px] font-medium">数据目录</div>
            <div class="mt-1 text-[12px] text-gray-500">打开数据库与应用数据存放位置</div>
          </div>

          <n-button
            type="info"
            secondary
            @click="handleOpenDbDirectory"
          >
            打开目录
          </n-button>
        </div>
      </n-card>

      <!-- 重置 -->
      <n-card
        size="small"
        embedded
      >
        <div class="flex items-center justify-between gap-4">
          <div>
            <div class="text-[14px] font-medium text-red-500">重置数据</div>
            <div class="mt-1 text-[12px] text-gray-500">清空所有本地数据，此操作不可恢复，操作前请确认已备份好数据</div>
          </div>

          <n-popconfirm @positive-click="handleResetData">
            <template #trigger>
              <n-button
                type="error"
                secondary
              >
                重置
              </n-button>
            </template>

            确认清空所有数据？
          </n-popconfirm>
        </div>
      </n-card>
    </n-space>
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
