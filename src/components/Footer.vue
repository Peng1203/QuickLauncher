<template>
  <n-layout-footer
    bordered
    position="absolute"
    class="h-6 px-1 border-t border-border flex gap-4 items-center justify-between text-xs text-muted-foreground z-10"
  >
    <!-- 左侧分类信息 -->
    <div class="flex-sb-c flex-10">
      <!-- <div class="flex"></div> -->
      <!-- <span>当前分类：</span> -->
      <span
        :title="t('main.currentCategory')"
        class="flex-s-c w-[90px] gap-1 overflow-hidden cursor-pointer"
      >
        <n-icon
          size="14"
          class="iconfont icon-fenlei"
        />
        <span class="overflow-hidden whitespace-nowrap">
          {{ activeCategoryItem?.name }}
        </span>
      </span>

      <!-- <span>🚀：</span> -->
      <span
        :title="t('main.itemCount')"
        class="flex-s-c w-[60px] gap-1 overflow-hidden cursor-pointer"
      >
        <span>🚀</span>
        <span>{{ launchData.length }}</span>
        <!-- <n-number-animation
          ref="numberAnimationInstRef"
          :from="0"
          :to="launchData.length"
        /> -->
      </span>

      <!-- 目录关联状态信息 -->
      <span class="flex-s-c cursor-pointer">
        <n-icon
          v-if="activeCategoryItem?.association_directory"
          size="16"
          :title="t('main.associatedDir')"
          class="iconfont icon-guanlian"
        />
        <n-icon
          v-else
          size="14"
          :title="t('main.unassociatedDir')"
          class="iconfont icon-mti-weiguanlian"
        />
      </span>
    </div>

    <!-- 选中的启动项信息 -->
    <div class="flex-20 !max-w-[535px]">
      <div
        v-if="Object.keys(activeLaunchItem || {}).length"
        class="flex-sb-c overflow-hidden whitespace-nowrap"
      >
        <div
          v-if="activeLaunchItem"
          class="flex w-full min-w-0"
        >
          <span class="flex overflow-hidden whitespace-nowrap min-w-0 flex-20">
            <span>{{ t('footerExtra.selected') }}</span>
            <b
              class="inline-block max-w-[115px] overflow-hidden ml-1"
              :title="activeLaunchItem.name"
            >
              {{ activeLaunchItem.name }}
            </b>
          </span>

          <span class="overflow-hidden whitespace-nowrap min-w-0 flex-10">
            <span>{{ t('footerExtra.typeLabel') }}</span>
            <b class="ml-1">{{ formatLaunchType(activeLaunchItem.type) }}</b>
          </span>

          <span class="flex-s-c gap-1 overflow-hidden whitespace-nowrap min-w-0 flex-8">
            <n-icon
              size="14"
              :title="t('footerExtra.launchCount')"
              class="iconfont icon-qidongcishu cursor-pointer"
            />

            <b>{{ activeLaunchItem.launch_count }}</b>
          </span>

          <span class="flex-s-c gap-1 overflow-hidden whitespace-nowrap min-w-0 flex-10">
            <n-icon
              size="14"
              :title="t('footerExtra.lastLaunch')"
              class="iconfont icon-zuijinshiyong cursor-pointer"
            />

            <b>{{ activeLaunchItem.last_used_at ? getFromNow(activeLaunchItem.last_used_at) : '--' }}</b>
          </span>
        </div>
      </div>
    </div>

    <!-- <div class="flex-23">
      <div
        v-if="Object.keys(activeLaunchItem || {}).length"
        class="flex-sb-c flex-nowrap"
      >
        <template v-if="activeLaunchItem">
          <span>
            已选中:
            <b>{{ activeLaunchItem.name }}</b>
          </span>

          <span>
            类型:
            <b>{{ activeLaunchItem.type }}</b>
          </span>

          <span>
            启动次数:
            <b>{{ activeLaunchItem.launch_count }}</b>
          </span>

          <span>
            上次启动:
            <b>{{ activeLaunchItem.last_used_at ? dayjs(activeLaunchItem.last_used_at) : '--' }}</b>
          </span>
        </template>
      </div>
    </div> -->

    <!-- 其他 -->
    <div class="flex-sb-c flex-13">
      <!-- 布局 -->
      <n-icon
        size="16"
        :title="activeCategoryItem?.layout === 'grid' ? t('common.tile') : t('common.list')"
        :class="`iconfont ${activeCategoryItem?.layout === 'grid' ? 'icon-24gl-appsSmall' : 'icon-liebiao'} cursor-pointer`"
        @click="handleLayoutOrderSortChange(activeCategoryItem?.layout === 'grid' ? 'list' : 'grid', 'layout')"
      />

      <!-- 排序 -->
      <n-icon
        size="16"
        :title="sortInfo.title"
        :class="`iconfont ${sortInfo.icon} cursor-pointer`"
        @click="handleToggleSortBy"
      />

      <!-- 排序方式 -->
      <n-icon
        size="16"
        :title="activeCategoryItem?.sort_order === 'asc' ? t('common.ascending') : t('common.descending')"
        :class="`iconfont ${activeCategoryItem?.sort_order === 'asc' ? 'icon-shengxu' : 'icon-jiangxu'} cursor-pointer`"
        @click="
          handleLayoutOrderSortChange(activeCategoryItem?.sort_order === 'asc' ? 'desc' : 'asc', 'sort_order', true)
        "
      />

      <!-- 内置快捷键 -->
      <n-popover trigger="hover">
        <template #trigger>
          <n-icon
            size="16"
            class="iconfont icon-kuaijiejian- cursor-pointer ml-2"
          />
        </template>

        <div>
          <h4>{{ t('main.shortcutKeys') }}</h4>
          <template
            v-for="item in shortcutKeys"
            :key="item.name"
          >
            <div class="flex-s-c">
              <template v-if="item.combKey">
                <div class="!w-[150px] flex-s-c">
                  <template
                    v-for="(key, j) in item.keys"
                    :key="key"
                  >
                    <Kbd size="10">{{ key }}</Kbd>

                    <span
                      v-if="!(item.keys.length - 1 === j)"
                      class="mx-1"
                    >
                      +
                    </span>
                  </template>
                </div>

                <span>{{ item.name }}</span>

                <span
                  v-if="item?.global"
                  class="ml-2"
                >
                  {{ t('common.global') }}
                </span>
              </template>

              <template v-else>
                <div class="!w-[150px] flex-s-c">
                  <Kbd size="10">{{ item.keys[0] }}</Kbd>
                </div>

                <span>{{ item.name }}</span>

                <span
                  v-if="item?.global"
                  class="ml-2"
                >
                  {{ t('common.global') }}
                </span>
              </template>
            </div>
          </template>
        </div>
      </n-popover>

      <!-- 版本号 -->
      <div class="flex-s-c gap-1">
        <span
          class="cursor-pointer"
          @click="handleToSettingAbout"
        >
          v{{ version }}
        </span>
      </div>
    </div>
  </n-layout-footer>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { getLaunchByID } from '@/api';
