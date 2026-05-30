<template>
  <n-card
    data-tauri-drag-region
    size="small"
    role="dialog"
    aria-modal="true"
    :bordered="false"
    :title="isEdit ? t('launch.editItem') : t('launch.newItem')"
    class="h-full px-5 pt-3 pb-5"
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
          <!-- {{ { ...form, icon: '' } }} -->
          <!-- -- {{ appConfigStore }} -->

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
              <template
                v-for="sItem in currentFormSchemas"
                :key="sItem.prop"
              >
                <n-col
                  v-if="sItem.slot === 'iconSlot'"
                  :span="(sItem.span as any)"
                  class="!flex justify-start items-end"
                >
                  <NAvatar
                    size="large"
                    :style="form.icon ? 'background-color: transparent' : ''"
                    :src="form.icon || ''"
                  />
                </n-col>
                <n-col
                  v-else-if="sItem.slot === 'selectIconSlot'"
                  :span="(sItem.span as any)"
                  class="!flex justify-start items-end"
                >
                  <IconPicker v-model="form.icon!" />
                </n-col>

                <n-col
                  v-else-if="sItem.slot === 'pathSlot'"
                  :span="(sItem.span as any)"
                >
                  <n-form-item
                    :label="sItem.label"
                    :path="sItem.prop"
                  >
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
                    {{ t('common.select') }}
                  </n-button>
                  {{ t('common.dragSupport') }}
                </n-col>

                <n-col
                  v-else-if="sItem.slot === 'appsSelectSlot'"
                  :span="(sItem.span as any)"
                >
                  <n-form-item
                    :label="sItem.label"
                    :path="sItem.prop"
                  >
                    <!-- <n-input
                        v-model:value="(form[sItem.prop] as any)"
                        placeholder=""
                        :type="sItem.type || 'text'"
                        :theme-overrides="inputTheme"
                      /> -->
                    <!-- {{ appsSelectValue }}
                      {{ form.path }} -->
                    <n-select
                      v-model:value="appsSelectValue"
                      multiple
                      filterable
                      placeholder=""
                      :options="options"
                      :render-label="renderLabel"
                      :render-tag="renderMultipleSelectTag"
                    />
                  </n-form-item>
                </n-col>

                <n-col
                  v-else-if="sItem.slot === 'urlSlot'"
                  :span="(sItem.span as any)"
                >
                  <n-form-item
                    :label="sItem.label"
                    :path="sItem.prop"
                  >
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
                    {{ t('common.fetchUrlInfo') }}
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
                    {{ t('common.proxy') }}
                  </n-checkbox>
                </n-col>

                <n-col
                  v-else-if="sItem.slot === 'hotkeySlot'"
                  :span="(sItem.span as any)"
                >
                  <n-form-item
                    :label="sItem.label"
                    :path="sItem.prop"
                  >
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
                    :checked-value="true"
                    :unchecked-value="false"
                    :default-checked="false"
                    :disabled="!form.hotkey"
                  >
                    {{ t('general.shortcutKeyTitle') }}
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
                      :checked-value="true"
                      :unchecked-value="false"
                      :default-checked="false"
                      :disabled="form.extension !== 'exe'"
                    >
                      {{ t('common.adminRun') }}
                    </n-checkbox>
                  </n-form-item>
                </n-col>

                <!-- 网址选择指定浏览器打开 -->
                <n-col
                  v-else-if="sItem.slot === 'browserSlot'"
                  :span="(sItem.span as any)"
                >
                  <n-form-item
                    :label="sItem.label"
                    :path="sItem.prop"
                  >
                    <template #label>
                      <div class="flex">
                        <span class="mr-1">{{ sItem.label }}</span>

                        <n-tooltip trigger="hover">
                          <template #trigger>
                            <n-icon
                              size="16"
                              class="cursor-pointer"
                            >
                              <!-- @click="handleClose" -->
                              <AlertCircleOutline />
                            </n-icon>
                          </template>
                          <span style="color: var(--muted-foreground)">
                            {{ t('launch.browserFormat') }}
                            <br />
                            {{ t('launch.browserExample') }}
                            <br />
                            {{ t('launch.browserExampleValue') }}
                          </span>
                        </n-tooltip>
                      </div>
                    </template>

                    <!--  -->
                    <template v-if="form.type === 'url'">
                      <BrowerPicker v-model="(form.args as string)" />
                    </template>
                  </n-form-item>
                </n-col>

                <!-- 关键字 -->
                <n-col
                  v-else-if="sItem.slot === 'keywordsSlot'"
                  :span="(sItem.span as any)"
                >
                  <n-form-item
                    :label="sItem.label"
                    :path="sItem.prop"
                  >
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
                  <n-form-item
                    :label="sItem.label"
                    :path="sItem.prop"
                  >
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
                    <span class="ml-2 text-muted-foreground">{{ t('launch.sortHint') }}</span>
                  </n-form-item>
                </n-col>

                <!-- 启动项分类 -->
                <n-col
                  v-else-if="sItem.slot === 'categorySlot'"
                  :span="(sItem.span as any)"
                >
                  <n-form-item
                    :label="sItem.label"
                    :path="sItem.prop"
                  >
                    <!-- TODO 使用级联选择器 -->
                    <n-select
                      v-model:value="form.category_id"
                      clearable
                      placeholder=""
                      :default-value="null"
                      :options="(categoryOptions as any)"
                    />
                  </n-form-item>
                </n-col>

                <n-col
                  v-else-if="sItem.slot === 'enabledSlot'"
                  :span="(sItem.span as any)"
                >
                  <n-form-item
                    :label="sItem.label"
                    :path="sItem.prop"
                  >
                    <n-switch
                      v-model:value="form.enabled"
                      :default-value="true"
                      :checked-value="true"
                      :unchecked-value="false"
                    />
                  </n-form-item>
                </n-col>

                <n-col
                  v-else
                  :span="(sItem.span as any)"
                >
                  <n-form-item
                    :label="sItem.label"
                    :path="sItem.prop"
                  >
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
    <!-- {{ editItemWithCategory }} -->
    <!-- {{ form }} -->

    <template #footer>
      <div class="flex justify-end gap-4">
        <n-button
          size="small"
          type="info"
          :disabled="!form.name || !form.path"
          @click="handleConfirm"
        >
          {{ t('common.confirm') }}
        </n-button>
        <n-button
          size="small"
          @click="handleClose"
        >
          {{ t('common.cancel') }}
        </n-button>
      </div>
    </template>
  </n-card>
