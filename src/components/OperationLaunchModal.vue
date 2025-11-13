<template>
  <n-modal
    v-model:show="modalStatus"
    transform-origin="center"
    :mask-closable="false"
    @close="handleClose"
  >
    <n-card
      size="small"
      role="dialog"
      aria-modal="true"
      :bordered="false"
      :title="isEdit ? '编辑项目' : '新建项目'"
    >
      <template #header-extra>
        <n-icon size="20" class="cursor-pointer" @click="handleClose">
          <Close />
        </n-icon>
      </template>
      <n-tabs
        v-model:value="form.type"
        animated
        type="bar"
        placement="left"
        size="small"
        :default-value="form.type"
        :on-update:value="handleTypeChange"
        :style="`--n-tabs-nav-visbile: ${typesBarVisible}`"
      >
        <n-tab-pane
          v-for="item in launchTypes"
          :key="item.value"
          :disabled="isEdit || urlInfoLoading"
          :name="item.value"
          :tab="item.label"
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
              <!-- {{ isEdit }} <br /> -->
              <!-- {{ currentFormSchemas }} -->
              <!-- {{ form }} -->
              <n-row>
                <template v-for="sItem in currentFormSchemas" :key="sItem.prop">
                  <n-col
                    v-if="sItem.slot === 'iconSlot'"
                    :span="(sItem.span as any)"
                    class="!flex justify-start items-end"
                  >
                    <n-avatar
                      :style="form.icon ? 'background-color: transparent' : ''"
                      size="large"
                      :src="form.icon || ''"
                    />
                  </n-col>

                  <n-col
                    v-else-if="sItem.slot === 'pathSlot'"
                    :span="(sItem.span as any)"
                  >
                    <n-form-item :label="sItem.label" :path="sItem.prop">
                      <n-input
                        v-model:value="(form[sItem.prop] as any)"
                        placeholder=""
                        :type="sItem.type || 'text'"
                        :theme-overrides="inputTheme"
                      />
                    </n-form-item>
                    <n-button
                      class="!mt-1"
                      size="small"
                      color="lightgray"
                      text-color="gary"
                      @click="handleSelectLaunch"
                    >
                      选 择
                    </n-button>
                    *支持拖拽
                  </n-col>

                  <n-col
                    v-else-if="sItem.slot === 'urlSlot'"
                    :span="(sItem.span as any)"
                  >
                    <n-form-item :label="sItem.label" :path="sItem.prop">
                      <n-input
                        v-model:value="(form[sItem.prop] as any)"
                        placeholder="https://www.bilibili.com"
                        :type="sItem.type || 'text'"
                        :theme-overrides="inputTheme"
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

                    <n-checkbox
                      v-model:checked="appConfigStore.proxy"
                      class="ml-3"
                      size="small"
                      :checked-value="true"
                      :unchecked-value="false"
                      :default-checked="appConfigStore.proxy"
                      :on-update:checked="handleSwitchProxy"
                    >
                      代理
                    </n-checkbox>
                  </n-col>

                  <n-col
                    v-else-if="sItem.slot === 'hotkeySlot'"
                    :span="(sItem.span as any)"
                  >
                    <n-form-item :label="sItem.label" :path="sItem.prop">
                      <n-input
                        v-model:value="(form[sItem.prop] as any)"
                        placeholder=""
                        :type="sItem.type || 'text'"
                        :theme-overrides="inputTheme"
                      />
                    </n-form-item>
                    <!-- class="mt-1" -->
                    <n-checkbox
                      v-model:checked="form.hotkey_global"
                      size="small"
                      :checked-value="1"
                      :unchecked-value="0"
                      :default-checked="0"
                      :disabled="!form.hotkey"
                    >
                      全局快捷键
                    </n-checkbox>
                  </n-col>

                  <n-col
                    v-else-if="sItem.slot === 'runAsAdminSlot'"
                    :span="(sItem.span as any)"
                  >
                    <n-form-item
                      :label="sItem.label"
                      :path="sItem.prop"
                      class="run_as_admin"
                    >
                      <n-checkbox
                        v-model:checked="form.run_as_admin"
                        size="small"
                        :checked-value="1"
                        :unchecked-value="0"
                        :default-checked="0"
                        :disabled="form.extension !== 'exe'"
                      >
                        以管理员身份运行
                      </n-checkbox>
                    </n-form-item>
                  </n-col>

                  <!-- 网址选择指定浏览器打开 -->
                  <n-col
                    v-else-if="sItem.slot === 'browserSlot'"
                    :span="(sItem.span as any)"
                  >
                    <n-form-item :label="sItem.label" :path="sItem.prop">
                      <template #label>
                        <div class="flex">
                          <span class="mr-1">{{ sItem.label }}</span>

                          <n-tooltip trigger="hover">
                            <template #trigger>
                              <n-icon
                                size="16"
                                class="cursor-pointer"
                                @click="handleClose"
                              >
                                <AlertCircleOutline />
                              </n-icon>
                            </template>
                            按照以下格式添加自定义浏览器
                            <br />
                            浏览器名称=浏览器exe文件地址
                            <br />
                            例: QQ浏览器=C:\Application\QQBrowser\QQBrowser.exe
                          </n-tooltip>
                        </div>
                      </template>
                      <!--           @change="handleChangeOpenBrowser"
                        @click="handleChangeOpenBrowser" -->
                      <n-dynamic-tags
                        v-model:value="(browserOptions as any)"
                        size="small"
                        :render-tag="handleRenderBrowserTag"
                        @create="handleCreateBrowserOption"
                      />

                      <n-button
                        class="!ml-2"
                        size="tiny"
                        title="重置默认选项"
                        @click="handleSetDefaultBrowserOptions"
                      >
                        <n-icon size="16">
                          <RefreshOutline />
                        </n-icon>
                      </n-button>
                    </n-form-item>
                  </n-col>

                  <!-- 关键字 -->
                  <n-col
                    v-else-if="sItem.slot === 'keywordsSlot'"
                    :span="(sItem.span as any)"
                  >
                    <n-form-item :label="sItem.label" :path="sItem.prop">
                      <n-dynamic-tags
                        v-model:value="keywordsTags"
                        type="info"
                        size="small"
                      />
                    </n-form-item>
                  </n-col>

                  <!-- 排序 -->
                  <n-col
                    v-else-if="sItem.slot === 'orderSlot'"
                    :span="(sItem.span as any)"
                  >
                    <n-form-item :label="sItem.label" :path="sItem.prop">
                      <!-- borderFocus: 'inherit', -->
                      <!-- boxShadowFocus: 'none', -->
                      <!-- caretColor: 'inherit', -->
                      <!-- borderHover: 'inherit', -->
                      <n-input-number
                        v-model:value.number="form.order_index"
                        placeholder=""
                        class="w-25"
                        size="small"
                      />
                      <span class="ml-2 text-gray-400">
                        用于搜索返回展示的优先级 数字越小越靠前
                      </span>
                    </n-form-item>
                  </n-col>

                  <!-- 启动项分类 -->
                  <n-col
                    v-else-if="sItem.slot === 'categorySlot'"
                    :span="(sItem.span as any)"
                  >
                    <n-form-item :label="sItem.label" :path="sItem.prop">
                      <!-- TODO 分类切换 重置子分类 -->
                      <!-- {{ form }} -->
                      <n-select
                        v-model:value="form.category_id"
                        clearable
                        placeholder=""
                        :default-value="null"
                        :options="(categoryOptions as any)"
                        :disabled="activeCategory !== -1"
                      />
                    </n-form-item>
                  </n-col>

                  <n-col
                    v-else-if="sItem.slot === 'enabledSlot'"
                    :span="(sItem.span as any)"
                  >
                    <n-form-item :label="sItem.label" :path="sItem.prop">
                      <n-switch
                        v-model:value="form.enabled"
                        :default-value="1"
                        :checked-value="1"
                        :unchecked-value="0"
                      />
                    </n-form-item>
                  </n-col>

                  <n-col v-else :span="(sItem.span as any)">
                    <n-form-item :label="sItem.label" :path="sItem.prop">
                      <n-input
                        v-model:value="(form[sItem.prop] as any)"
                        :theme-overrides="inputTheme"
                        :type="sItem.type || 'text'"
                        :placeholder="sItem.placeholder || ''"
                      />
                    </n-form-item>
                  </n-col>
                </template>
              </n-row>
            </n-form>
          </div>
        </n-tab-pane>
      </n-tabs>

      <template #footer>
        <div class="flex justify-end gap-4">
          <n-button
            size="small"
            type="info"
            :disabled="!form.name || !form.path"
            @click="handleConfirm"
          >
            确 认
          </n-button>
          <n-button size="small" @click="handleClose">取 消</n-button>
        </div>
      </template>
    </n-card>
  </n-modal>
