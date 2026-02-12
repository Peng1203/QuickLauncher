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
      label-placement="left"
      :bordered="false"
      :title="isEdit ? `编辑${editItem?.name}分类` : '新建分类'"
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

      <n-form
        ref="formRef"
        size="small"
        label-width="80"
        :show-feedback="false"
        :model="form"
        label-placement="left"
      >
        <n-row>
          <!-- {{ form }} -->
          <n-col span="22">
            <n-form-item
              label=" "
              path="name"
              class="!flex justify-start items-end"
            >
              <n-avatar
                size="large"
                :style="form.icon ? 'background-color: transparent' : ''"
                :src="form.icon || ''"
              />

              <IconPicker v-model="form.icon!" />
            </n-form-item>
          </n-col>

          <n-col span="22">
            <n-form-item
              label="名称"
              path="name"
            >
              <n-input
                v-model:value="form.name"
                placeholder=""
                type="text"
                :theme-overrides="inputTheme"
              />
            </n-form-item>
          </n-col>

          <n-col span="22">
            <n-form-item
              label="关联目录"
              path="association_directory"
            >
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
              class="!mt-1"
              size="small"
              color="lightgray"
              text-color="gary"
              :disabled="isEdit"
              @click="handleSelectDir"
            >
              选 择
            </n-button>
            <span class="ml-1 text-gray-400">*关联指定目录后无法操作该分类下的启动项只能作为搜索结果</span>
          </n-col>

          <n-col span="22">
            <!-- TODO -->
            <n-form-item
              label="搜索排除"
              path="exclude"
            >
              <n-switch
                v-model:value="form.exclude"
                :checked-value="1"
                :unchecked-value="0"
              />
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
import { open } from '@tauri-apps/plugin-dialog';
import { Close } from '@vicons/ionicons5';
import { ref } from 'vue';
import { addCategory, updateCategory } from '@/api';
import IconPicker from '@/components/IconPicker.vue';
import { useCategoryCorrelationDir } from '@/composables/useCategoryCorrelationDir';
import { useFormState } from '@/composables/useFormState';
import { useLoading } from '@/composables/useLoading';
import { useNaiveUiApi } from '@/composables/useNaiveUiApi';
import { AppEvent } from '@/constant';
import { useStore } from '@/store/useStore';
import { EventBus } from '@/utils/eventBus';

const store = useStore();

const { message } = useNaiveUiApi();
const { handleCreateLaunchFromCategoryDir, registerAllCategoryDirWatch } = useCategoryCorrelationDir();

const inputTheme = {
  borderFocus: 'inherit',
  boxShadowFocus: 'none',
  caretColor: 'inherit',
  borderHover: 'inherit',
};

const modalStatus = defineModel<boolean>({ default: false });

const { form, initForm, setForm } = useFormState<NewCategoryItem>({
  icon: '',
  name: '',
  parent_id: 0,
  association_directory: '',
  exclude: 0,
  layout: 'list',
});

function handleClose() {
  modalStatus.value = false;
  initForm();
}

async function handleSelectDir() {
  const path = await open({
    multiple: false,
    directory: true,
  });
  if (!path) return;
  form.value.association_directory = path;
  const arr = path.split('\\');
  form.value.name || (form.value.name = arr[arr.length - 1]);
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
    } else {
      const res = await addCategory(form.value);
      // 如果有关联目录 创建该目录下的启动项
      await handleCreateLaunchFromCategoryDir(res);
      // 选中新创建的分类
      store.handleChangeCategory(res.id);
      // TODO 滚动到该分类

      registerAllCategoryDirWatch();
    }
    EventBus.emit(AppEvent.UPDATE_CATEGORY_LIST);
    handleClose();
  } catch (e) {
    message.error(e as string);
  } finally {
    stopLoading();
  }
}

// 打开对话框
EventBus.listen<typeof editItem.value>(AppEvent.OPEN_OPERATION_CATEGORY, val => {
  isEdit.value = !!val;
  editItem.value = val;

  if (val) setForm(val);

  modalStatus.value = true;
});
</script>

<style scoped>
.n-modal {
  padding: 10px;
}

.n-card {
  width: 600px;
  height: 350px;
}

.n-col {
  margin-top: 10px;
}

::v-deep(.n-card-header),
::v-deep(.n-card__content),
::v-deep(.n-card__footer) {
  padding: 0;
}

/* ::v-deep(.n-card__content) {
  max-height: 200px;
} */

::v-deep(.n-input) {
  transition: none !important;
}

.n-input * {
  transition: none !important;
}

::v-deep(.n-input-wrapper) {
  resize: none !important;
}
::v-deep(.n-form-item-blank) {
  display: flex;
  align-items: end;
  gap: 10px;
}
</style>