</template>

<script setup lang="tsx">
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { open } from '@tauri-apps/plugin-dialog';
import { AlertCircleOutline, Close } from '@vicons/ionicons5';
import { NAvatar, NTag } from 'naive-ui';
import { storeToRefs } from 'pinia';
import { computed, nextTick, ref } from 'vue';
import { addLaunch, getCategoryTree, getFileInfo, getWebsiteInfo, updateLaunch } from '@/api';
import BrowerPicker from '@/components/BrowserPicker.vue';
import IconPicker from '@/components/IconPicker.vue';
import { useAppConfig, useEsc, useFormState, useNaiveUiApi, useToggleWindowVisible } from '@/composables';
import { AppEvent } from '@/constant';
import { t } from '@/i18n';
import piniaStore from '@/store';
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

const store = useStore(piniaStore);
const { categoryOptions, activeCategory } = storeToRefs(store);

const { appConfigStore } = useAppConfig();
const { message } = useNaiveUiApi();
const { getOperLaunchWindow, toogleOperLaunchWindowVisible } = useToggleWindowVisible();

const launchTypes = computed<OptionItem<LaunchItemType>[]>(() => [
  { value: 'file', label: t('launch.tabFile') },
  { value: 'directory', label: t('launch.tabFolder') },
  { value: 'url', label: t('launch.tabWebsite') },
  { value: 'apps', label: t('launch.tabApps') },
]);

