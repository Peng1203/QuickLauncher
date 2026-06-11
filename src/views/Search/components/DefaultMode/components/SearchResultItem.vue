<template>
  <li
    ref="itemRef"
    class="flex items-center justify-between h-[48px] px-4 py-2 transition-colors"
    :class="[
      disabled ? 'cursor-not-allowed opacity-50' : 'cursor-pointer',
      !disabled && (active ? 'bg-muted' : 'hover:bg-muted'),
    ]"
    @click="!disabled && emit('click')"
    @contextmenu.prevent.stop="!disabled && emit('contextmenu', $event)"
  >
    <div class="flex items-center">
      <n-icon
        v-if="item.type === 'alias'"
        size="32"
        class="iconfont icon-minglinghangchaxun !m-2 text-[32px]"
      />

      <img
        v-else
        :src="item.icon || ''"
        alt="icon"
        class="!m-2 object-contain pointer-events-none w-8 h-8"
      />

      <span class="!ml-0.5">
        {{ item.name }}
      </span>
    </div>

    <div v-if="appConfigStore.showCategory" class="flex items-end space-x-1">
      <n-tag v-if="item.type === 'alias'" bordered size="small" type="info">
        {{ $t?.("search.commandAlias") ?? "Alias" }}
      </n-tag>

      <template v-else>
        <n-tag v-if="item.category_name" bordered size="small" type="default">
          {{ item.category_name }}
        </n-tag>

        <n-tag
          v-if="appConfigStore.showSubCategory && item.subcategory_name"
          bordered
          size="tiny"
          type="default"
        >
          {{ item.subcategory_name }}
        </n-tag>
      </template>
    </div>
  </li>
</template>

<script setup lang="ts" generic="T extends SearchLauncItem">
import { useAppConfig } from "@/composables";
import { NIcon, NTag } from "naive-ui";

interface Props {
  item: T;
  active?: boolean;
  disabled?: boolean;
}
const { appConfigStore } = useAppConfig();

defineProps<Props>();

const emit = defineEmits<{
  click: [];
  contextmenu: [event: MouseEvent];
}>();

const itemRef = useTemplateRef<HTMLLIElement>("itemRef");
function scrollToIntoView() {
  itemRef.value?.scrollIntoView({ behavior: "smooth", block: "nearest" });
}

defineExpose({
  scrollToIntoView,
});
</script>