</template>

<script setup lang="tsx">
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { open } from '@tauri-apps/plugin-dialog';
import {
  AlertCircleOutline,
  Close,
  LogoChrome,
  LogoEdge,
  LogoFirefox,
  RefreshOutline,
} from '@vicons/ionicons5';
import { storeToRefs } from 'pinia';
import { computed, ref } from 'vue';
import { addLaunch, getFileInfo, getWebsiteInfo, updateLaunch } from '@/api';
import { useAppConfig } from '@/composables/useAppConfig';
import { useNaiveUiApi } from '@/composables/useNaiveUiApi';
import { AppEvent } from '@/constant';
import { useStore } from '@/store/useStore';
import { EventBus } from '@/utils/eventBus';

// 表单字段 schema 类型
interface FieldSchema {
  prop: keyof NewLaunchItem;

  label: string;

  type?: 'text' | 'textarea';

  slot?: string;

  span?: number;

  placeholder?: string;
}

type LaunchItemType = NewLaunchItem['type'];

const modalStatus = defineModel<boolean>({ default: true });

const store = useStore();
const { categoryOptions, activeCategory } = storeToRefs(store);

const { appConfigStore } = useAppConfig();
const { message } = useNaiveUiApi();

const launchTypes = [
  { value: 'file', label: '文 件' },
  { value: 'directory', label: '文件夹' },
  { value: 'url', label: '网 站' },
] as const satisfies OptionItem<LaunchItemType>[];
// ] as const satisfies { value: LaunchItemType; label: string }[]

