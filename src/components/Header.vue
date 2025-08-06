<template>
  <header
    :data-tauri-drag-region="!appConfigStore.center"
    class="fixed top-0 left-0 right-0 h-8 bg-white flex items-center justify-between px-2 border-b-1 border-gray-200 z-10"
  >
    <span class="text-gray-700">Quick Launcher</span>

    <!-- 右侧操作 -->
    <div class="flex items-center gap-2">
      <n-dropdown
        placement="bottom-start"
        trigger="click"
        size="small"
        :options="options"
        @select=""
      >
        <n-icon
          size="25"
          class="cursor-pointer"
        >
          <MenuOutline />
        </n-icon>
      </n-dropdown>

      <n-icon
        size="25"
        class="cursor-pointer"
        @click="handleClose"
      >
        <CloseOutline />
      </n-icon>
    </div>
  </header>
</template>

<script setup lang="tsx">
import { CloseOutline, MenuOutline } from '@vicons/ionicons5'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { DropdownMixedOption } from 'naive-ui/es/dropdown/src/interface'
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart'
import { useAppConfig } from '@/composables/useAppConfig'

const { appConfigStore } = useAppConfig()

const cuurrentWindow = getCurrentWebviewWindow()

const handleClose = async () => cuurrentWindow?.hide()

const options: DropdownMixedOption[] = [
  {
    key: 'onTop',
    label: '窗口置顶',
    type: 'render',
    render: () =>
      h(
        // v-model:
        // checked={appConfigStore.onTop}
        <n-checkbox
          size='small'
          class='mx-2'
          label='窗口置顶'
          default-checked={appConfigStore.onTop}
          on-update:checked={(val: boolean) => {
            appConfigStore.onTop = val
            cuurrentWindow?.setAlwaysOnTop(val)
          }}
        />
      ),
  },
  {
    key: 'center',
    label: '居中显示',
    type: 'render',
    render: () =>
      h(
        // v-model:checked={appConfigStore.center}
        <n-checkbox
          size='small'
          class='mx-2'
          label='居中显示'
          default-checked={appConfigStore.center}
          on-update:checked={(val: boolean) => {
            appConfigStore.center = val
            val && cuurrentWindow?.center()
          }}
        />
      ),
  },
  {
    key: 'silentStart',
    label: '静默启动',
    type: 'render',
    render: () =>
      h(
        <n-checkbox
          size='small'
          class='mx-2'
          label='静默启动'
          default-checked={appConfigStore.silentStart}
          v-model:checked={appConfigStore.silentStart}
        />
      ),
  },
  {
    key: 'autoStart',
    label: '开机自启',
    type: 'render',
    render: () =>
      h(
        // v-model:checked={appConfigStore.autoStart}
        <n-checkbox
          size='small'
          class='mx-2'
          label='开机自启'
          default-checked={appConfigStore.autoStart}
          on-update:checked={async (val: boolean) => {
            appConfigStore.autoStart = val

            const isEna = await isEnabled()

            if (!isEna && val) await enable()
            else await disable()
          }}
        />
      ),
  },

  // {
  //   key: 'about',
  //   label: '关于',
  // },
]
</script>
