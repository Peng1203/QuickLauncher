<template>
  <n-notification-provider>
    <n-message-provider>
      <n-config-provider>
        <n-modal-provider>
          <router-view />
        </n-modal-provider>
      </n-config-provider>
    </n-message-provider>
  </n-notification-provider>
</template>

<script setup lang="ts"></script>
