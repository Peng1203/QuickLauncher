<template>
  <div class="flex flex-col gap-4 p-4">
    <SettingGroup :title="t('quickSearch.groupEnable')">
      <!-- TODO -->
      <SettingSwitchItem
        v-model="appConfigStore.enableDefaultSearch"
        icon="icon-icon-sousuofenlei"
        :title="t('quickSearch.enableTitle')"
        :description="t('search.placeholder')"
      />
      <!-- :description="t('quickSearch.enableDesc')" -->
    </SettingGroup>

    <SettingGroup :title="t('quickSearch.groupCategory')">
      <SettingSwitchItem
        v-model="appConfigStore.enableDefaultSearchByCategory"
        icon="icon-fenlei"
        :title="t('quickSearch.categorySearch')"
        :description="t('quickSearch.categorySearchDesc')"
      />

      <SettingSwitchItem
        v-model="appConfigStore.enableCategorySearchDefaultData"
        icon="icon-fenlei"
        :title="t('quickSearch.defaultData')"
        :description="t('quickSearch.defaultDataDesc')"
      />

      <SettingSwitchItem
        v-model="appConfigStore.showCategory"
        icon="icon-fenlei"
        :title="t('quickSearch.showCategoryTitle')"
        :description="t('quickSearch.showCategoryDesc')"
      />

      <SettingSwitchItem
        v-model="appConfigStore.showSubCategory"
        icon="icon-tianjiazifenlei"
        :title="t('quickSearch.showSubCategoryTitle')"
        :description="t('quickSearch.showSubCategoryDesc')"
      />
    </SettingGroup>

    <SettingGroup :title="t('quickSearch.groupAutocomplete')">
      <SettingSwitchItem
        v-model="appConfigStore.enableAutocomplete"
        icon="icon-zidongbuquanshurukuang"
        :title="t('quickSearch.autocompleteTitle')"
        :description="t('quickSearch.autocompleteDesc')"
      />

      <SettingSwitchItem
        v-model="appConfigStore.enableAutocompleteFrequencyFilter"
        icon="icon-hashjinghao"
        :title="t('quickSearch.frequencyTitle')"
        :description="t('quickSearch.frequencyDesc')"
      />
    </SettingGroup>

    <SettingGroup :title="t('quickSearch.groupHistory')">
      <SettingSwitchItem
        v-model="appConfigStore.enableHistory"
        icon="icon-lishijilu_o"
        :title="t('quickSearch.historyTitle')"
        :description="t('quickSearch.historyDesc')"
      />

      <SettingSwitchItem
        v-model="appConfigStore.showHistory"
        icon="icon-daohang"
        :title="t('quickSearch.historyNavTitle')"
        :description="t('quickSearch.historyNavDesc')"
      />
    </SettingGroup>
  </div>
</template>

<script setup lang="ts">
import { useAppConfig } from "@/composables";
import { t } from "@/i18n";

const { appConfigStore } = useAppConfig();

const shortcutKey = ref("");
watch(
  () => appConfigStore.searchGlobalShortcutKey,
  (val) => (shortcutKey.value = val),
  { immediate: true },
);

function handleShowCategory(val: boolean) {
  if (!val) appConfigStore.showSubCategory = false;
}
</script>

<style scoped>
.n-form-item {
  width: 90%;
  padding-left: 8px;
}
</style>
