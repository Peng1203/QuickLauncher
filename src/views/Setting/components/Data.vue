<template>
  <div class="flex flex-col gap-4 p-4">
    <SettingGroup :title="t('data.groupData')" :description="t('data.dataDesc')">
      <SettingItem
        icon="icon-daochu"
        :title="t('data.backupTitle')"
        :description="t('data.backupDesc')"
      >
        <n-button size="small" type="success" @click="handleExportBackup">
          {{ t("common.backup") }}
        </n-button>
      </SettingItem>

      <SettingItem
        icon="icon-daoru"
        :title="t('data.importTitle')"
        :description="t('data.importDesc')"
      >
        <n-button size="small" type="info" @click="handleImportBackup">
          {{ t("common.import") }}
        </n-button>
      </SettingItem>

      <SettingItem
        icon="icon-dakaiweizhi"
        :title="t('data.dirTitle')"
        :description="t('data.dirDesc')"
      >
        <n-button size="small" @click="handleOpenDbDirectory">
          {{ t("common.open") }}
        </n-button>
      </SettingItem>
    </SettingGroup>

    <SettingGroup :title="t('data.groupDanger')">
      <SettingItem
        icon-color="red"
        icon="icon-shanchu"
        :title="`<span style='color: var(--destructive);'>${t('data.resetTitle')}</span>`"
        :description="t('data.resetDesc')"
      >
        <n-button type="error" size="small" @click="handleResetData">
          {{ t("common.reset") }}
        </n-button>
      </SettingItem>
    </SettingGroup>
  </div>
</template>

<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";
import { backupDatabase, importDatabase, openAppDataDir, resetData } from "@/api";
import { useNaiveUiApi } from "@/composables";
import { t } from "@/i18n";

const { notification } = useNaiveUiApi();

async function handleExportBackup() {
  // 选择保存位置
  const filePath = await open({
    title: t("data.exportDialogTitle"),
    directory: true,
    multiple: false,
  });
  if (!filePath) return;
  const message = await backupDatabase(filePath);
  notification.success({ content: message, title: t("data.backupSuccess"), duration: 2000 });
}

async function handleImportBackup() {
  const filePath = await open({
    title: t("data.importDialogTitle"),
    directory: false,
    multiple: false,
    filters: [
      {
        name: "Database",
        extensions: ["db"],
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
