<template>
  <div class="flex flex-col gap-4 p-4">
    <SettingGroup
      :title="t('proxy.groupProxy')"
      description=""
    >
      <SettingSwitchItem
        v-model="appConfigStore.proxy"
        icon="icon-wangluodaili"
        :title="t('proxy.useProxyTitle')"
        :description="t('proxy.useProxyDesc')"
      />

      <SettingItem
        icon="icon-wangluo"
        :title="`${t('proxy.addressTitle')} <span class='text-destructive'>*</span>`"
        :description="t('proxy.addressDesc')"
      >
        <n-input
          v-model:value="appConfigStore.proxyHost"
          size="small"
          type="text"
          placeholder="http://127.0.0.1:10090"
        />
      </SettingItem>

      <SettingItem
        icon="icon-yonghuming"
        :title="t('proxy.usernameTitle')"
        :description="t('proxy.usernameDesc')"
      >
        <n-input
          v-model:value="appConfigStore.proxyUsername"
          size="small"
          type="text"
          :placeholder="t('proxy.optional')"
        />
      </SettingItem>

      <SettingItem
        icon="icon-mima"
        :title="t('proxy.passwordTitle')"
        :description="t('proxy.passwordDesc')"
      >
        <n-input
          v-model:value="appConfigStore.proxyPassword"
          size="small"
          type="text"
          :placeholder="t('proxy.optional')"
        />
      </SettingItem>

      <DescText
        class="!leading-3.5 mt-5"
        small
      >
        {{ t('proxy.proxyNote') }}
      </DescText>
    </SettingGroup>
  </div>
</template>

<script setup lang="ts">
import { useAppConfig } from '@/composables';
import { t } from '@/i18n';

const { appConfigStore } = useAppConfig();
</script>