import { formatLaunchType } from '@/common/formatLaunchType';
import { useAppConfig, useAppVersion, useCategorySort, useMainWindowShortcut } from '@/composables';
import { AppEvent } from '@/constant';
import { t } from '@/i18n';
import { useStore } from '@/store/useStore';
import { getFromNow } from '@/utils/date';
import { EventBus } from '@/utils/eventBus';

const store = useStore();
const { searchGlobalShortcutKey, mainWindowGlobalShortcutKey } = useAppConfig();
const { activeCategoryItem, launchData, activeLaunchItem } = storeToRefs(store);
const { handleLayoutOrderSortChange } = useCategorySort(activeCategoryItem);

const shortcutKeys = computed(() => {
  // main窗口内置快捷键
  const base = [
    { combKey: false, keys: ['F2'], name: t('main.rename') },
    { combKey: false, keys: ['F4'], name: t('main.edit') },
    // { combKey: false, keys: ['F5'], name: '刷 新' },
    { combKey: false, keys: ['Esc'], name: t('main.close') },
    { combKey: false, keys: ['Delete'], name: t('main.delete') },
    // { combKey: true, keys: ['Ctrl', 'P'], name: '快速定位' },
    { combKey: true, keys: ['Ctrl', 'Shift', 'N'], name: t('main.create') },
    { combKey: true, keys: ['Ctrl', 'Shift', 'C'], name: t('main.createCategory') },
    { combKey: true, keys: ['Alt', 'S'], name: t('main.openSettings') },
    // { combKey: true, keys: ['⌘', 'Alt', 'S'], name: '打开设置' },
  ];

  // 全局快捷键
  const globalShortcutKeys: any = [];
  if (searchGlobalShortcutKey.value) {
    const keys = searchGlobalShortcutKey.value.split('+');
    const item = { combKey: !!keys.length, keys, name: t('main.toggleSearch'), global: true };
    globalShortcutKeys.push(item);
  }
  if (mainWindowGlobalShortcutKey.value) {
    const keys = mainWindowGlobalShortcutKey.value.split('+');
    const item = { combKey: !!keys.length, keys, name: t('main.toggleMain'), global: true };
    globalShortcutKeys.push(item);
  }

  return [...base, ...globalShortcutKeys];
});

const { version, fetchVersion } = useAppVersion();

const sortInfo = computed(() => {
  switch (activeCategoryItem.value?.sort_by) {
    case 'name':
      return {
        title: t('common.name'),
        icon: 'icon-mingchengpaixu',
      };
    case 'type':
      return {
        title: t('common.type'),
        icon: 'icon-anleixingpaixu',
      };
    case 'time':
      return {
        title: t('common.date'),
        icon: 'icon-anchuangjianshijianpaixu',
      };
    case 'order':
      return {
        title: t('common.searchPriority'),
        icon: 'icon-youxianji',
      };

    default:
      return {
        title: t('common.name'),
        icon: 'icon-mingchengpaixu',
      };
  }
});

function handleToggleSortBy() {
  const { sort_by } = activeCategoryItem.value;
  const sort_by_arr: SortByType[] = ['name', 'type', 'time', 'order'];

  const currentIndex = sort_by_arr.indexOf(sort_by);
  const nextIndex = (currentIndex + 1) % sort_by_arr.length;

  const nextSortBy = sort_by_arr[nextIndex];
  handleLayoutOrderSortChange(nextSortBy, 'sort_by', true);
}

EventBus.listen(AppEvent.UPDATE_LAUNCH_ITEM_COUNT, (id: number) => {
  const findRes = launchData.value.find(item => item.id === id);
  if (!findRes) return;
  // findRes.launch_count += 1;
  // 当选中的启动项处于选中的分类时 更新启动项列表
  // EventBus.emit(AppEvent.UPDATE_LAUNCH_LIST);
  nextTick(async () => {
    const upItem = await getLaunchByID(id);

    findRes.launch_count = upItem.launch_count;
    findRes.last_used_at = upItem.last_used_at;
  });
});

function handleToSettingAbout() {
  EventBus.emit(AppEvent.OPEN_SETTING, 'about');
}

useMainWindowShortcut();

onMounted(() => fetchVersion());
</script>
