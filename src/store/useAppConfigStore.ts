import { defineStore } from 'pinia';
import { saveAppConfig } from '@/api';
import {
  SEARCH_INPUT_HEIGHT,
  SEARCH_RESULT_ITEM_HEIGHT,
  SEARCH_WINDOW_MAX_HEIGHT,
  SEARCH_WINDOW_WIDTH,
} from '@/constant';

export const useAppConfigStore = defineStore(
  'appConfig',
  //
  {
    state: (): AppConfigState => ({
      saveFlag: true,
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

      mainWindowPositionX: 0,
      mainWindowPositionY: 0,
      mainWindowGlobalShortcutKey: '',

      settingWindowPositionX: 0,
      settingWindowPositionY: 0,

      language: 'zh-CN',

      enableSearch: true,
      searchLostFocusHide: false,
      searchHideAfterOpen: true,

      enableWebSearch: true,
      webSearchOpenModel: 0,
      webSearchSourceList: [],

      showHistory: false,
      enableAutocomplete: true,
      autocompleteMatchMode: 'prefix',
      enableAutocompleteFrequencyFilter: true,

      enableTranslation: true,
      BDTranslationAppid: '',
      BDTranslationKey: '',
      BDTranslationTo: 'en',
    }),
    actions: {
      loadConfig(initData: AppConfigState) {
        for (const key in initData) {
          // @ts-ignore
          initData[key] !== undefined && (this.$state[key] = initData[key]);
        }
      },
      saveConfig() {
        saveAppConfig(this.$state);
      },
    },
    tauri: {
      autoStart: true,
      saveInterval: 1000,
      saveStrategy: 'debounce',

      hooks: {
        beforeBackendSync: (state: any) => {
          // 初始化钩子 在rust端之前调用
          console.log(
            `%c beforeBackendSync ----`,
            'color: #fff;background-color: #000;font-size: 18px',
            state
          );
          saveAppConfig(state);

          return state;
        },
      },
      saveOnChange: true,
    },
    persist: true,
  }
);
