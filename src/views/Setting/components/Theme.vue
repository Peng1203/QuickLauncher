<template>
  <div class="flex flex-col gap-4 p-4">
    <!-- 外观模式 -->
    <SettingGroup
      title="外观模式"
      description="切换应用的主题明暗"
    >
      <div class="flex gap-2">
        <div
          v-for="mode in appearanceModes"
          :key="mode.value"
          class="theme-card flex-1 cursor-pointer rounded-lg border-2 p-3 text-center transition-all"
          :class="[
            appConfigStore.themeModel === mode.value
              ? 'border-primary bg-primary/50'
              : 'border-transparent bg-secondary/30',
          ]"
          @click="handleSwitchTheme($event, mode.value)"
        >
          <Icon :name="mode.icon" />
          <p class="mt-1.5 text-xs font-medium">{{ mode.label }}</p>
        </div>
      </div>
    </SettingGroup>

    <!-- 主题色 -->
    <SettingGroup
      title="主题色"
      description="选择应用的强调色"
    >
      <div class="flex flex-wrap gap-2">
        <button
          v-for="color in presetColors"
          :key="color"
          class="color-swatch h-7 w-7 rounded-full border-2 transition-transform"
          :class="[
            appConfigStore.themeColor === color
              ? 'scale-110 border-white shadow-md ring-2 ring-primary'
              : 'border-transparent hover:scale-105',
          ]"
          :style="{ backgroundColor: color }"
          @click="appConfigStore.themeColor = color"
        ></button>
        <n-color-picker
          :value="appConfigStore.themeColor"
          :show-alpha="false"
          :modes="['hex']"
          size="small"
          @update:value="appConfigStore.themeColor = $event"
        >
          <button
            class="color-swatch h-7 w-7 rounded-full border-2 border-dashed border-border bg-linear-to-br from-red-400 via-green-400 to-blue-400 hover:scale-105 transition-transform"
          ></button>
        </n-color-picker>
      </div>
    </SettingGroup>

    <!-- 页面风格 & 布局 -->
    <SettingGroup title="页面风格">
      <SettingSelectItem
        v-model="appConfigStore.pageStyle"
        icon="icon-jiemianfengge"
        title="界面风格"
        description="调整整体视觉风格"
        :options="pageStyleOptions"
      />

      <!-- <SettingSelectItem
        v-model="appConfigStore.layoutDensity"
        icon="icon-buju"
        title="布局紧凑度"
        description="调整元素之间的间距"
        :options="densityOptions"
      /> -->

      <SettingSelectItem
        v-model="appConfigStore.font"
        icon="icon-ziti"
        title="字体切换"
        description="调整界面字体(使用系统字体)"
        :options="fontOptions"
      />

      <SettingItem
        icon="icon-zitidaxiao"
        title="字体大小"
        description="调整界面字体大小"
      >
        <div class="flex items-center gap-2 min-w-40">
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
    <!-- <SettingGroup title="窗口效果">
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
    </SettingGroup> -->

    <!-- 动画效果 -->
    <!-- <SettingGroup title="动画效果">
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
    </SettingGroup> -->

    <!-- 预览区 -->
    <!-- <SettingGroup
      title="预览"
      description="当前配置的视觉效果预览"
    >
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
          ></div>
          <div class="flex-1 space-y-1.5">
            <div
              class="h-2.5 w-3/5 rounded"
              :style="{ backgroundColor: appConfigStore.themeColor, opacity: 0.8 }"
            ></div>
            <div class="h-2 w-2/5 rounded bg-muted"></div>
          </div>
        </div>
        <div class="space-y-1.5">
          <div class="h-2 w-full rounded bg-muted"></div>
          <div class="h-2 w-4/5 rounded bg-muted"></div>
          <div class="h-2 w-3/5 rounded bg-muted"></div>
        </div>
      </div>
    </SettingGroup> -->
  </div>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { useAppConfig, useTheme } from '@/composables';
import { useStore } from '@/store/useStore';

const { globalSetThemeFlag } = storeToRefs(useStore());

const { setHTMLThemeClass } = useTheme();
const { appConfigStore } = useAppConfig();

interface AppearanceMode {
  label: string;
  value: ThemeModel;
  icon: string;
}

const appearanceModes: AppearanceMode[] = [
  { label: '浅色', value: 'light', icon: 'icon-taiyang' },
  { label: '深色', value: 'dark', icon: 'icon-yueliang' },
  { label: '跟随系统', value: 'system', icon: 'icon-gensuixitong1' },
];

const presetColors = [
  '#2080f0',
  '#18a058',
  '#f0a020',
  '#d03050',
  '#7b1fa2',
  '#00838f',
  '#e65100',
  '#5c6bc0',
  '#2e7d32',
  '#c62828',
  '#283593',
  '#00695c',
];

const pageStyleOptions: OptionItem<string>[] = [
  { label: '默认', value: 'normal' },
  { label: 'macOS', value: 'macos' },
  { label: 'Windows 11', value: 'win11' },
];

// const densityOptions: OptionItem<string>[] = [
//   { label: '紧凑', value: 'compact' },
//   { label: '默认', value: 'default' },
//   { label: '舒适', value: 'comfortable' },
// ];

const fontOptions = ref([]);

function handleSwitchTheme(e: PointerEvent, newMode: ThemeModel) {
  if (appConfigStore.themeModel === newMode) return;

  if (newMode === 'system') return (appConfigStore.themeModel = newMode);

  globalSetThemeFlag.value = false;
  appConfigStore.themeModel = newMode;
  nextTick(() => {
    setHTMLThemeClass(newMode, e);
    globalSetThemeFlag.value = true;
  });
}
// const speedOptions: OptionItem<string>[] = [
//   { label: '较快', value: 'fast' },
//   { label: '正常', value: 'normal' },
//   { label: '较慢', value: 'slow' },
// ];

async function getSystemFonts() {
  // @ts-ignore
  if (window.queryLocalFonts) {
    // @ts-ignore
    const fonts = await window.queryLocalFonts();
    const options: any = [{ label: '默认', value: '' }];
    fonts.forEach((fontItem: any) => {
      const { family, fullName, style } = fontItem;
      // 只添加每种字体的 Regular/Normal 风格
      // @ts-ignore
      const findRes = options.find(item => item.value === family);
      if (findRes) return;

      if (style === 'Regular') {
        options.push({
          label: fullName,
          value: family,
          style: `font-family: "${family}"`,
        });
      }
    });

    fontOptions.value = options;
  }
}

onMounted(() => {
  // 加载系统字体
  getSystemFonts();
});
</script>

<style scoped lang="scss">
.theme-card {
  border-color: transparent;
  &:hover {
    border-color: var(--primary);
    // background: rgba(0, 0, 0, 0.02);
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
