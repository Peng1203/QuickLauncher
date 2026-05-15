<template>
  <div class="flex-1 mr-3 overflow-auto">
    <div class="flex flex-col gap-1">
      <NCard
        class="col-span-4 !rounded-[24px]"
        :bordered="false"
      >
        <div class="flex flex-col !mt-[0]">
          <div class="flex-sb-c">
            <h3>启用</h3>
            <NSwitch
              v-model:value="appConfigStore.portalEnabled"
              size="small"
            />
          </div>

          <DescText>开启后，当检测到复制目录、链接等内容时显示提示窗口</DescText>
        </div>
      </NCard>

      <!-- 通知方式 -->
      <NCard
        class="col-span-8 !rounded-[24px]"
        :bordered="false"
      >
        <h3>通知方式</h3>
        <DescText>选择提示出现的通知形式</DescText>

        <div class="mt-2 grid grid-cols-3 gap-4">
          <div
            v-for="item in notifyModes"
            :key="item.value"
            class="flex h-12 cursor-pointer items-center justify-center gap-2 rounded-2xl border transition-all"
            :class="
              appConfigStore.portalNotifyMode === item.value
                ? 'border-[#16a34a] bg-[#f0fdf4] text-[#16a34a]'
                : 'border-black/8 bg-white hover:border-[#16a34a]/40'
            "
            @click="appConfigStore.portalNotifyMode = item.value"
          >
            <NIcon
              v-if="item.iconClass"
              size="18"
              :class="item.iconClass"
            />
            <NIcon
              v-else
              size="18"
            >
              <component :is="item.icon" />
            </NIcon>

            <span class="text-[12px] font-semibold">
              {{ item.label }}
            </span>
          </div>
        </div>
      </NCard>

      <!-- 持续时长 -->
      <NCard
        class="col-span-12 !rounded-[24px]"
        :bordered="false"
      >
        <div class="text-[14px] font-semibold">持续时长</div>
        <DescText>提示窗口自动关闭前的持续时间</DescText>

        <div class="mt-2 flex items-center gap-8">
          <NSlider
            v-model:value="appConfigStore.portalDuration"
            :min="500"
            :max="10000"
            :step="100"
            class="flex-2"
          />

          <div class="flex items-center gap-2 flex-1">
            <NInputNumber
              v-model:value="appConfigStore.portalDuration"
              size="small"
              :min="500"
              :max="10000"
              button-placement="both"
            />

            <span class="text-sm text-[#6b7280] whitespace-nowrap">ms</span>
          </div>
        </div>
      </NCard>

      <!-- 显示路径 -->
      <NCard
        class="col-span-4 !rounded-[24px]"
        :bordered="false"
      >
        <div class="flex flex-col">
          <div class="flex-sb-c">
            <h3>显示路径</h3>
            <NSwitch
              v-model:value="appConfigStore.portalShowPath"
              size="small"
            />
          </div>
          <DescText>在通知中展示完整路径或链接</DescText>
        </div>
      </NCard>

      <!-- 显示倒计时条 -->
      <NCard
        class="col-span-4 !rounded-[24px]"
        :bordered="false"
      >
        <div class="flex flex-col">
          <div class="flex-sb-c">
            <h3>显示倒计时条</h3>
            <NSwitch
              v-model:value="appConfigStore.portalShowProgress"
              size="small"
            />
          </div>

          <DescText>在窗口底部显示进度条</DescText>
        </div>
      </NCard>

      <NCard
        class="col-span-4 !rounded-[24px]"
        :bordered="false"
      >
        <div class="flex flex-col">
          <div class="flex-sb-c">
            <h3>启用快捷键</h3>
            <NSwitch
              v-model:value="appConfigStore.portalEnableShortcut"
              size="small"
            />
          </div>
          <DescText>启用后可通过快捷键快速打开</DescText>
        </div>
      </NCard>

      <!-- 显示快捷键提示 -->
      <NCard
        class="col-span-4 !rounded-[24px]"
        :bordered="false"
      >
        <div class="flex flex-col">
          <div class="flex-sb-c">
            <h3>显示快捷键提示</h3>
            <NSwitch
              v-model:value="appConfigStore.portalShowShortcut"
              size="small"
            />
          </div>
          <DescText>在窗口中显示快捷键说明</DescText>
        </div>
      </NCard>

      <!-- 通知位置 -->
      <NCard
        class="col-span-6 !rounded-[24px]"
        :bordered="false"
      >
        <h3>通知位置</h3>
        <DescText>选择提示窗口出现的位置 以通知窗口的左上角坐标位置为准</DescText>

        <div class="mt-2 flex items-center gap-10">
          <!-- grid -->
          <div class="flex-c-c flex-3">
            <NButton
              color="#1989fa"
              @click="handleSetLocation"
            >
              选择展示位置
            </NButton>
          </div>

          <!-- offset -->
          <div class="flex flex-2 flex-col gap-5">
            <div class="flex items-center gap-3">
              <span class="w-[10px] text-sm text-[#6b7280]">X</span>

              <NInputNumber
                v-model:value="appConfigStore.portalWindowPositionX"
                size="small"
                :min="0"
                button-placement="both"
              />

              <span class="text-sm text-[#6b7280] whitespace-nowrap">px</span>
            </div>

            <div class="flex items-center gap-3">
              <span class="w-[10px] text-sm text-[#6b7280]">Y</span>

              <NInputNumber
                v-model:value="appConfigStore.portalWindowPositionY"
                size="small"
                :min="0"
                button-placement="both"
              />

              <span class="text-sm text-[#6b7280] whitespace-nowrap">px</span>
            </div>
          </div>
        </div>
      </NCard>

      <!-- 动画效果 -->
      <!-- <NCard
        class="col-span-7 !rounded-[24px]"
        :bordered="false"
      >
        <h3>动画效果</h3>
        <DescText>控制提示窗口的出现动画</DescText>

        <NRadioGroup
          v-model:value="form.animation"
          class="mt-2"
        >
          <div class="flex gap-10">
            <NRadio value="scale">缩放</NRadio>

            <NRadio value="fade">淡入</NRadio>

            <NRadio value="slide">滑入</NRadio>

            <NRadio value="none">无动画</NRadio>
          </div>
        </NRadioGroup>
      </NCard>

      <NCard
        class="col-span-5 !rounded-[24px]"
        :bordered="false"
      >
        <h3>动画时长</h3>
        <DescText>控制动画播放速度</DescText>

        <NSelect
          v-model:value="form.animationDuration"
          :options="animationOptions"
          class="mt-2"
          size="small"
        />
      </NCard> -->

      <!-- 窗口透明度 -->
      <NCard
        class="col-span-7 !rounded-[24px]"
        :bordered="false"
      >
        <h3>窗口透明度</h3>
        <DescText>调整窗口背景透明度（含模糊效果）</DescText>

        <div class="mt-2 flex items-center gap-8">
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

            <span class="text-sm text-[#6b7280]">%</span>
          </div>
        </div>
      </NCard>

      <!-- 预览 -->
      <NCard
        class="col-span-12 !rounded-[24px]"
        :bordered="false"
      >
        <h3>预览效果</h3>
        <DescText>以下为当前配置的显示效果预览</DescText>

        <div class="flex justify-center py-5">
          <n-carousel
            autoplay
            trigger="hover"
          >
            <ClipboardToast
              :visible="true"
              type="Url"
              content="https://www.baidu.com"
              model="demo"
              style="width: 320px !important; height: 131px !important; border-radius: 10px"
            />

            <ClipboardToast
              :visible="true"
              type="Directory"
              content="C:\Users"
              model="demo"
              style="width: 320px !important; height: 131px !important; border-radius: 10px"
            />
          </n-carousel>
        </div>
      </NCard>
    </div>
  </div>
</template>

<script setup lang="ts">
// import { ref } from 'vue';
import { CloseOutline, NotificationsOffOutline } from '@vicons/ionicons5';
import { useAppConfig } from '@/composables';
import { AppEvent } from '@/constant';
import { EventBus } from '@/utils/eventBus';
import ClipboardToast from '@/views/ClipboardToast/ClipboardToast.vue';

const { appConfigStore } = useAppConfig();

const notifyModes: { label: string; value: PortalNotifyMode; iconClass?: string; icon?: any }[] = [
  {
    label: '弹窗通知',
    value: 'window',
    iconClass: 'iconfont icon-danchuangzujian',
  },
  {
    label: '托盘闪烁',
    value: 'tray',
    icon: NotificationsOffOutline,
  },
  {
    label: '关闭通知',
    value: 'silent',
    icon: CloseOutline,
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
  background: #16a34a;
}

:deep(.n-slider .n-slider-handle) {
  border-color: #16a34a;
}

:deep(.n-switch.n-switch--active .n-switch__rail) {
  background-color: #16a34a !important;
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
