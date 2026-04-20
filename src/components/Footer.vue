<template>
  <n-layout-footer
    bordered
    position="absolute"
    class="h-6 px-1 border-t flex gap-4 items-center justify-between text-xs text-gray-400 z-10"
  >
    <!-- 左侧分类信息 -->
    <div class="flex-sb-c flex-10">
      <!-- <div class="flex"></div> -->
      <!-- <span>当前分类：</span> -->
      <span
        title="当前分类"
        class="flex-s-c w-[90px] gap-1 overflow-hidden cursor-pointer"
      >
        <n-icon
          size="14"
          class="iconfont icon-fenlei"
        />
        <span>
          {{ activeCategory === -1 ? '默认' : activeCategoryItem.name }}
        </span>
      </span>

      <!-- <span>🚀：</span> -->
      <span
        title="启动项数量"
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
          v-if="activeCategory !== -1 && activeCategoryItem.association_directory"
          size="16"
          title="已关联目录"
          class="iconfont icon-guanlian"
        />
        <n-icon
          v-else
          size="14"
          title="未关联目录"
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
            <span>已选中:</span>
            <b
              class="inline-block max-w-[115px] overflow-hidden ml-1"
              :title="activeLaunchItem.name"
            >
              {{ activeLaunchItem.name }}
            </b>
          </span>

          <span class="overflow-hidden whitespace-nowrap min-w-0 flex-10">
            <span>类型:</span>
            <b class="ml-1">{{ formatLaunchType(activeLaunchItem.type) }}</b>
          </span>

          <span class="flex-s-c gap-1 overflow-hidden whitespace-nowrap min-w-0 flex-8">
            <n-icon
              size="14"
              title="启动次数"
              class="iconfont icon-qidongcishu cursor-pointer"
            />

            <b>{{ activeLaunchItem.launch_count }}</b>
          </span>

          <span class="flex-s-c gap-1 overflow-hidden whitespace-nowrap min-w-0 flex-10">
            <n-icon
              size="14"
              title="最近一次启动日期"
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
        :title="activeCategoryItem?.layout === 'grid' ? '平铺' : '列表'"
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
        :title="activeCategoryItem?.sort_order === 'asc' ? '升序' : '降序'"
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
          <h4>内置快捷键</h4>
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
              </template>

              <template v-else>
                <div class="!w-[150px] flex-s-c">
                  <Kbd size="10">{{ item.keys[0] }}</Kbd>
                </div>

                <span>{{ item.name }}</span>
              </template>
            </div>
          </template>
        </div>
      </n-popover>

      <!-- 版本号 -->
      <div class="flex-s-c gap-1">
        <span>v{{ version }}</span>
      </div>
    </div>
  </n-layout-footer>
</template>

<script setup lang="ts">
import { getVersion } from '@tauri-apps/api/app';
import { storeToRefs } from 'pinia';
import { getLaunchByID } from '@/api';
import { formatLaunchType } from '@/common/formatLaunchType';
import { useCategorySort } from '@/composables/useCategorySort';
import { AppEvent } from '@/constant';
import { useStore } from '@/store/useStore';
import { getFromNow } from '@/utils/date';
import { EventBus } from '@/utils/eventBus';
import Kbd from './Kbd.vue';

const store = useStore();

const { activeCategory, activeCategoryItem, launchData, activeLaunchItem } = storeToRefs(store);
const { handleLayoutOrderSortChange } = useCategorySort(activeCategoryItem);

const shortcutKeys = [
  { combKey: false, keys: ['F2'], name: '重命名' },
  { combKey: false, keys: ['F4'], name: '编 辑' },
  { combKey: false, keys: ['F5'], name: '刷 新' },
  { combKey: false, keys: ['Esc'], name: '关闭窗口' },
  { combKey: false, keys: ['Delete'], name: '删 除' },
  // { combKey: true, keys: ['Alt', 'N'], name: '新 建' },
  // { combKey: true, keys: ['Alt', 'C'], name: '新建分类' },
  { combKey: true, keys: ['Ctrl', 'P'], name: '快速定位' },
  { combKey: true, keys: ['Ctrl', 'Shift', 'N'], name: '新 建' },
  { combKey: true, keys: ['Ctrl', 'Shift', 'C'], name: '新建分类' },
  // { combKey: true, keys: ['Ctrl', 'Alt', 'S'], name: '打开设置' },
  // { combKey: true, keys: ['⌘', 'Alt', 'S'], name: '打开设置' },
];

const version = ref<string>();

// const oldLaunchCount = ref(0);
// const newLaunchCount = ref(0);
// watch(
//   () => launchData.value,
//   (newVal, oldVal) => {
//     console.log(`%c newVal ----`, 'color: #fff;background-color: #000;font-size: 18px', newVal.length);
//     console.log(`%c oldVal ----`, 'color: #fff;background-color: #000;font-size: 18px', oldVal.length);
//   },
// );

const sortInfo = computed(() => {
  // if (activeCategory.value === -1) {
  //   return {
  //     title: '名称',
  //     icon: 'icon-mingchengpaixu',
  //   };
  // }
  switch (activeCategoryItem.value?.sort_by) {
    case 'name':
      return {
        title: '名称',
        icon: 'icon-mingchengpaixu',
      };
    case 'type':
      return {
        title: '类型',
        icon: 'icon-anleixingpaixu',
      };
    case 'time':
      return {
        title: '日期',
        icon: 'icon-anchuangjianshijianpaixu',
      };
    case 'order':
      return {
        title: '搜索优先级',
        icon: 'icon-youxianji',
      };

    default:
      return {
        title: '名称',
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

onMounted(async () => {
  version.value = await getVersion();
});
</script>