function getFormSchemas(): Record<LaunchItemType, FieldSchema[]> {
  return {
    file: [
      { prop: 'icon', label: '', span: 3, slot: 'iconSlot' },
      { prop: 'name', label: t('common.name'), span: 17 },
      { prop: 'name', label: '', span: 20, slot: 'selectIconSlot' },
      {
        prop: 'path',
        label: t('common.path'),
        type: 'textarea',
        span: 20,
        slot: 'pathSlot',
      },

      {
        prop: 'start_dir',
        label: t('launch.labelStartDir'),
        span: 20,
        placeholder: 'C:\\Windows\\System32',
      },
      {
        prop: 'args',
        label: t('launch.labelArgs'),
        span: 20,
        placeholder: 'chrome.exe --incognito',
      },
      { prop: 'run_as_admin', label: '', span: 20, slot: 'runAsAdminSlot' },

      { prop: 'keywords', label: t('common.searchKeyword'), span: 20, slot: 'keywordsSlot' },
      { prop: 'category_id', label: t('common.category'), span: 20, slot: 'categorySlot' },
      {
        prop: 'subcategory_id',
        label: t('common.subcategory'),
        span: 20,
        slot: 'subCategorySlot',
      },

      // { prop: 'hotkey', label: '快捷键', span: 20, slot: 'hotkeySlot' },
      { prop: 'enabled', label: t('common.enableSearch'), span: 20, slot: 'enabledSlot' },
      { prop: 'order_index', label: t('common.sort'), span: 20, slot: 'orderSlot' },
      { prop: 'remarks', label: t('common.note'), type: 'textarea', span: 20 },
    ],
    directory: [
      { prop: 'icon', label: '', span: 3, slot: 'iconSlot' },
      { prop: 'name', label: t('common.name'), span: 17 },
      { prop: 'name', label: '', span: 20, slot: 'selectIconSlot' },
      {
        prop: 'path',
        label: t('common.path'),
        type: 'textarea',
        span: 20,
        slot: 'pathSlot',
      },

      { prop: 'keywords', label: t('common.searchKeyword'), span: 20, slot: 'keywordsSlot' },
      { prop: 'category_id', label: t('common.category'), span: 20, slot: 'categorySlot' },
      {
        prop: 'subcategory_id',
        label: t('common.subcategory'),
        span: 20,
        slot: 'subCategorySlot',
      },

      // { prop: 'hotkey', label: '快捷键', span: 20, slot: 'hotkeySlot' },
      { prop: 'enabled', label: t('common.enableSearch'), span: 20, slot: 'enabledSlot' },
      { prop: 'order_index', label: t('common.sort'), span: 20, slot: 'orderSlot' },
      { prop: 'remarks', label: t('common.note'), type: 'textarea', span: 20 },
    ],
    url: [
      { prop: 'icon', label: '', span: 3, slot: 'iconSlot' },
      { prop: 'name', label: t('common.name'), span: 17 },
      { prop: 'name', label: '', span: 20, slot: 'selectIconSlot' },
      {
        prop: 'path',
        label: t('common.url'),
        type: 'textarea',
        span: 20,
        slot: 'urlSlot',
      },
      { prop: 'args', label: t('launch.labelBrowser'), span: 20, slot: 'browserSlot' },

      { prop: 'keywords', label: t('common.searchKeyword'), span: 20, slot: 'keywordsSlot' },
      { prop: 'category_id', label: t('common.category'), span: 20, slot: 'categorySlot' },
      {
        prop: 'subcategory_id',
        label: t('common.subcategory'),
        span: 20,
        slot: 'subCategorySlot',
      },

      // { prop: 'hotkey', label: '快捷键', span: 20, slot: 'hotkeySlot' },
      { prop: 'enabled', label: t('common.enableSearch'), span: 20, slot: 'enabledSlot' },
      { prop: 'order_index', label: t('common.sort'), span: 20, slot: 'orderSlot' },
      { prop: 'remarks', label: t('common.note'), type: 'textarea', span: 20 },
    ],
    apps: [
      { prop: 'icon', label: '', span: 3, slot: 'iconSlot' },
      { prop: 'name', label: t('common.name'), span: 17 },
      { prop: 'name', label: '', span: 20, slot: 'selectIconSlot' },
      {
        prop: 'path',
        label: t('launch.labelItemList'),
        type: 'textarea',
        span: 20,
        slot: 'appsSelectSlot',
      },
      // {
      //   prop: 'start_dir',
      //   label: '起始位置',
      //   span: 20,
      //   placeholder: 'C:\\Windows\\System32',
      // },
      {
        prop: 'args',
        label: t('launch.labelInterval'),
        span: 20,
        placeholder: '',
      },
      { prop: 'keywords', label: t('common.searchKeyword'), span: 20, slot: 'keywordsSlot' },
      { prop: 'category_id', label: t('common.category'), span: 20, slot: 'categorySlot' },
      {
        prop: 'subcategory_id',
        label: t('common.subcategory'),
        span: 20,
        slot: 'subCategorySlot',
      },

      // { prop: 'hotkey', label: '快捷键', span: 20, slot: 'hotkeySlot' },
      { prop: 'enabled', label: t('common.enableSearch'), span: 20, slot: 'enabledSlot' },
      { prop: 'order_index', label: t('common.sort'), span: 20, slot: 'orderSlot' },
      { prop: 'remarks', label: t('common.note'), type: 'textarea', span: 20 },
    ],
    alias: [],
  };
}

