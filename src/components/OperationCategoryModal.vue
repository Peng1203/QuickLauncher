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
      :title="isEdit ? '编辑分类' : '新建分类'"
    >
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
      >
        <n-row>
          <n-col span="22">
            <n-form-item label="名称" path="name">
              <n-input
                v-model:value="form.name"
                placeholder=""
                type="text"
                :theme-overrides="inputTheme"
              />
            </n-form-item>
          </n-col>

          <n-col span="22">
            <n-form-item label="关联目录" path="association_directory">
              <n-input
                v-model:value="form.association_directory"
                placeholder=""
                type="textarea"
                :theme-overrides="inputTheme"
              />
            </n-form-item>
            <n-button
              style="margin-left: 80px"
              class="!mt-1"
              size="small"
              color="lightgray"
              text-color="gary"
              @click="handleSelectDir"
            >
              选 择
            </n-button>
            <span class="ml-1 text-gray-400">
              *关联指定目录后无法操作该分类下的启动项只能作为搜索结果
            </span>
          </n-col>
        </n-row>
      </n-form>

      <template #footer>
        <div class="flex justify-end gap-4">
          <n-button
            size="small"
            type="info"
            :disabled="!form.name"
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

<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog';
import { Close } from '@vicons/ionicons5';
import { ref } from 'vue';
import { addCategory, updateCategory } from '@/api';
import { useNaiveUiApi } from '@/composables/useNaiveUiApi';
import { AppEvent } from '@/constant';
import { EventBus } from '@/utils/eventBus';

const { message } = useNaiveUiApi();

const inputTheme = {
  borderFocus: 'inherit',
  boxShadowFocus: 'none',
  caretColor: 'inherit',
  borderHover: 'inherit',
};

const modalStatus = defineModel<boolean>({ default: false });

const form = ref<NewCategoryItem>({
  name: '',
  parent_id: 0,
  association_directory: '',
});
function initForm() {
  form.value.name = '';
  form.value.parent_id = 0;
  form.value.association_directory = '';
}

function handleClose() {
  modalStatus.value = false;
  initForm();
}

async function handleSelectDir() {
  const path = await open({
    multiple: false,
    directory: true,
  });

  form.value.association_directory = path!;
}

const isEdit = ref<boolean>(false);

const editItem = ref<CategoryItem>();
async function handleConfirm() {
  try {
    if (isEdit.value) {
      const item = {
        ...editItem.value,
        ...form.value,
      };
      await updateCategory(item);
    } else {
      await addCategory(form.value);
    }
    EventBus.emit(AppEvent.UPDATE_CATEGORY_LIST);
    handleClose();
  } catch (e) {
    message.error(e as string);
  }
}

// 打开对话框
EventBus.listen<typeof editItem.value>(
  AppEvent.OPEN_OPERATION_CATEGORY,
  val => {
    isEdit.value = !!val;
    editItem.value = val;

    if (val) {
      for (const key in form.value) {
        // @ts-ignore
        form.value[key] = val[key];
      }
    }

    modalStatus.value = true;
  }
);
</script>

<style scoped>
.n-modal {
  padding: 10px;
}

.n-card {
  width: 600px;
  height: 250px;
}

.n-col {
  margin-top: 10px;
}

::v-deep(.n-card-header),
::v-deep(.n-card__content),
::v-deep(.n-card__footer) {
  padding: 0;
}

::v-deep(.n-card__content) {
  max-height: 200px;
}

::v-deep(.n-input) {
  transition: none !important;
}

.n-input * {
  transition: none !important;
}

::v-deep(.n-input-wrapper) {
  resize: none !important;
}
</style>
