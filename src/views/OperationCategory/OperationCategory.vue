<template>
  <n-card
    data-tauri-drag-region
    size="small"
    role="dialog"
    aria-modal="true"
    label-placement="left"
    :bordered="false"
    class="h-full px-5 pt-3 pb-5"
  >
    <template #header>
      <div class="flex items-center gap-2">
        <TrafficLights />
        <span>{{ isEdit ? t("category.editCategory") : t("category.newCategory") }}</span>
      </div>
    </template>
    <template #header-extra>
      <n-icon size="20" class="cursor-pointer" @click="handleClose">
        <Close />
      </n-icon>
    </template>

    <n-form
      ref="formRef"
      size="small"
      label-width="80"
      :show-feedback="false"
      :model="form"
      label-placement="left"
      class="h-full"
    >
      <n-row class="h-full" align-items="center" justify-content="space-between">
        <!-- {{ form }} -->
        <n-col span="22">
          <n-form-item label=" " path="name" class="icon-item">
            <n-avatar
              size="large"
              :style="form.icon ? 'background-color: transparent' : ''"
              :src="form.icon || ''"
            />

            <IconPicker v-model="form.icon!" ref="iconPickerRef" />
          </n-form-item>
        </n-col>

        <n-col span="22">
          <n-form-item :label="t('category.labelName')" path="name">
            <n-input
              v-model:value="form.name"
              tabindex="1"
              placeholder=""
              type="text"
              :theme-overrides="inputTheme"
            />
          </n-form-item>
        </n-col>

        <n-col span="22">
          <n-form-item :label="t('category.labelDir')" path="association_directory">
            <n-input
              v-model:value="form.association_directory"
              tabindex="1"
              placeholder=""
              type="textarea"
              readonly
              :theme-overrides="inputTheme"
            />
            <!-- @click="handleSelectDir" -->
          </n-form-item>
          <n-button
            style="margin-left: 80px"
            class="mt-1!"
            size="small"
            color="lightgray"
            text-color="gary"
            :disabled="isEdit"
            @click="handleSelectDir"
          >
            {{ t("common.select") }}
          </n-button>
          <span class="ml-1 text-muted-foreground text-[12px]">
            {{ t("category.dirHint") }}
          </span>
        </n-col>

        <n-col span="22">
          <n-form-item :label="t('category.labelExclude')" path="exclude">
            <div class="flex-s-c">
              <n-switch v-model:value="form.exclude" size="small" />
            </div>
          </n-form-item>
        </n-col>
      </n-row>
    </n-form>

    <template #footer>
      <div class="flex justify-end gap-4">
        <n-button
          size="small"
          type="info"
          :loading="loading"
          :disabled="!form.name"
          @click="handleConfirm"
        >
          {{ t("common.confirm") }}
        </n-button>
        <n-button size="small" @click="handleClose">
          {{ t("common.cancel") }}
        </n-button>
      </div>
    </template>
  </n-card>
</template>

<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";
import { Close } from "@vicons/ionicons5";
import { ref } from "vue";
import { addCategory, updateCategory, updateLaunchEnabledByCategory } from "@/api";
import IconPicker from "@/components/IconPicker.vue";
import TrafficLights from "@/components/TrafficLights.vue";
import {
  useCategoryCorrelationDir,
  useEsc,
  useFormState,
  useLoading,
  useNaiveUiApi,
  useToggleWindowVisible,
} from "@/composables";
import { AppEvent } from "@/constant";
import { t } from "@/i18n";
import { EventBus } from "@/utils/eventBus";

const { message } = useNaiveUiApi();
const { handleCreateLaunchFromCategoryDir, registerAllCategoryDirWatch } =
  useCategoryCorrelationDir();
const { getOperCategoryWindow, toogleOperCategoryWindowVisible } = useToggleWindowVisible();
const inputTheme = {
  borderFocus: "inherit",
  boxShadowFocus: "none",
  caretColor: "inherit",
  borderHover: "inherit",
};

const modalStatus = ref(true);

const { form, initForm, setForm } = useFormState<NewCategoryItem>({
  icon: "",
  name: "",
  parent_id: null,
  association_directory: "",
  exclude: false,
  layout: "grid",
  sort_by: "default",
  sort_order: "default",
  order_index: 0,
});

async function handleClose() {
  initForm();
  const window = await getOperCategoryWindow();
  window?.hide();
}

const iconPickerRef = useTemplateRef("iconPickerRef");
async function handleSelectDir() {
  const path = await open({
    multiple: false,
    directory: true,
  });
  if (!path) return;
  form.value.association_directory = path;
  const arr = path.split("\\");
  // if (!form.value.name)
  form.value.name = arr[arr.length - 1];
  // form.value.name || (form.value.name = arr[arr.length - 1]);
  // form.value.icon = await getLocalIconBase64(path);
  // form.value.icon = await getLocalIconBase64(path);
  iconPickerRef.value?.handleGetLocalDirIcon(path);
}

const isEdit = ref<boolean>(false);

const { loading, startLoading, stopLoading } = useLoading();

const editItem = ref<CategoryItem>();
async function handleConfirm() {
  try {
    startLoading();
    if (isEdit.value) {
      const item = {
        ...editItem.value,
        ...form.value,
      };
      await updateCategory(item);
      // 更新启动项的排除搜索
      await updateLaunchEnabledByCategory(item.id as number, item.exclude !== true);

      EventBus.emit(AppEvent.UPDATE_CATEGORY_LIST);
    } else {
      const res = await addCategory(form.value);
      // 如果有关联目录 创建该目录下的启动项
      await handleCreateLaunchFromCategoryDir(res);
      // 更新分类数据 并选中新创建的分类
      EventBus.emit(AppEvent.ACTIVE_CATEGORY, res);
      registerAllCategoryDirWatch();
    }
    handleClose();
  } catch (e) {
    message.error(e as string);
  } finally {
    stopLoading();
  }
}

// 打开对话框
EventBus.listen<typeof editItem.value>(AppEvent.OPEN_OPERATION_CATEGORY, async (val) => {
  initForm();
  isEdit.value = !!val;
  editItem.value = val;
  modalStatus.value = true;
  if (val) setForm(val);

  toogleOperCategoryWindowVisible();
  const window = await getOperCategoryWindow();
  window?.setTitle(isEdit.value ? t("category.editCategory") : t("category.newCategory"));
});

useEsc(handleClose);
</script>

<style scoped lang="scss">
.n-input * {
  transition: none !important;
}

::v-deep(.n-input:not(.n-input--disabled).n-input--focus) {
  background: var(--n-color) !important;
}

::v-deep(.n-card-header),
::v-deep(.n-card-content),
::v-deep(.n-card__footer) {
  padding: 0 !important;
}

::v-deep(.n-input) {
  transition: none !important;
}

::v-deep(.n-input-wrapper) {
  resize: none !important;
}

::v-deep(.icon-item .n-form-item-blank) {
  display: flex;
  align-items: end;
  gap: 10px;
}
</style>
