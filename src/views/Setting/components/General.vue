<template>
  <n-form
    ref="formRef"
    size="small"
    label-placement="left"
    :model="appConfigStore"
    :label-width="160"
    :show-feedback="false"
  >
    <h3>系统</h3>
    <n-form-item>
      <n-checkbox
        size="small"
        v-model:checked="appConfigStore.autoStart"
      >
        开机自启
      </n-checkbox>
    </n-form-item>

    <n-form-item>
      <n-checkbox
        v-model:checked="appConfigStore.silentStart"
        @update-checked=""
      >
        静默启动
      </n-checkbox>
    </n-form-item>

    <!-- <n-form-item>
      <n-checkbox v-model:checked="appConfigStore.autoStart"> 开机自启 </n-checkbox>
    </n-form-item> -->

    <h3>语言</h3>
    <n-form-item class="mt-1">
      <n-select
        size="small"
        v-model:value="appConfigStore.language"
        placeholder="Select"
        :options="languageOptions"
      />
    </n-form-item>

    <h3>窗口</h3>
    <n-form-item>
      <n-checkbox v-model:checked="appConfigStore.onTop"> 窗口置顶 </n-checkbox>
    </n-form-item>
    <n-form-item>
      <n-checkbox
        v-model:checked="appConfigStore.center"
        @update-checked="setWindowCenter"
      >
        居中显示
      </n-checkbox>
    </n-form-item>

    <h3>显示/隐藏</h3>
    <n-form-item
      label="快捷键"
      label-width="auto"
    >
      <n-input
        clearable
        v-model:value="shortcutKey"
        type="text"
        readonly
        placeholder=""
        @input="handleChange"
        @keydown.prevent="handleKeydown"
        @blur="handleBlur"
      />
    </n-form-item>
    <n-button size="small">Alt + P</n-button>
    <n-button size="small">Alt + M</n-button>
  </n-form>
</template>

<script setup lang="ts">
import { useAppConfig } from '@/composables/useAppConfig'
import { useAppConfigActions } from '@/composables/useAppConfigActions'
import { getShortcutKey } from '@/utils/shortcutKey'
import {
  isRegistered,
  register,
  unregister,
  unregisterAll,
} from '@tauri-apps/plugin-global-shortcut'

const { appConfigStore } = useAppConfig()
const { setAlwaysOnTop, setWindowCenter, setAutoStart } = useAppConfigActions()

const languageOptions: OptionItem<LanguageType>[] = [
  { label: '简体中文', value: 'zh-CN' },
  { label: '繁體中文', value: 'zh-HK' },
  { label: 'English', value: 'en' },
  { label: '日本語', value: 'ja' },
]

const shortcutKey = ref(appConfigStore.mainWindowGlobalShortcutKey)
const handleKeydown = (e: KeyboardEvent) => {
  const keyValue = getShortcutKey(e, appConfigStore.mainWindowGlobalShortcutKey)
  console.log(`%c keyValue ----`, 'color: #fff;background-color: #000;font-size: 18px', keyValue)
  // TODO 校验输入的组合键是否合法
  shortcutKey.value = keyValue
  const { code, key, keyCode } = e

  // if (code === 'Escape' && keyCode === 27) return appConfigStore.mainWindowGlobalShortcutKey
  // if (code === 'Backspace' && keyCode === 8) return (shortcutKey.value = '')
  // // console.log(`%c e ----`, 'color: #fff;background-color: #000;font-size: 18px', e, code)
  // // 组合按键前置键
  // // const keys = ['Shift', 'Control', 'Alt', 'Meta']
  console.log('key ------', key)
  console.log('code ------', code)
  console.log('keyCode ------', keyCode)

  // if (keys.includes(key)) shortcutKey.value += `${key} + `
  // else {
  //   // 当判断前面已经存在组合按键 存在则根据 + 号拆分 则进行组合拼接 不存在时 只绑定单个按键
  //   if (keys.some(cKey => shortcutKey.value.includes(cKey))) {
  //     shortcutKey.value += code
  //     // shortcutKey.value += key.toUpperCase()
  //   } else shortcutKey.value = code
  // }

  // e.preventDefault()
}

const handleChange = (e: string) => {
  // shortcutKey.value = ''
}

const handleBlur = () => {
  console.log(`%c handleBlur ----`, 'color: #fff;background-color: #000;font-size: 18px')
}

const test1 = () => {
  unregisterAll()
}

const test = () => {
  console.log(
    `%c shortcutKey.value ----`,
    'color: #fff;background-color: #000;font-size: 18px',
    shortcutKey.value
  )
  // register(shortcutKey.value, e => {
  register('Win + Numpad5', e => {
    console.log(
      `%c 快捷键触发 ----`,
      'color: #fff;background-color: #000;font-size: 18px',
      shortcutKey.value,
      e
    )
  })
}
</script>

<style scoped>
h3 {
  font-size: 16px;
  font-weight: 600;
  margin-top: 5px;
}

.n-form-item {
  width: 90%;
  padding-left: 8px;
}

::v-deep(input[readonly]) {
  caret-color: #333 !important; /* 设置一个明显的光标颜色，如深灰色 */
}
::v-deep(.n-input__input) {
  caret-color: #333 !important; /* 设置一个明显的光标颜色，如深灰色 */
}
</style>
