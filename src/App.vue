<template>
  <!-- :theme="darkTheme" -->
  <n-config-provider>
    <n-dialog-provider>
      <n-notification-provider>
        <n-message-provider>
          <n-modal-provider>
            <router-view />
          </n-modal-provider>
        </n-message-provider>
      </n-notification-provider>
    </n-dialog-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
// import { darkTheme } from 'naive-ui';
</script>
