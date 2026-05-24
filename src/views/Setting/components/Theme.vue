<template>
  <div class="flex flex-col gap-4 p-4">
    <!-- 外观模式 -->
    <SettingGroup title="外观模式" description="切换应用的主题明暗">
      <div class="flex gap-2">
        <div
          v-for="mode in appearanceModes"
          :key="mode.value"
          :class="[
            'theme-card flex-1 cursor-pointer rounded-lg border-2 p-3 text-center transition-all',
            appConfigStore.appearanceMode === mode.value
              ? 'border-[var(--primary)] bg-primary/5'
              : 'border-transparent bg-secondary/30 hover:bg-secondary/50',
          ]"
          @click="appConfigStore.appearanceMode = mode.value as any"
        >
          <n-icon :size="24" :color="appConfigStore.appearanceMode === mode.value ? 'var(--primary)' : undefined">
            <component :is="mode.icon" />
          </n-icon>
          <p class="mt-1.5 text-xs font-medium">{{ mode.label }}</p>
        </div>
      </div>
    </SettingGroup>

    <!-- 主题色 -->
    <SettingGroup title="主题色" description="选择应用的强调色">
      <div class="flex flex-wrap gap-2">
        <button
          v-for="color in presetColors"
          :key="color"
          :class="[
            'color-swatch h-7 w-7 rounded-full border-2 transition-transform',
            appConfigStore.themeColor === color
              ? 'scale-110 border-white shadow-md ring-2 ring-[var(--primary)]'
              : 'border-transparent hover:scale-105',
          ]"
          :style="{ backgroundColor: color }"
          @click="appConfigStore.themeColor = color"
        />
        <n-color-picker
          :value="appConfigStore.themeColor"
          :show-alpha="false"
          :modes="['hex']"
          size="small"
          @update:value="appConfigStore.themeColor = $event"
        >
          <button
            class="color-swatch h-7 w-7 rounded-full border-2 border-dashed border-border bg-gradient-to-br from-red-400 via-green-400 to-blue-400 hover:scale-105 transition-transform"
          />
        </n-color-picker>
      </div>
    </SettingGroup>

    <!-- 页面风格 & 布局 -->
    <SettingGroup title="页面风格">
      <SettingItem
        icon="icon-yangshi"
        title="界面风格"
        description="调整整体视觉风格"
      >
        <n-select
          v-model:value="appConfigStore.pageStyle"
          size="small"
          :consistent-menu-width="false"
          :options="pageStyleOptions as any"
        />
      </SettingItem>

      <SettingItem
        icon="icon-buju"
        title="布局紧凑度"
        description="调整元素之间的间距"
      >
        <n-select
          v-model:value="appConfigStore.layoutDensity"
          size="small"
          :consistent-menu-width="false"
          :options="densityOptions as any"
        />
      </SettingItem>

      <SettingItem
        icon="icon-ziti"
        title="字体大小"
        description="调整界面字体大小"
      >
        <div class="flex items-center gap-2 min-w-[160px]">
          <span class="text-[10px] text-muted-foreground">A</span>
          <n-slider
            v-model:value="appConfigStore.fontSize"
            :min="12"
            :max="18"
            :step="1"
            :format-tooltip="(v: number) => `${v}px`"
            style="flex: 1"
          />
          <span class="text-sm font-medium text-muted-foreground">A</span>
        </div>
      </SettingItem>
    </SettingGroup>

    <!-- 窗口效果 -->
    <SettingGroup title="窗口效果">
      <SettingItem
        icon="icon-yuanjiao"
        title="窗口圆角"
        description="调整窗口边角的圆角尺寸"
      >
        <div class="flex items-center gap-2 min-w-[140px]">
          <n-slider
            v-model:value="appConfigStore.borderRadius"
            :min="0"
            :max="24"
            :step="2"
            :format-tooltip="(v: number) => `${v}px`"
            style="flex: 1"
          />
        </div>
      </SettingItem>

      <SettingItem
        icon="icon-mohu"
        title="背景模糊"
        description="窗口背景的模糊效果强度"
      >
        <div class="flex items-center gap-2 min-w-[140px]">
          <n-slider
            v-model:value="appConfigStore.backgroundBlur"
            :min="0"
            :max="100"
            :step="5"
            :format-tooltip="(v: number) => `${v}%`"
            style="flex: 1"
          />
        </div>
      </SettingItem>

      <SettingItem
        icon="icon-toumingdu"
        title="窗口透明度"
        description="调整窗口整体透明度"
      >
        <div class="flex items-center gap-2 min-w-[140px]">
          <n-slider
            v-model:value="appConfigStore.windowOpacity"
            :min="60"
            :max="100"
            :step="5"
            :format-tooltip="(v: number) => `${v}%`"
            style="flex: 1"
          />
        </div>
      </SettingItem>
    </SettingGroup>

    <!-- 动画效果 -->
    <SettingGroup title="动画效果">
      <SettingSwitchItem
        v-model="appConfigStore.enableAnimation"
        icon="icon-donghua"
        title="启用动画"
        description="开启界面过渡动画效果"
      />

      <SettingItem
        v-if="appConfigStore.enableAnimation"
        icon="icon-sudu"
        title="动画速度"
        description="界面过渡动画的播放速度"
      >
        <n-select
          v-model:value="appConfigStore.animationSpeed"
          size="small"
          :consistent-menu-width="false"
          :options="speedOptions as any"
        />
      </SettingItem>
    </SettingGroup>

    <!-- 预览区 -->
    <SettingGroup title="预览" description="当前配置的视觉效果预览">
      <div
        class="preview-box rounded-xl border bg-card/50 p-4"
        :style="{
          borderRadius: `${appConfigStore.borderRadius}px`,
          opacity: appConfigStore.windowOpacity / 100,
        }"
      >
        <div class="flex items-center gap-3 mb-3">
          <div
            class="h-8 w-8 rounded-full"
            :style="{ backgroundColor: appConfigStore.themeColor }"
          />
          <div class="flex-1 space-y-1.5">
            <div class="h-2.5 w-3/5 rounded" :style="{ backgroundColor: appConfigStore.themeColor, opacity: 0.8 }" />
            <div class="h-2 w-2/5 rounded bg-muted" />
          </div>
        </div>
        <div class="space-y-1.5">
          <div class="h-2 w-full rounded bg-muted" />
          <div class="h-2 w-4/5 rounded bg-muted" />
          <div class="h-2 w-3/5 rounded bg-muted" />
        </div>
      </div>
    </SettingGroup>
  </div>
