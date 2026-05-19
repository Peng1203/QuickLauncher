<template>
  <div class="flex flex-col gap-4 p-4">
    <SettingGroup
      title="代理"
      description=""
    >
      <SettingSwitchItem
        v-model="appConfigStore.proxy"
        icon="icon-wangluodaili"
        title="使用代理"
        description="启用 HTTP 代理进行网络请求"
      />

      <SettingItem
        icon="icon-wangluo"
        title="地址 <span class='text-red-500'>*</span>"
        description="代理服务器地址"
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
        title="用户名"
        description="代理认证用户名（可选）"
      >
        <n-input
          v-model:value="appConfigStore.proxyUsername"
          size="small"
          type="text"
          placeholder="可选"
        />
      </SettingItem>

      <SettingItem
        icon="icon-mima"
        title="密码"
        description="代理认证密码（可选）"
      >
        <n-input
          v-model:value="appConfigStore.proxyPassword"
          size="small"
          type="text"
          placeholder="可选"
        />
      </SettingItem>

      <DescText
        class="!leading-3.5 mt-5"
        small
      >
        代理设置将应用于获取网站信息、加载网络图片、翻译等需要网络请求的功能。 如果您使用 Clash
        等代理工具，请确保已开启系统代理或手动配置代理地址。
      </DescText>
    </SettingGroup>
  </div>
</template>

<script setup lang="ts">
import { useAppConfig } from '@/composables';

const { appConfigStore } = useAppConfig();
</script>
