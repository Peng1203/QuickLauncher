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

    <SettingGroup :title="t('data.groupWebDav')">
      <SettingSwitchItem
        v-model="appConfigStore.webdavEnabled"
        icon="icon-wangluo"
        :title="t('data.webdavEnabled')"
        :description="t('data.webdavEnabledDesc')"
      />

      <template v-if="appConfigStore.webdavEnabled">
        <SettingItem
          icon="icon-wangluo"
          :title="t('data.webdavUrl')"
          :description="t('data.webdavUrlDesc')"
        >
          <n-input
            v-model:value="appConfigStore.webdavUrl"
            size="small"
            placeholder="https://dav.jianguoyun.com/dav/"
          />
        </SettingItem>

        <SettingItem
          icon="icon-yonghu"
          :title="t('data.webdavUsername')"
          :description="t('data.webdavUsernameDesc')"
        >
          <n-input
            v-model:value="appConfigStore.webdavUsername"
            size="small"
            placeholder="username@example.com"
          />
        </SettingItem>

        <SettingItem
          icon="icon-miyue"
          :title="t('data.webdavPassword')"
          :description="t('data.webdavPasswordDesc')"
        >
          <n-input
            v-model:value="appConfigStore.webdavPassword"
            size="small"
            type="password"
            show-password-toggle
            placeholder="应用密码"
          />
        </SettingItem>

        <SettingItem
          icon="icon-wenjian"
          :title="t('data.webdavPath')"
          :description="t('data.webdavPathDesc')"
        >
          <n-input
            v-model:value="appConfigStore.webdavPath"
            size="small"
            placeholder="/QuickLauncher/"
          />
        </SettingItem>

        <SettingItem
          icon="icon-wangluo"
          :title="t('data.webdavTest')"
          :description="t('data.webdavTestDesc')"
        >
          <n-button size="small" :loading="testing" @click="handleTestConnection">
            {{ t("data.webdavTest") }}
          </n-button>
        </SettingItem>

        <SettingItem
          icon="icon-daochu"
          :title="t('data.cloudBackup')"
          :description="t('data.cloudBackupDesc')"
        >
          <n-button size="small" type="success" :loading="backingUp" @click="handleCloudBackup">
            {{ t("data.cloudBackup") }}
          </n-button>
        </SettingItem>

        <SettingItem
          icon="icon-daoru"
          :title="t('data.cloudRestore')"
          :description="t('data.cloudRestoreDesc')"
        >
          <n-button size="small" type="info" :loading="restoring" @click="handleShowRestoreDialog">
            {{ t("data.cloudRestore") }}
          </n-button>
        </SettingItem>
      </template>
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

    <!-- 恢复对话框 -->
    <n-modal v-model:show="showRestoreDialog" preset="dialog" :title="t('data.cloudRestore')">
      <n-spin :show="loadingBackups">
        <div class="max-h-60 overflow-y-auto">
          <div v-if="backups.length === 0" class="text-center py-4 text-gray-400">
            {{ t("data.noBackups") }}
          </div>
          <div
            v-for="backup in backups"
            :key="backup.filename"
            class="flex items-center justify-between p-2 hover:bg-gray-100 rounded cursor-pointer"
            @click="handleCloudRestore(backup.filename)"
          >
            <span class="text-sm">{{ backup.filename }}</span>
            <span class="text-xs text-gray-400">{{ formatTimestamp(backup.timestamp) }}</span>
          </div>
        </div>
      </n-spin>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";
import {
  backupDatabase,
  importDatabase,
  openAppDataDir,
  resetData,
  webdavTestConnection,
  webdavBackup,
  webdavRestore,
  webdavListBackups,
} from "@/api";
import type { WebDavBackupInfo } from "@/api";
import { useAppConfig, useNaiveUiApi } from "@/composables";
import { t } from "@/i18n";

const { appConfigStore } = useAppConfig();
const { notification } = useNaiveUiApi();

const testing = ref(false);
const backingUp = ref(false);
const restoring = ref(false);
const showRestoreDialog = ref(false);
const backups = ref<WebDavBackupInfo[]>([]);
const loadingBackups = ref(false);

function formatTimestamp(timestamp: number): string {
  if (!timestamp) return "";
  const date = new Date(timestamp * 1000);
  return date.toLocaleString();
}

async function handleExportBackup() {
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

async function handleTestConnection() {
  testing.value = true;
  try {
    const message = await webdavTestConnection();
    notification.success({ content: message, title: t("data.webdavTest"), duration: 2000 });
  } catch (e) {
    notification.error({ content: e as string, title: t("data.webdavTest"), duration: 3000 });
  } finally {
    testing.value = false;
  }
}

async function handleCloudBackup() {
  backingUp.value = true;
  try {
    const message = await webdavBackup();
    notification.success({ content: message, title: t("data.cloudBackupSuccess"), duration: 2000 });
  } catch (e) {
    notification.error({
      content: e as string,
      title: t("data.cloudBackupFailed"),
      duration: 3000,
    });
  } finally {
    backingUp.value = false;
  }
}

async function handleShowRestoreDialog() {
  showRestoreDialog.value = true;
  loadingBackups.value = true;
  try {
    backups.value = await webdavListBackups();
  } catch (e) {
    notification.error({
      content: e as string,
      title: t("data.cloudRestoreFailed"),
      duration: 3000,
    });
    showRestoreDialog.value = false;
  } finally {
    loadingBackups.value = false;
  }
}

async function handleCloudRestore(filename: string) {
  restoring.value = true;
  showRestoreDialog.value = false;
  try {
    await webdavRestore(filename);
    // webdavRestore 会重启应用，无需额外操作
  } catch (e) {
    notification.error({
      content: e as string,
      title: t("data.cloudRestoreFailed"),
      duration: 3000,
    });
    restoring.value = false;
  }
}
</script>
