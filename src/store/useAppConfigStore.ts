import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'

export const useAppConfigStore = defineStore('appConfig', {
  state: (): AppConfigState => ({
    silentStart: true,
    autoStart: true,
    onTop: true,
    center: true,
  }),
  actions: {
    loadConfig(initData: AppConfigState) {
      for (const key in initData) {
        // @ts-ignore
        initData[key] !== undefined && (this.$state[key] = initData[key])
      }
    },
    saveConfig() {
      invoke('save_app_config', {
        config: {
          name: 'appConfig',
          data: JSON.stringify(this.$state),
        },
      })
    },
  },
})
