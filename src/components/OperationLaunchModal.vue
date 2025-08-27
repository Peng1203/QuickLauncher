<template>
  <n-modal
    :mask-closable="false"
    v-model:show="modalStatus"
    @close="handleClose"
  >
    <!-- TODO 新建支持拖拽 -->
    <n-card
      title="新建项目"
      size="huge"
      role="dialog"
      aria-modal="true"
      :bordered="false"
    >
      <template #header-extra>
        <n-icon
          size="20"
          class="cursor-pointer"
          @click="handleClose"
        >
          <Close />
        </n-icon>
      </template>
      <n-tabs
        animated
        type="bar"
        placement="left"
        size="small"
        :default-value="form.type"
        :on-update:value="handleTypeChange"
      >
        <n-tab-pane
          :name="item.value"
          :tab="item.label"
          v-for="item in launchTypes"
        >
          <div style="max-height: 310px; overflow-y: auto">
            <n-form
              ref="formRef"
              size="small"
              label-width="80"
              :show-feedback="false"
              :model="form"
              :rules="formRules"
            >
              <!-- {{ currentFormSchemas }} -->
              <!-- {{ form }} -->
              <n-row>
                <template v-for="item in currentFormSchemas">
                  <n-col
                    :span="(item.span  as any)"
                    class="!flex justify-start items-end"
                    v-if="item.slot === 'iconSlot'"
                  >
                    <n-avatar
                      size="large"
                      :src="form.icon || ''"
                    />
                  </n-col>

                  <n-col
                    :span="(item.span  as any)"
                    v-else-if="item.slot === 'pathSlot'"
                  >
                    <n-form-item
                      :label="item.label"
                      :path="item.prop"
                    >
                      <n-input
                        placeholder=""
                        :type="item.type || 'text'"
                        :theme-overrides="inputTheme"
                        v-model:value="(form[item.prop] as any)"
                      />
                    </n-form-item>
                    <n-button
                      class="!mt-1"
                      size="small"
                      color="lightgray"
                      text-color="gary"
                    >
                      选 择
                    </n-button>
                  </n-col>

                  <n-col
                    :span="(item.span  as any)"
                    v-else-if="item.slot === 'urlSlot'"
                  >
                    <n-form-item
                      :label="item.label"
                      :path="item.prop"
                    >
                      <n-input
                        placeholder=""
                        :type="item.type || 'text'"
                        :theme-overrides="inputTheme"
                        v-model:value="(form[item.prop] as any)"
                      />
                    </n-form-item>
                    <n-button
                      class="!mt-1"
                      size="small"
                      type="info"
                      :loading="urlInfoLoading"
                      @click="getUrlInfo"
                    >
                      获取网址信息
                    </n-button>

                    <!-- TODO 快捷开启代理 v-model:checked="" -->
                    <n-checkbox
                      class="ml-5"
                      size="small"
                      :checked-value="1"
                      :unchecked-value="0"
                      :default-checked="0"
                    >
                      代理
                    </n-checkbox>
                  </n-col>

                  <n-col
                    :span="(item.span  as any)"
                    v-else-if="item.slot === 'hotkeySlot'"
                  >
                    <n-form-item
                      :label="item.label"
                      :path="item.prop"
                    >
                      <n-input
                        placeholder=""
                        :type="item.type || 'text'"
                        :theme-overrides="inputTheme"
                        v-model:value="(form[item.prop] as any)"
                      />
                    </n-form-item>
                    <!-- class="mt-1" -->
                    <n-checkbox
                      size="small"
                      v-model:checked="form.hotkey_global"
                      :checked-value="1"
                      :unchecked-value="0"
                      :default-checked="0"
                    >
                      全局快捷键
                    </n-checkbox>
                  </n-col>

                  <n-col
                    :span="(item.span  as any)"
                    v-else-if="item.slot === 'runAsAdminSlot'"
                  >
                    <n-form-item
                      :label="item.label"
                      :path="item.prop"
                      class="run_as_admin"
                    >
                      <n-checkbox
                        size="small"
                        v-model:checked="form.run_as_admin"
                        :checked-value="1"
                        :unchecked-value="0"
                        :default-checked="0"
                      >
                        以管理员身份运行
                      </n-checkbox>
                    </n-form-item>
                  </n-col>

                  <!-- 关键字 -->
                  <n-col
                    :span="(item.span  as any)"
                    v-else-if="item.slot === 'keywordsSlot'"
                  >
                    <n-form-item
                      :label="item.label"
                      :path="item.prop"
                    >
                      <n-dynamic-tags
                        type="success"
                        size="small"
                        v-model:value="keywordsTags"
                      />
                    </n-form-item>
                  </n-col>

                  <!-- 排序 -->
                  <n-col
                    :span="(item.span  as any)"
                    v-else-if="item.slot === 'orderSlot'"
                  >
                    <n-form-item
                      :label="item.label"
                      :path="item.prop"
                    >
                      <!-- borderFocus: 'inherit', -->
                      <!-- boxShadowFocus: 'none', -->
                      <!-- caretColor: 'inherit', -->
                      <!-- borderHover: 'inherit', -->
                      <n-input-number
                        placeholder=""
                        class="w-25"
                        size="small"
                        v-model:value.number="form.order_index"
                      />
                      <span class="ml-2 text-gray-400"> 数字越小越靠前 </span>
                    </n-form-item>
                  </n-col>

                  <!-- 启动项分类 -->
                  <n-col
                    :span="(item.span  as any)"
                    v-else-if="item.slot === 'categorySlot'"
                  >
                    <n-form-item
                      :label="item.label"
                      :path="item.prop"
                    >
                      <n-select
                        placeholder=""
                        :default-value="null"
                        :options="categoryOptions"
                        v-model:value="form.category_id"
                      />
                    </n-form-item>
                  </n-col>

                  <n-col
                    :span="(item.span  as any)"
                    v-else-if="item.slot === 'enabledSlot'"
                  >
                    <n-form-item
                      :label="item.label"
                      :path="item.prop"
                    >
                      <n-switch
                        v-model:value="form.enabled"
                        :default-value="1"
                        :checked-value="1"
                        :unchecked-value="0"
                      />
                    </n-form-item>
                  </n-col>

                  <n-col
                    v-else
                    :span="(item.span  as any)"
                  >
                    <n-form-item
                      :label="item.label"
                      :path="item.prop"
                    >
                      <n-input
                        :theme-overrides="inputTheme"
                        :type="item.type || 'text'"
                        :placeholder="item.placeholder || ''"
                        v-model:value="(form[item.prop] as any)"
                      />
                    </n-form-item>
                  </n-col>
                </template>
              </n-row>
            </n-form>
          </div>
        </n-tab-pane>
        <template #suffix> </template>
      </n-tabs>

      <template #footer>
        <div class="flex justify-end gap-4">
          <n-button
            size="small"
            type="info"
          >
            确 认
          </n-button>
          <n-button
            size="small"
            @click="handleClose"
          >
            取 消
          </n-button>
        </div>
      </template>
    </n-card>
  </n-modal>
