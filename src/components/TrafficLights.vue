<template>
  <div v-if="appConfigStore.pageStyle === 'macos'" class="traffic-lights">
    <div class="traffic-light red" @mouseenter="hovered = 'red'" @mouseleave="hovered = ''">
      <span v-if="hovered === 'red'" class="traffic-light-icon">×</span>
    </div>
    <div class="traffic-light yellow" @mouseenter="hovered = 'yellow'" @mouseleave="hovered = ''">
      <span v-if="hovered === 'yellow'" class="traffic-light-icon">−</span>
    </div>
    <div class="traffic-light green" @mouseenter="hovered = 'green'" @mouseleave="hovered = ''">
      <span v-if="hovered === 'green'" class="traffic-light-icon">+</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useAppConfig } from "@/composables";

const { appConfigStore } = useAppConfig();
const hovered = ref("");
</script>

<style scoped lang="scss">
.traffic-lights {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}

.traffic-light {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: opacity 0.15s;

  &.red {
    background-color: #ff5f57;
  }
  &.yellow {
    background-color: #febc2e;
  }
  &.green {
    background-color: #28c840;
  }

  &:hover {
    opacity: 0.85;
  }
}

.traffic-light-icon {
  font-size: 8px;
  line-height: 1;
  color: rgba(0, 0, 0, 0.5);
  font-weight: bold;
}
</style>