</template>

<script setup lang="ts">
import { SunnyOutline, MoonOutline, DesktopOutline } from '@vicons/ionicons5';
import { useAppConfig } from '@/composables';

const { appConfigStore } = useAppConfig();

const appearanceModes = [
  { label: '浅色', value: 'light', icon: SunnyOutline },
  { label: '深色', value: 'dark', icon: MoonOutline },
  { label: '跟随系统', value: 'system', icon: DesktopOutline },
];

const presetColors = [
  '#2080f0', '#18a058', '#f0a020', '#d03050',
  '#7b1fa2', '#00838f', '#e65100', '#5c6bc0',
  '#2e7d32', '#c62828', '#283593', '#00695c',
];

const pageStyleOptions: OptionItem<string>[] = [
  { label: '正常', value: 'normal' },
  { label: 'macOS 风格', value: 'macos' },
  { label: 'Windows 11 风格', value: 'win11' },
];

const densityOptions: OptionItem<string>[] = [
  { label: '紧凑', value: 'compact' },
  { label: '默认', value: 'default' },
  { label: '舒适', value: 'comfortable' },
];

const speedOptions: OptionItem<string>[] = [
  { label: '较快', value: 'fast' },
  { label: '正常', value: 'normal' },
  { label: '较慢', value: 'slow' },
];
</script>

<style scoped lang="scss">
.theme-card {
  border-color: transparent;
  &:hover {
    border-color: var(--primary);
    background: rgba(0, 0, 0, 0.02);
  }
}

.color-swatch {
  cursor: pointer;
  flex-shrink: 0;
}

.preview-box {
  backdrop-filter: blur(8px);
  transition: all 0.3s ease;
}
</style>
