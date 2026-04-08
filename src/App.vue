<template>
  <!-- :theme="darkTheme" -->
  <NConfigProvider>
    <NDialogProvider>
      <NNotificationProvider>
        <NMessageProvider>
          <NModalProvider>
            <router-view />
          </NModalProvider>
        </NMessageProvider>
      </NNotificationProvider>
    </NDialogProvider>
  </NConfigProvider>
</template>