const formSchemas: Record<LaunchItemType, FieldSchema[]> = {
  file: [
    { prop: 'icon', label: '', span: 3, slot: 'iconSlot' },
    { prop: 'name', label: '名称', span: 17 },
    {
      prop: 'path',
      label: '路径',
      type: 'textarea',
      span: 20,
      slot: 'pathSlot',
    },

    {
      prop: 'start_dir',
      label: '起始位置',
      span: 20,
      placeholder: 'C:\\Windows\\System32',
    },
    {
      prop: 'args',
      label: '启动参数',
      span: 20,
      placeholder: 'chrome.exe --incognito',
    },
    { prop: 'run_as_admin', label: '', span: 20, slot: 'runAsAdminSlot' },

    { prop: 'keywords', label: '搜索关键字', span: 20, slot: 'keywordsSlot' },
    { prop: 'category_id', label: '分类', span: 20, slot: 'categorySlot' },
    {
      prop: 'subcategory_id',
      label: '子分类',
      span: 20,
      slot: 'subCategorySlot',
    },

    // { prop: 'hotkey', label: '快捷键', span: 20, slot: 'hotkeySlot' },
    { prop: 'enabled', label: '启用搜索', span: 20, slot: 'enabledSlot' },
    { prop: 'order_index', label: '排序', span: 20, slot: 'orderSlot' },
    { prop: 'remarks', label: '备注', type: 'textarea', span: 20 },
  ],
  directory: [
    { prop: 'icon', label: '', span: 3, slot: 'iconSlot' },
    { prop: 'name', label: '名称', span: 17 },
    {
      prop: 'path',
      label: '路径',
      type: 'textarea',
      span: 20,
      slot: 'pathSlot',
    },

    { prop: 'keywords', label: '搜索关键字', span: 20, slot: 'keywordsSlot' },
    { prop: 'category_id', label: '分类', span: 20, slot: 'categorySlot' },
    {
      prop: 'subcategory_id',
      label: '子分类',
      span: 20,
      slot: 'subCategorySlot',
    },

    // { prop: 'hotkey', label: '快捷键', span: 20, slot: 'hotkeySlot' },
    { prop: 'enabled', label: '启用搜索', span: 20, slot: 'enabledSlot' },
    { prop: 'order_index', label: '排序', span: 20, slot: 'orderSlot' },
    { prop: 'remarks', label: '备注', type: 'textarea', span: 20 },
  ],
  url: [
    { prop: 'icon', label: '', span: 3, slot: 'iconSlot' },
    { prop: 'name', label: '名称', span: 17 },
    {
      prop: 'path',
      label: '网址',
      type: 'textarea',
      span: 20,
      slot: 'urlSlot',
    },
    { prop: 'args', label: '浏览器', span: 20, slot: 'browserSlot' },

    { prop: 'keywords', label: '搜索关键字', span: 20, slot: 'keywordsSlot' },
    { prop: 'category_id', label: '分类', span: 20, slot: 'categorySlot' },
    {
      prop: 'subcategory_id',
      label: '子分类',
      span: 20,
      slot: 'subCategorySlot',
    },

    // { prop: 'hotkey', label: '快捷键', span: 20, slot: 'hotkeySlot' },
    { prop: 'enabled', label: '启用搜索', span: 20, slot: 'enabledSlot' },
    { prop: 'order_index', label: '排序', span: 20, slot: 'orderSlot' },
    { prop: 'remarks', label: '备注', type: 'textarea', span: 20 },
  ],
};