const {
  form,
  initForm,
  setForm: _setForm,
} = useFormState<NewLaunchItem>({
  name: '',
  lnk_name: '',
  path: '',
  type: launchTypes.value[0].value,
  icon: '',

  hotkey: '',
  hotkey_global: false,
  keywords: '',
  start_dir: '',
  remarks: '',
  args: '',
  run_as_admin: false,
  order_index: 0,
  enabled: true,
  category_id: null,
  subcategory_id: null,
  extension: null,
});

const currentFormSchemas = computed(() => getFormSchemas()[form.value.type]);

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
const isEdit = ref<boolean>(false);

const urlInfoLoading = ref(false);

async function handleClose() {
  urlInfoLoading.value = false;
  initForm();
  toogleOperLaunchWindowVisible();
}

async function getUrlInfo() {
  try {
    if (!form.value.path.trim()) return;

    if (!(form.value.path.includes('http://') || form.value.path.includes('https://'))) {
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

function handleTypeChange(val: LaunchItemType) {
  initForm();
  nextTick(() => {
    form.value.type = val;
    // 在指定分类下 新建启动项时 分类值回显
    if (!isEdit.value) form.value.category_id = activeCategory.value;
    // 当切换的为多项目类型时 设置默认图标
    if (!isEdit.value && form.value.type === 'apps') setAppsDefaultIcon();
  });
}
function setAppsDefaultIcon() {
  const svg = `<svg t="1778134928909" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="2818" width="200" height="200"><path d="M292.24751 0c-34.713988 0-72.68633 24.625627-72.686329 67.360176l0.313303 742.215151c0 42.671888 37.972341 67.360176 72.686329 67.360177h585.312936c34.713988 0 72.74899-24.688288 72.748991-67.422837l-0.313303-465.756456c0-19.174152-7.519275-37.471056-20.92865-51.131073L663.825041 21.93122C650.039702 7.895239 631.241513 0 611.628737 0H292.24751z m0.689267 803.81055L292.623474 73.062293h292.311835v293.251744h292.311834l0.313303 437.496513h-584.623669zM657.997602 293.251744V120.308408l169.622323 172.943336H657.997602zM73.373933 950.937707V292.311835h73.062293v658.563211h584.62367v73.124954h-584.62367A73.062293 73.062293 0 0 1 73.373933 950.937707z" p-id="2819"></path></svg>`;
  const defaultBase64 = `data:image/svg+xml;base64,${btoa(unescape(encodeURIComponent(svg)))}`;
  form.value.icon = defaultBase64;
}
function setForm(fileInfo: FileInfo) {
  // 重新选择文件/文件夹时 将该参数重置
  form.value.run_as_admin = false;
  form.value.hotkey_global = false;

  _setForm(fileInfo);
}

async function handleSelectLaunch() {
  const path =
    form.value.type === 'directory'
      ? await open({
          multiple: false,
          directory: true,
        })
      : await invoke<string>('open_file_with_lnk');

  const fileInfo = await getFileInfo(path!);
  setForm(fileInfo);
}

function handleSwitchProxy(val: boolean) {
  appConfigStore.proxy = val;
  if (appConfigStore.proxyHost && val) {
    appConfigStore.proxy = true;
  } else if (!appConfigStore.proxyHost && val) {
    message.warning(t('common.proxyFirst'));
    appConfigStore.proxy = false;
  } else {
    appConfigStore.proxy = false;
  }
}

const editItem = ref<LaunchItem>();

async function handleConfirm() {
  if (isEdit.value) {
    const item: LaunchItem = JSON.parse(
      JSON.stringify({
        ...editItem.value,
        ...form.value,
      }),
    );
    // TODO 错误处理
    await updateLaunch(item);
  } else {
    await addLaunch(form.value);
  }
  EventBus.emit(AppEvent.UPDATE_LAUNCH_LIST);
  handleClose();
}

// 获取当前编辑启动项 所属分类的详情
// const editItemWithCategory = computed(() => {
//   if (!isEdit.value) return null;
//   const findRes = store.categoryData.find(item => item.id === editItem.value?.category_id);
//   if (!findRes) return null;
//   return findRes;
// });

const typesBarVisible = computed(() => (isEdit.value ? 'none' : 'initial'));

const appsSelectValue = computed({
  get: () => (form.value.path.trim() ? form.value.path.split(',') : []),
  set: val => (form.value.path = val.join(',')),
});
const options = ref<any>([]);
// const iconMap = ref(new Map());
async function getAppsSelectOptions() {
  const data = await getCategoryTree();
  // console.log(`%c data ----`, 'color: #fff;background-color: #000;font-size: 18px', data);
  options.value = data.map(category => {
    const { id, name, icon, children = [] } = category;
    const childrenOption = children.map((item: LaunchItem) => {
      const { id, name, icon, type, enabled, pinyin_abbr, pinyin_full } = item;
      // iconMap.value.set(id, icon);
      let disabled = false;
      disabled = !enabled;
      // 当编辑项目时排除选择自身作为启动项 防止递归
      if (isEdit.value && editItem.value?.id === id) disabled = true;
      // 多任务中不支持添加新的多任务
      if (type === 'apps') disabled = true;
      return {
        label: name,
        value: `${id}`,
        icon,
        pinyin_abbr,
        pinyin_full,
        disabled,
      };
    });

    return {
      type: 'group',
      icon,
      label: name,
      key: id,
      children: childrenOption,
    };
  });
}
function renderLabel(option: any) {
  const { label, icon = '' } = option;
  if (option.type === 'group') return `${label}`;
  return h(
    <div class="flex">
      {/* {{ icon }} */}
      <img
        src={icon}
        alt="icon"
        class="w-4 h-4 mr-3 shrink-0 object-contain pointer-events-none"
      />

      <span>{label}</span>
    </div>,
  );
}
function renderMultipleSelectTag({ option, handleClose }: any) {
  const { icon, label } = option;
  return h(
    // size="small"
    <NTag
      closable
      onClose={e => {
        e.stopPropagation();
        handleClose();
      }}
    >
      <div class="flex">
        <img
          src={icon}
          alt="icon"
          class="w-4 h-4 mr-1 shrink-0 object-contain pointer-events-none"
        />
        <span>{label}</span>
      </div>
    </NTag>,
  );
}

const baseSelectionHeight = computed(() => {
  return form.value.type === 'apps' ? '80px' : 'initial';
});

// 拖拽
getCurrentWebviewWindow().onDragDropEvent(async e => {
  if (isEdit.value) return;
  // 当添加对话框没打开时不触发后续操作 防止和外层 拖拽事件相互影响
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
// 打开对话框
EventBus.listen<LaunchItem | undefined>(AppEvent.OPEN_OPERATION_LAUNCH, async val => {
  initForm();
  getAppsSelectOptions();
  isEdit.value = !!val;
  editItem.value = val;
  if (isEdit.value) {
    _setForm(val!);
  } else {
    // 新建时 设置默认选中的分类
    form.value.category_id = activeCategory.value;
  }

  modalStatus.value = true;

  // 判断当前窗口是否处于展示状态
  toogleOperLaunchWindowVisible();
  const window = await getOperLaunchWindow();
  window?.setTitle(isEdit.value ? t('launch.editLaunchItem') : t('launch.newLaunchItem'));
});

useEsc(handleClose);
</script>

<style scoped lang="scss">
.n-tabs {
  --background: initial !important;
}
.n-modal {
  padding: 10px;
  transition: none !important;
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

// ::v-deep(.n-input) {
//   transition: none !important;
//   // --n-color-focus: initial !important;
// }

::v-deep(.n-base-selection:not(.n-base-selection--disabled).n-base-selection--active .n-base-selection-tags) {
  background-color: initial !important;
}
::v-deep(.n-base-selection-tags) {
  background-color: initial !important;
}

.n-input * {
  transition: none !important;
}

::v-deep(.n-input:not(.n-input--disabled).n-input--focus) {
  background: var(--n-color) !important;
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

  background-color: var(--n-color);
}

::v-deep(.n-input--textarea) {
  min-height: 80px !important;
}
::v-deep(.n-base-selection) {
  min-height: v-bind('baseSelectionHeight') !important;
  // background-color: transparent !important;
  background-color: var(--n-color);
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

::v-deep(.n-base-select-menu .n-virtual-list) {
  max-height: 150px;
}
</style>