</template>

<script setup lang="ts">
import { getWebsiteInfo } from '@/api'
import { Close } from '@vicons/ionicons5'
import { ref, computed } from 'vue'
import { useNaiveUiApi } from '@/composables/useNaiveUiApi'

// 表单字段 schema 类型
type FieldSchema = {
  prop: keyof NewLaunchItem
  label: string
  type?: 'text' | 'textarea'
  slot?: string
  span?: number
  placeholder?: string
}

type LaunchItemType = NewLaunchItem['type']

const modalStatus = defineModel<boolean>({ default: true })
const launchTypes = [
  { value: 'file', label: '文 件' },
  { value: 'directory', label: '文件夹' },
  { value: 'url', label: '网 站' },
] as const satisfies { value: LaunchItemType; label: string }[]

const formSchemas: Record<LaunchItemType, FieldSchema[]> = {
  file: [
    { prop: 'icon', label: '', span: 3, slot: 'iconSlot' },
    { prop: 'name', label: '名称', span: 17 },
    { prop: 'path', label: '路径', type: 'textarea', span: 20, slot: 'pathSlot' },

    { prop: 'start_dir', label: '起始位置', span: 20, placeholder: 'C:\\Windows\\System32' },
    { prop: 'args', label: '启动参数', span: 20, placeholder: 'chrome.exe --incognito' },
    { prop: 'run_as_admin', label: '', span: 20, slot: 'runAsAdminSlot' },

    { prop: 'keywords', label: '搜索关键字', span: 20, slot: 'keywordsSlot' },
    { prop: 'category_id', label: '分类', span: 20, slot: 'categorySlot' },

    { prop: 'hotkey', label: '快捷键', span: 20, slot: 'hotkeySlot' },
    { prop: 'enabled', label: '启用搜索', span: 20, slot: 'enabledSlot' },
    { prop: 'order_index', label: '排序', span: 20, slot: 'orderSlot' },
    { prop: 'remarks', label: '备注', type: 'textarea', span: 20 },
  ],
  directory: [
    { prop: 'icon', label: '', span: 3, slot: 'iconSlot' },
    { prop: 'name', label: '名称', span: 17 },
    { prop: 'path', label: '路径', type: 'textarea', span: 20, slot: 'pathSlot' },

    { prop: 'keywords', label: '搜索关键字', span: 20, slot: 'keywordsSlot' },
    { prop: 'category_id', label: '分类', span: 20, slot: 'categorySlot' },

    { prop: 'hotkey', label: '快捷键', span: 20, slot: 'hotkeySlot' },
    { prop: 'enabled', label: '启用搜索', span: 20, slot: 'enabledSlot' },
    { prop: 'order_index', label: '排序', span: 20, slot: 'orderSlot' },
    { prop: 'remarks', label: '备注', type: 'textarea', span: 20 },
  ],
  url: [
    { prop: 'icon', label: '', span: 3, slot: 'iconSlot' },
    { prop: 'name', label: '名称', span: 17 },
    { prop: 'path', label: '网址', type: 'textarea', span: 20, slot: 'urlSlot' },

    { prop: 'keywords', label: '搜索关键字', span: 20, slot: 'keywordsSlot' },
    { prop: 'category_id', label: '分类', span: 20, slot: 'categorySlot' },

    { prop: 'hotkey', label: '快捷键', span: 20, slot: 'hotkeySlot' },
    { prop: 'enabled', label: '启用搜索', span: 20, slot: 'enabledSlot' },
    { prop: 'order_index', label: '排序', span: 20, slot: 'orderSlot' },
    { prop: 'remarks', label: '备注', type: 'textarea', span: 20 },
  ],
}
const categoryOptions = ref([
  {
    label: '应用程序',
    value: 1,
  },
])