const LOCAL_BROWSER_KEY = 'local_browser_key';

const defaultBrowserOptions = ref([
  {
    label: '默认',
    value: '',
  },
  {
    label: 'Chrome',
    value: 'chrome',
  },
  {
    label: 'Edge',
    value: 'msedge',
  },
  {
    label: 'Firefox',
    value: 'firefox',
  },
]);

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
  subcategory_id: null,
  extension: null,
});
const currentFormSchemas = computed(() => formSchemas[form.value.type]);

const keywordsTags = computed({
  get: () => (form.value?.keywords || '').split(',').filter(item => item),
  set: val => (form.value.keywords = val.join()),
});

const formRules = ref({});

const inputTheme = {
  borderFocus: 'inherit',
  boxShadowFocus: 'none',
  caretColor: 'inherit',
  borderHover: 'inherit',
};

// 浏览器选择
const baseBrowserOptions = ref<OptionItem[]>(
  localStorage.getItem(LOCAL_BROWSER_KEY)
    ? JSON.parse(localStorage.getItem(LOCAL_BROWSER_KEY) as any)
    : defaultBrowserOptions.value
);

const browserOptions = computed<OptionItem[]>({
  get: () => baseBrowserOptions.value,
  set: val =>
    (baseBrowserOptions.value = val.filter(item => item.value !== undefined)),
});

function handleSetDefaultBrowserOptions() {
  baseBrowserOptions.value = JSON.parse(
    JSON.stringify(defaultBrowserOptions.value)
  );
  saveBrowserOption();
}

function saveBrowserOption() {
  nextTick(() => {
    localStorage.setItem(
      LOCAL_BROWSER_KEY,
      JSON.stringify(baseBrowserOptions.value)
    );
  });
}

function handleCreateBrowserOption(newTag: string) {
  const [label, value] = newTag.split('=');

  if (!value || !label) message.warning('输入信息有误');
  else saveBrowserOption();

  return {
    label,
    value,
  };
}

function handleDeleteBrowserOption(item: OptionItem) {
  // 当删除的是当前选中的浏览器 则重置为默认
  if (form.value.args === item.value) form.value.args = '';
  // baseBrowserOptions.value
  const delIndex = baseBrowserOptions.value.findIndex(
    tag => tag.value === item.value
  );
  baseBrowserOptions.value.splice(delIndex, 1);
  // 持久化保存到本地
  saveBrowserOption();
}

const browserIcons: Record<string, any> = {
  chrome: LogoChrome,
  msedge: LogoEdge,
  firefox: LogoFirefox,
};

function handleRenderBrowserTag(tag: OptionItem, index: number) {
  const IconComponent = browserIcons[tag.value as string];

  const NIcon = (
    <n-icon size="16" style={{ paddingTop: '1px' }}>
      <IconComponent />
    </n-icon>
  );

  return (
    <n-tag
      size="small"
      closable={index !== 0}
      style="cursor: pointer;"
      title={tag.value}
      type={form.value.args === tag.value ? 'info' : ''}
      onClick={() => (form.value.args = tag.value as any)}
      onClose={() => handleDeleteBrowserOption(tag)}
    >
      {IconComponent ? NIcon : tag.label}
    </n-tag>
  );
}

const urlInfoLoading = ref(false);

function handleClose() {
  modalStatus.value = false;
  urlInfoLoading.value = false;
  initForm();
}

async function getUrlInfo() {
  try {
    if (!form.value.path.trim()) return;
    // TODO 优化 输入的url进行补全
    if (
      !(
        form.value.path.includes('http://') ||
        form.value.path.includes('https://')
      )
    ) {
      form.value.path = `https://${form.value.path}`;
    }
    urlInfoLoading.value = true;
    const data: any = await getWebsiteInfo(form.value.path);
    form.value.icon = data.icon;
    form.value.name = data.title;
  } catch (e) {
    message.error(e as string);
  } finally {
    urlInfoLoading.value = false;
  }
}

