<template>
  <div class="flex items-center gap-2">
    <!-- 预设按钮 -->
    <div
      v-if="presets?.length"
      class="flex gap-1"
    >
      <n-button
        v-for="item in presets"
        :key="item"
        type="info"
        size="tiny"
        @click="handlePreset(item)"
      >
        {{ item }}
      </n-button>
    </div>

    <!-- 输入框 -->
    <n-input
      v-model:value="innerValue"
      :style="{ width: `calc(${innerValue.length}ch + 36px ) !important`, minWidth: `calc(50px + 36px ) !important` }"
      size="tiny"
      readonly
      clearable
      :status="status"
      :placeholder="t('shortcutKeyInput.placeholder')"
      @keydown="handleKeydown"
      @blur="handleBlur"
      @clear="handleClear"
      @focus="status = 'success'"
    />
  </div>
</template>

<script setup lang="ts">
import type { FormValidationStatus } from 'naive-ui';
import { useNaiveUiApi } from '@/composables';
import { t } from '@/i18n';
import { checkShortcutKey, checkShortcutKeyComplete, getShortcutKey } from '@/utils/shortcutKey';

defineProps<{
  presets?: string[];
}>();

const emit = defineEmits<{
  (e: 'commit', val: string): void;
  (e: 'clear'): void;
}>();

const modelValue = defineModel<string>();

const { message } = useNaiveUiApi();

const innerValue = ref(modelValue.value || '');
const status = ref<FormValidationStatus>('success');

function handleKeydown(e: KeyboardEvent) {
  e.preventDefault();

  const target = e.target as HTMLInputElement | null;
  if (!target) return;

  // 判断是否有文本被选中
  const hasSelection =
    target.selectionStart !== null && target.selectionEnd !== null && target.selectionStart !== target.selectionEnd;

  if (hasSelection) {
    return;
  }

  const keyValue = getShortcutKey(e, '');
  innerValue.value = keyValue;
}

async function handleBlur() {
  if (!innerValue.value) return handleClear();

  if (modelValue.value !== '' && innerValue.value === modelValue.value) return;

  const isComplete = checkShortcutKeyComplete(innerValue.value);
  if (!isComplete) {
    message.error(t('shortcutKeyInput.incomplete'));
    status.value = 'error';
    return;
  }

  const { checked, message: msg } = await checkShortcutKey(innerValue.value);
  if (!checked) {
    msg && message.warning(msg);
    status.value = 'warning';
    return;
  }

  nextTick(() => emit('commit', innerValue.value));
}

function handleClear() {
  innerValue.value = '';
  emit('clear');
}

function handlePreset(val: string) {
  // if (modelValue.value !== '' && modelValue.value === val) return;
  innerValue.value = val;
  status.value = 'success';
  nextTick(() => emit('commit', val));
}

watch(
  () => modelValue.value,
  val => {
    if (!val) return;
    innerValue.value = val;
    status.value = 'success';
  },
);
</script>