const currentFormSchemas = computed(() => formSchemas[form.value.type])

const form = ref<NewLaunchItem>({
  name: '',
  path: '',
  type: launchTypes[0].value,

  icon: '',
  hotkey: '',
  hotkey_global: 0,
  keywords: '',
  start_dir: '',
  remarks: '',
  args: '',
  run_as_admin: 0,
  order_index: 0,
  enabled: 1,
  category_id: null,
  extension: null,
})

const keywordsTags = computed({
  get: () => (form.value?.keywords || '').split(',').filter(item => item),
  set: val => (form.value.keywords = val.join()),
})

const formRules = ref({})

const inputTheme = {
  borderFocus: 'inherit',
  boxShadowFocus: 'none',
  caretColor: 'inherit',
  borderHover: 'inherit',
}

const handleClose = () => {
  modalStatus.value = false
}

const { message } = useNaiveUiApi()

const urlInfoLoading = ref(false)
const getUrlInfo = async () => {
  if (!form.value.path.trim()) return
  // 输入的url进行补全
  if (!(form.value.path.includes('http://') || form.value.path.includes('https://')))
    form.value.path = `http://${form.value.path}`
  urlInfoLoading.value = true
  const data: any = await getWebsiteInfo(form.value.path).catch(err => message.error(err))
  urlInfoLoading.value = false
  form.value.icon = `${form.value.path}${data.icon}`
  form.value.name = data.title
  // console.log('res ------', res)
}

const initForm = () => {
  form.value.type = launchTypes[0].value
  form.value.name = ''
  form.value.path = ''
  form.value.icon = ''
  form.value.hotkey = ''
  form.value.hotkey_global = 0
  form.value.keywords = ''
  form.value.start_dir = ''
  form.value.remarks = ''
  form.value.args = ''
  form.value.run_as_admin = 0
  form.value.order_index = 0
  form.value.enabled = 1
  form.value.category_id = null

  form.value.extension = null
}

const handleTypeChange = (val: LaunchItemType) => {
  initForm()
  nextTick(() => {
    form.value.type = val
  })
}
</script>

<style scoped>
.n-modal {
  padding: 10px;
}

.n-card {
  width: 600px;
  height: 400px;
}
::v-deep(.n-card-header),
::v-deep(.n-card__content),
::v-deep(.n-card__footer) {
  padding: 0;
}

.n-col {
  margin-top: 10px;
}
.n-col:nth-of-type(1),
.n-col:nth-of-type(2) {
  margin-top: 0px;
}

::v-deep(.n-form-item-label) {
  align-items: center !important;
  padding-bottom: 0px !important;
}

.run_as_admin {
  --n-label-height: 0px !important;
}

::v-deep(.n-input--focus:hover) {
  border-color: red;
  background-color: inherit !important;
}

::v-deep(.n-input) {
  transition: none !important;
}

.n-input * {
  transition: none !important;
}

::v-deep(.n-input-number > .n-input) {
  --n-caret-color: inherit !important;
  --n-border-hover: inherit !important;
  --n-border-focus: inherit !important;
  --n-box-shadow-focus: none !important;
  transition: none !important;
}

/* prettier-ignore */
::v-deep(.n-input-number > .n-input * ) {
  --n-caret-color: inherit !important;
  --n-border-hover: inherit !important;
  --n-border-focus: inherit !important;
  --n-box-shadow-focus: none !important;
  transition: none !important;
}

::v-deep(.n-input--textarea) {
  min-height: 80px !important;
}

::v-deep(.n-input-wrapper) {
  resize: none !important;
}
</style>
