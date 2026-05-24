<template>
  <div class="flex flex-col gap-4 p-4">
    <SettingGroup title="启用">
      <SettingSwitchItem
        v-model="appConfigStore.portalEnabled"
        icon="icon-chuansongmen"
        title="启用传送门"
        description="开启后，当检测到复制目录地址、链接等内容时显示提示"
      />
    </SettingGroup>

    <SettingGroup
      title="通知方式"
      description="选择提示出现的通知形式"
    >
      <div class="grid grid-cols-3 gap-4">
        <div
          v-for="item in notifyModes"
          :key="item.value"
          class="flex h-6 cursor-pointer items-center justify-center gap-2 rounded border transition-all"
          :class="
            appConfigStore.portalNotifyMode === item.value
              ? 'border-primary bg-primary/5 text-primary'
              : 'border-border bg-card hover:border-primary/40'
          "
          @click="appConfigStore.portalNotifyMode = item.value"
        >
          <Icon
            :name="item.icon"
            size="14"
          />

          <span class="text-[12px] font-semibold">
            {{ item.label }}
          </span>
        </div>
      </div>
    </SettingGroup>

    <SettingGroup
      title="持续时长"
      description="提示窗口自动关闭前的持续时间"
    >
      <div class="flex items-center gap-8">
        <n-slider
          v-model:value="appConfigStore.portalDuration"
          :min="500"
          :max="10000"
          :step="100"
          class="flex-2"
        />

        <div class="flex items-center gap-2 flex-1">
          <n-input-number
            v-model:value="appConfigStore.portalDuration"
            size="small"
            :min="500"
            :max="10000"
            button-placement="both"
          />

          <span class="text-sm text-muted-foreground whitespace-nowrap">ms</span>
        </div>
      </div>
    </SettingGroup>

    <SettingGroup title="显示选项">
      <SettingSwitchItem
        v-model="appConfigStore.portalShowPath"
        icon="icon-icon-flowpath"
        title="显示路径"
        description="在通知中展示完整路径或链接"
      />
      <SettingSwitchItem
        v-model="appConfigStore.portalShowProgress"
        icon="icon-daojishi"
        title="显示倒计时条"
        description="在窗口底部显示进度条"
      />
      <SettingSwitchItem
        v-model="appConfigStore.portalEnableShortcut"
        icon="icon-kuaijiejian-"
        title="启用快捷键"
        description="允许使用快捷键快速访问传送门"
      />
      <SettingSwitchItem
        v-model="appConfigStore.portalShowShortcut"
        icon="icon-jujiao"
        title="显示快捷键提示"
        description="在窗口中显示快捷键说明"
      />
    </SettingGroup>

    <SettingGroup
      title="通知位置"
      description="选择提示窗口出现的位置，以通知窗口的左上角坐标位置为准"
    >
      <div class="flex gap-5">
        <n-button
          size="small"
          type="success"
          @click="handleSetLocation"
        >
          <template #icon>
            <Icon
              name="icon-address"
              size="14"
            />
          </template>
          选择展示位置
        </n-button>

        <div class="flex items-center gap-2">
          <span class="text-sm text-muted-foreground">X</span>

          <NInputNumber
            v-model:value="appConfigStore.portalWindowPositionX"
            size="small"
            :min="0"
            button-placement="both"
          />
        </div>

        <div class="flex items-center gap-2">
          <span class="text-sm text-muted-foreground">Y</span>

          <NInputNumber
            v-model:value="appConfigStore.portalWindowPositionY"
            size="small"
            :min="0"
            button-placement="both"
          />
        </div>
      </div>
    </SettingGroup>

    <SettingGroup
      title="窗口透明度"
      description="调整窗口背景透明度（含模糊效果）"
    >
      <div class="flex items-center gap-8">
        <NSlider
          v-model:value="appConfigStore.portalOpacity"
          :min="60"
          :max="100"
          class="flex-2"
        />

        <div class="flex items-center gap-2 flex-1">
          <NInputNumber
            v-model:value="appConfigStore.portalOpacity"
            size="small"
            :min="60"
            :max="100"
            button-placement="both"
          />

          <span class="text-sm text-muted-foreground">%</span>
        </div>
      </div>
    </SettingGroup>
  </div>
</template>

<script setup lang="ts">
import { useAppConfig } from '@/composables';
import { AppEvent, PortalNotifyMode } from '@/constant';
import { EventBus } from '@/utils/eventBus';

const { appConfigStore } = useAppConfig();

const notifyModes: { label: string; value: PortalNotifyMode; iconClass?: string; icon?: any }[] = [
  {
    label: '弹窗通知',
    value: PortalNotifyMode.WINDOW,
    icon: 'icon-program-code',
  },
  {
    label: '托盘闪烁',
    value: PortalNotifyMode.TRAY,
    icon: 'icon-shanshuo',
  },
  {
    label: '关闭通知',
    value: PortalNotifyMode.SILENT,
    icon: 'icon-guanbitongzhi',
  },
];

function handleSetLocation() {
  // 通过事件总线通知剪贴板窗口进入设置位置模式
  EventBus.emit(AppEvent.OPEN_CLIPBOARD_WINDOW_BY_SET_LOCATION_MODAL);
}
</script>

<style scoped lang="scss">
:deep(.n-card) {
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.03);
}

:deep(.n-slider .n-slider-rail__fill) {
  background: var(--primary);
}

:deep(.n-slider .n-slider-handle) {
  border-color: var(--primary);
}

:deep(.n-switch.n-switch--active .n-switch__rail) {
  background-color: var(--primary) !important;
}

:deep(.n-card__content) {
  padding: 5px 0px !important;
}
:deep(.n-input__input-el) {
  text-align: center;
}

:deep(.n-carousel__slide) {
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
