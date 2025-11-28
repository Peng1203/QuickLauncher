<template>
  <div class="w-full h-full bg-amber-600 opacity-20"></div>
</template>

<script setup lang="ts">
import { listen } from '@tauri-apps/api/event';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { LogicalPosition } from '@tauri-apps/api/window';

import { useAppConfig } from '@/composables/useAppConfig';

const currentWindow = getCurrentWebviewWindow();

const { appConfigStore } = useAppConfig();

// currentWindow.onDragDropEvent(async e => {
//   console.log(
//     `%c e ----`,
//     'color: #fff;background-color: #000;font-size: 18px',
//     e
//   );
// });

// currentWindow.once('')

listen('tauri://drag-drop', e => {
  currentWindow.hide();
  console.log('Dropped files:', e);
});
listen('tauri://drag-leave', () => {
  setTimeout(() => {
    currentWindow.hide();
  }, 50);
});

// listen('tauri://drag-enter', () => {
//   console.log('Drag enter');
// });

currentWindow.setIgnoreCursorEvents(false);

onMounted(() => {
  // currentWindow.show();
  currentWindow.setPosition(
    new LogicalPosition(
      appConfigStore.mainWindowPositionX + 192 + 12,
      appConfigStore.mainWindowPositionY + 32 + 4
    )
  );
});
</script>
