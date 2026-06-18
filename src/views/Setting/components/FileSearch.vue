<template>
  <div class="flex flex-col gap-4 p-4">
    <SettingGroup :title="t('quickSearch.groupEnable')">
      <SettingSwitchItem
        v-model="appConfigStore.enableFileSearch"
        icon="icon-everything"
        :title="t('fileSearch.enable')"
        :description="t('fileSearch.enableDesc')"
      />

      <SettingSwitchItem
        v-model="appConfigStore.fileSearchAutoStart"
        icon="icon-guanlianqidongqi"
        :title="t('fileSearch.autoStart')"
        :description="t('fileSearch.autoStartDesc')"
      />

      <SettingItem
        v-if="appConfigStore.fileSearchAutoStart"
        icon="icon-wenjian"
        :title="t('fileSearch.everythingExePath')"
        :description="t('fileSearch.everythingExePathDesc')"
      >
        <n-input-group class="w-40!">
          <n-input
            v-model:value="appConfigStore.everythingExePath"
            readonly
            clearable
            size="small"
            placeholder="Everything.exe"
            :title="appConfigStore.everythingExePath"
          />
          <n-button size="small" @click="handleSelectEverythingPath">
            {{ t("common.browse") }}
          </n-button>
        </n-input-group>
      </SettingItem>
    </SettingGroup>

    <SettingGroup :title="t('fileSearch.groupMode')" :description="t('fileSearch.groupModeDesc')">
      <SettingSelectItem
        v-model:value="appConfigStore.fileSearchMode"
        :options="modeOptions"
        icon="icon-sousuo"
        :title="t('fileSearch.searchMode')"
        :description="t('fileSearch.searchModeDesc')"
      />

      <!-- es.exe 模式配置 -->
      <template v-if="appConfigStore.fileSearchMode === 'es'">
        <!-- <SettingGroup
          :title="t('fileSearch.groupPath')"
          :description="t('fileSearch.groupPathDesc')"
        >
        </SettingGroup> -->
        <SettingItem
          icon="icon-wenjian"
          :title="t('fileSearch.esPath')"
          :description="t('fileSearch.esPathDesc')"
        >
          <div class="flex items-center gap-2">
            <n-input-group class="w-40!">
              <n-input
                v-model:value="appConfigStore.esFilePath"
                readonly
                clearable
                size="small"
                class="flex-1"
                placeholder="es.exe"
                :title="appConfigStore.esFilePath"
              />
              <n-button size="small" @click="handleSelectEsPath">
                {{ t("common.browse") }}
              </n-button>
            </n-input-group>
          </div>
        </SettingItem>
        <DescText class="!leading-3.5">
          文件搜索功能基于 Everything 提供的高速索引能力实现。为保证搜索正常使用，
          请确保已安装并运行 Everything，同时安装 Everything 命令行工具（es.exe）。
          如未安装相关组件，
          <a
            class="text-blue-500 font-medium cursor-pointer"
            href="https://www.voidtools.com/zh-cn/downloads/#cli"
            target="_blank"
          >
            点击前往下载
          </a>
          。
        </DescText>
      </template>

      <!-- HTTP 模式配置 -->
      <template v-if="appConfigStore.fileSearchMode === 'http'">
        <!-- <SettingGroup
          :title="t('fileSearch.groupHttp')"
          :description="t('fileSearch.groupHttpDesc')"
        >
        </SettingGroup> -->
        <SettingItem
          icon="icon-wangluo"
          :title="t('fileSearch.httpHost')"
          :description="t('fileSearch.httpHostDesc')"
        >
          <n-input
            v-model:value="appConfigStore.everythingHttpHost"
            size="small"
            placeholder="127.0.0.1"
          />
        </SettingItem>

        <SettingItem
          icon="icon-wangluo"
          :title="t('fileSearch.httpPort')"
          :description="t('fileSearch.httpPortDesc')"
        >
          <n-input-number
            v-model:value="appConfigStore.everythingHttpPort"
            size="small"
            :min="1"
            :max="65535"
            class="w-24"
          />
        </SettingItem>

        <DescText class="!leading-3.5">
          通过 Everything HTTP 服务器进行搜索。请确保 Everything 已启动并启用 HTTP 服务器功能。 在
          Everything 菜单中选择「工具 → 选项 → HTTP 服务器」进行配置。
        </DescText>
      </template>
    </SettingGroup>

    <!-- <SettingGroup
      :title="t('fileSearch.groupAutoStart')"
      :description="t('fileSearch.groupAutoStartDesc')"
    >
    </SettingGroup> -->

    <SettingGroup
      :title="t('fileSearch.groupParams')"
      :description="t('fileSearch.groupParamsDesc')"
    >
      <SettingItem
        icon="icon-sousuojieguoweikong2"
        :title="t('fileSearch.maxResults')"
        :description="t('fileSearch.maxResultsDesc')"
      >
        <n-input-number
          v-model:value="appConfigStore.fileSearchMaxResults"
          size="small"
          :min="1"
          :max="100"
          class="w-24"
        />
      </SettingItem>

      <SettingItem
        icon="icon-xitongpanfu"
        :title="t('fileSearch.searchPath')"
        :description="t('fileSearch.searchPathDesc')"
      >
        <n-input
          class="w-40!"
          v-model:value="appConfigStore.fileSearchPath"
          size="small"
          placeholder="C:\;D:\"
        />
      </SettingItem>

      <SettingItem
        icon="icon-a-guolvleixingguolv"
        :title="t('fileSearch.filter')"
        :description="t('fileSearch.filterDesc')"
      >
        <n-input
          class="w-40!"
          v-model:value="appConfigStore.fileSearchFilter"
          size="small"
          placeholder="*.exe;*.txt;*.doc"
        />
      </SettingItem>

      <SettingSelectItem
        v-model:value="appConfigStore.fileSearchSort"
        :options="sortOptions"
        icon="icon-paixu"
        :title="t('fileSearch.sort')"
        :description="t('fileSearch.sortDesc')"
      />

      <SettingSwitchItem
        v-model="appConfigStore.fileSearchRegex"
        icon="icon-zhengzeshi"
        :title="t('fileSearch.regex')"
        :description="t('fileSearch.regexDesc')"
      />
    </SettingGroup>
  </div>
