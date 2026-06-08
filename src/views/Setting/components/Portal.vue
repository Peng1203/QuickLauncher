<template>
  <div class="flex flex-col gap-4 p-4">
    <SettingGroup :title="t('portal.groupEnable')">
      <SettingSwitchItem
        v-model="appConfigStore.portalEnabled"
        icon="icon-chuansongmen"
        :title="t('portal.enableTitle')"
        :description="t('portal.enableDesc')"
      />
    </SettingGroup>

    <SettingGroup :title="t('portal.groupNotify')" :description="t('portal.notifyDesc')">
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
          <Icon :name="item.icon" size="14" />

          <span class="text-[12px] font-semibold">
            {{ item.label }}
          </span>
        </div>
      </div>
    </SettingGroup>

    <SettingGroup :title="t('portal.groupDuration')" :description="t('portal.durationDesc')">
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

    <SettingGroup :title="t('portal.groupDisplay')">
      <SettingSwitchItem
        v-model="appConfigStore.portalShowPath"
        icon="icon-icon-flowpath"
        :title="t('portal.showPathTitle')"
        :description="t('portal.showPathDesc')"
      />
      <SettingSwitchItem
        v-model="appConfigStore.portalShowProgress"
        icon="icon-daojishi"
        :title="t('portal.showProgressTitle')"
        :description="t('portal.showProgressDesc')"
      />
      <!-- v-model:expanded="visible" -->
      <SettingSwitchItem
        v-model="appConfigStore.portalEnableShortcut"
        expandable
        icon="icon-kuaijiejian-"
        :title="t('portal.enableShortcutTitle')"
        :description="t('portal.enableShortcutDesc')"
      >
        <template #expand>
          <div>
            <!-- icon="icon-kuaijiejian-" -->
            <SettingItem :title="t('portal.openTitle')">
              <ShortcutKbd class="w-30" :value="appConfigStore.portalOpenShortcutKey" />
              <!-- <ShortcutKeyInput
                  v-model="test"
                  @commit="handleSave"
                /> -->
            </SettingItem>
            <SettingItem :title="t('portal.openInExplorer')">
              <ShortcutKbd class="w-30" :value="appConfigStore.portalOpenDirInManagerShortcutKey" />
            </SettingItem>
            <SettingItem :title="t('portal.openInTerminal')">
              <ShortcutKbd
                class="w-30"
                :value="appConfigStore.portalOpenDirInTerminalShortcutKey"
              />
            </SettingItem>
          </div>
        </template>
      </SettingSwitchItem>
      <!-- <SettingSwitchItem
        v-model="appConfigStore.portalShowShortcut"
        icon="icon-jujiao"
        title="显示快捷键提示"
        description="在窗口中显示快捷键说明"
      /> -->
    </SettingGroup>

    <SettingGroup :title="t('portal.groupBrowser')">
      <SettingSelectItem
        v-model:value="appConfigStore.portalBrowser"
        icon="icon-liulanqi"
        :title="t('portal.browserTitle')"
        :description="t('portal.browserDesc')"
        :options="appConfigStore.browserOptions"
        :placeholder="t('common.defaultBrowser')"
      />
    </SettingGroup>

    <SettingGroup :title="t('portal.groupPosition')" :description="t('portal.positionDesc')">
      <div class="flex gap-5">
        <n-button size="small" type="success" @click="handleSetLocation">
          <template #icon>
            <Icon name="icon-address" size="14" />
          </template>
          {{ t("portal.selectPosition") }}
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

    <SettingGroup :title="t('portal.groupOpacity')" :description="t('portal.opacityDesc')">
      <div class="flex items-center gap-8">
        <NSlider v-model:value="appConfigStore.portalOpacity" :min="60" :max="100" class="flex-2" />

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
import { useAppConfig } from "@/composables";
import { AppEvent, PortalNotifyMode } from "@/constant";
import { t } from "@/i18n";
import { EventBus } from "@/utils/eventBus";

const { appConfigStore } = useAppConfig();

const notifyModes = computed<
  { label: string; value: PortalNotifyMode; iconClass?: string; icon?: any }[]
>(() => [
  {
    label: t("portal.notifyWindow"),
    value: PortalNotifyMode.WINDOW,
    icon: "icon-program-code",
  },
  {
    label: t("portal.notifyTray"),
    value: PortalNotifyMode.TRAY,
    icon: "icon-shanshuo",
  },
  {
    label: t("portal.notifySilent"),
    value: PortalNotifyMode.SILENT,
    icon: "icon-guanbitongzhi",
  },
]);

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
