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
      <img
        v-if="item.icon"
        :src="item.icon"
        alt="icon"
        class="m-2! object-contain pointer-events-none w-8 h-8"
      />
      <Icon v-else name="icon-fenlei1" size="32" class="m-2!" />

      <span class="!ml-0.5">
        {{ item.name }}
      </span>
    </div>
  </li>
</template>

<script setup lang="ts" generic="T extends CategoryItem">
interface Props {
  item: T;
  active?: boolean;
  disabled?: boolean;
}

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
