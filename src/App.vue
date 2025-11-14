<template>
  <n-message-provider>
    <n-config-provider>
      <n-modal-provider>
        <router-view />
      </n-modal-provider>
    </n-config-provider>
  </n-message-provider>
</template>

<script setup lang="ts"></script>