function initForm() {
  form.value.type = launchTypes[0].value;
  form.value.name = '';
  form.value.path = '';
  form.value.icon = '';
  form.value.hotkey = '';
  form.value.hotkey_global = 0;
  form.value.keywords = '';
  form.value.start_dir = '';
  form.value.remarks = '';
  form.value.args = '';
  form.value.run_as_admin = 0;
  form.value.order_index = 0;
  form.value.enabled = 1;
  form.value.category_id = null;
  form.value.subcategory_id = null;

  form.value.extension = null;
}

function handleTypeChange(val: LaunchItemType) {
  initForm();
  nextTick(() => {
    form.value.type = val;
  });
}

function setForm(fileInfo: FileInfo) {
  // 重新选择文件/文件夹时 将该参数重置
  form.value.run_as_admin = 0;
  form.value.hotkey_global = 0;

  form.value.name = fileInfo.name;
  form.value.path = fileInfo.path;
  form.value.icon = fileInfo.icon;
  form.value.type = fileInfo.type;
  form.value.extension = fileInfo.extension;
  form.value.args = fileInfo.args;
  form.value.remarks = fileInfo.remarks;
}

async function handleSelectLaunch() {
  const path = await open({
    multiple: false,
    directory: form.value.type === 'directory',
  });

  const fileInfo = await getFileInfo(path!);
  setForm(fileInfo);
}

function handleSwitchProxy(val: boolean) {
  appConfigStore.proxy = val;
  if (appConfigStore.proxyHost && val) {
    appConfigStore.proxy = true;
  } else if (!appConfigStore.proxyHost && val) {
    message.warning('请先设置代理地址');
    appConfigStore.proxy = false;
  } else {
    appConfigStore.proxy = false;
  }
}

const editItem = ref<LaunchItem>();

const isEdit = ref<boolean>(false);

async function handleConfirm() {
  if (isEdit.value) {
    const item: LaunchItem = JSON.parse(
      JSON.stringify({
        ...editItem.value,
        ...form.value,
      })
    );
    // TODO 错误处理
    await updateLaunch(item);
  } else {
    await addLaunch(form.value);
  }
  EventBus.emit(AppEvent.UPDATE_LAUNCH_LIST);
  handleClose();
}

const typesBarVisible = computed(() => (isEdit.value ? 'none' : 'initial'));

// 打开对话框
EventBus.listen<LaunchItem | undefined>(AppEvent.OPEN_OPERATION_LAUNCH, val => {
  isEdit.value = !!val;
  editItem.value = val;

  if (isEdit.value) {
    for (const key in form.value) {
      // @ts-expect-error
      form.value[key] = val[key];
    }
  } else {
    // 新建时 设置默认选中的分类
    if (activeCategory.value !== -1)
      form.value.category_id = activeCategory.value;
  }

  modalStatus.value = true;
});

getCurrentWebviewWindow().onDragDropEvent(async e => {
  // 当添加对话框没打开时不触发后续操作 防止和外层 拖拽事件相互影响
  if (!modalStatus.value) return;
  if (e.payload.type === 'drop') {
    if (!e.payload.paths.length) return;
    const path = e.payload.paths[0];
    const fileInfo = await getFileInfo(path);

    // 当拖进来的启动项 不符合当前类型 则初始表单 并切换到对应的类型
    if (fileInfo.type !== form.value.type) {
      initForm();
      nextTick(() => {
        setForm(fileInfo);
      });
    } else {
      setForm(fileInfo);
    }
  }
});
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

/* prettier-ignore */
::v-deep(.n-base-selection__state-border) {
  transition: none !important;
}
/* prettier-ignore */
::v-deep(.n-base-selection) {
  --n-caret-color: inherit !important;
  --n-border-hover: inherit !important;
  --n-border-focus: inherit !important;
  --n-box-shadow-focus: none !important;
  --n-border-active: inherit !important;
  --n-border-focus: inherit !important;
  --n-border-hover: inherit !important;
  --n-box-shadow-active: inherit !important;
  --n-box-shadow-focus: inherit !important;
  transition: none !important;
}

::v-deep(.n-input--textarea) {
  min-height: 80px !important;
}

::v-deep(.n-input-wrapper) {
  resize: none !important;
}

::v-deep(.n-tabs-nav) {
  display: var(--n-tabs-nav-visbile);
}

::v-deep(.n-form-item-label__text) {
  display: flex;
  align-items: center;
}
</style>
