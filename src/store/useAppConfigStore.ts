import { defineStore } from 'pinia'
import {
  SEARCH_WINDOW_MAX_HEIGHT,
  SEARCH_WINDOW_WIDTH,
  SEARCH_INPUT_HEIGHT,
  SEARCH_RESULT_ITEM_HEIGHT,
} from '@/constant'
import { saveAppConfig } from '@/api'

export const useAppConfigStore = defineStore('appConfig', {
  state: (): AppConfigState => ({
    silentStart: true,
    autoStart: false,
    onTop: true,
    center: true,

    // 可变通过用户后续输入的高度动态调整
    searchWindowMaxHeight: SEARCH_WINDOW_MAX_HEIGHT,
    searchWindowWidth: SEARCH_WINDOW_WIDTH,
    searchWindowInput: SEARCH_INPUT_HEIGHT,
    searchResultItemHeight: SEARCH_RESULT_ITEM_HEIGHT,
    searchGlobalShortcutKey: 'Alt+Space',

    proxy: false, 
    proxyHost: '',
    // proxy: true,
    // proxyHost: 'http://127.0.0.1:10090',
    proxyUsername: '',
    proxyPassword: '',

    // mainWindowPosition: { x: 0, y: 0 },
    mainWindowPositionX: 0,
    mainWindowPositionY: 0,
  }),
  actions: {
    loadConfig(initData: AppConfigState) {
      for (const key in initData) {
        // @ts-ignore
        initData[key] !== undefined && (this.$state[key] = initData[key])
      }
    },
    saveConfig() {
      saveAppConfig(this.$state)
    },
  },
})
