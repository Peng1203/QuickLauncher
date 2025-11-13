<template>
  <div>
    <n-form
      ref="formRef"
      size="small"
      label-placement="left"
      :model="appConfigStore"
      :label-width="160"
      :show-feedback="false"
    >
      <h3>代理</h3>
      <n-form-item>
        <n-checkbox v-model:checked="appConfigStore.proxy" size="small">
          使用代理
        </n-checkbox>
      </n-form-item>

      <n-form-item label="地址" label-width="auto" :required="true">
        <n-input
          v-model:value="appConfigStore.proxyHost"
          :disabled="!appConfigStore.proxy"
          clearable
          type="text"
          placeholder="http://127.0.0.1:7890 或 socks5://127.0.0.1:10090"
        />
      </n-form-item>

      <n-form-item label="用户名" label-width="auto" class="mt-1">
        <n-input
          v-model:value="appConfigStore.proxyUsername"
          :disabled="!appConfigStore.proxy"
          clearable
          type="text"
          placeholder=""
        />
      </n-form-item>

      <n-form-item label="密码" label-width="auto" class="mt-1">
        <n-input
          v-model:value="appConfigStore.proxyPassword"
          :disabled="!appConfigStore.proxy"
          clearable
          type="text"
          placeholder=""
        />
      </n-form-item>
    </n-form>
  </div>
</template>

<script setup lang="ts">
import { useAppConfig } from '@/composables/useAppConfig';

const { appConfigStore } = useAppConfig();
</script>

<style scoped>
.n-form-item {
  width: 90%;
  padding-left: 8px;
}
</style>