</template>

<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";
import { useAppConfig } from "@/composables";
import { t } from "@/i18n";

const { appConfigStore } = useAppConfig();

const modeOptions = computed(() => [
  { label: t("fileSearch.modeEs"), value: "es" },
  { label: t("fileSearch.modeHttp"), value: "http" },
]);

const sortOptions = computed(() => [
  { label: t("fileSearch.sortDefault"), value: "default" },
  { label: t("fileSearch.sortName"), value: "name" },
  { label: t("fileSearch.sortPath"), value: "path" },
  { label: t("fileSearch.sortDate"), value: "date_modified" },
  { label: t("fileSearch.sortSize"), value: "size" },
  { label: t("fileSearch.sortReverse"), value: "reverse" },
]);

async function handleSelectEsPath() {
  try {
    const selected = await open({
      title: "选择 es.exe",
      filters: [
        {
          name: "Executable",
          extensions: ["exe"],
        },
      ],
      multiple: false,
    });

    if (selected) {
      appConfigStore.esFilePath = selected as string;
    }
  } catch (e) {
    console.error("选择 es.exe 失败:", e);
  }
}

async function handleSelectEverythingPath() {
  try {
    const selected = await open({
      title: "选择 Everything.exe",
      filters: [
        {
          name: "Executable",
          extensions: ["exe"],
        },
      ],
      multiple: false,
    });

    if (selected) {
      appConfigStore.everythingExePath = selected as string;
    }
  } catch (e) {
    console.error("选择 Everything.exe 失败:", e);
  }
}
</script>
