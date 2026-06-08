import { h } from "vue";
import { useCategorySort } from "@/composables";
import { t } from "@/i18n";
import Icon from "@/components/ui/Icon.vue";
function renderIcon(name: string) {
  return () => h(Icon, { name });
}

function renderLabel(prop: keyof CategoryItem, item: CategoryItem, label: string, value?: string) {
  if (value) return item?.[prop] === value ? `${label} (✅)` : label;
  return item?.[prop] ? `${label} (✅)` : label;
}

function renderMenuProps(prop: keyof CategoryItem, item: CategoryItem, value?: string) {
  if (!value) {
    return {
      class: item?.[prop] ? "font-bold text-[var(--n-color-danger)]" : "",
    };
  }
  return {
    class: item?.[prop] === value ? "font-bold text-[var(--n-color-danger)]" : "",
  };
}

export function useLayoutOrderMenu(
  itemRef: Ref<CategoryItem>,
  options?: { showDefault?: boolean; showLaunchCount?: boolean },
) {
  const { handleLayoutOrderSortChange } = useCategorySort(itemRef);
  const { showDefault = true, showLaunchCount = true } = options ?? {};

  const layoutMenu = computed(() => {
    const item = itemRef.value;
    return {
      label: t("common.layout"),
      key: "layout",
      icon: renderIcon("icon-buju"),
      children: [
        {
          label: renderLabel("layout", item, t("common.tile"), "grid"),
          key: "layout-grid",
          props: renderMenuProps("layout", item, "grid"),
          icon: renderIcon("icon-24gl-appsSmall"),
        },
        {
          label: renderLabel("layout", item, t("common.list"), "list"),
          key: "layout-list",
          props: renderMenuProps("layout", item, "list"),
          icon: renderIcon("icon-liebiao"),
        },
      ],
    };
  });

  const orderMenu = computed(() => {
    const item = itemRef.value;
    const sortByChildren = [];

    if (showDefault) {
      sortByChildren.push({
        label: renderLabel("sort_by", item, t("common.default"), "default"),
        key: "order-default",
        props: renderMenuProps("sort_by", item, "default"),
        icon: renderIcon("icon-moren1"),
      });
    }

    sortByChildren.push(
      {
        label: renderLabel("sort_by", item, t("common.name"), "name"),
        key: "order-name",
        props: renderMenuProps("sort_by", item, "name"),
        icon: renderIcon("icon-mingchengpaixu"),
      },
      {
        label: renderLabel("sort_by", item, t("common.type"), "type"),
        key: "order-type",
        props: renderMenuProps("sort_by", item, "type"),
        icon: renderIcon("icon-anleixingpaixu"),
      },
      {
        label: renderLabel("sort_by", item, t("common.date"), "time"),
        key: "order-time",
        props: renderMenuProps("sort_by", item, "time"),
        icon: renderIcon("icon-anchuangjianshijianpaixu"),
      },
      {
        label: renderLabel("sort_by", item, t("common.searchPriority"), "order"),
        key: "order-index",
        props: renderMenuProps("sort_by", item, "order"),
        icon: renderIcon("icon-youxianji"),
      },
    );

    if (showLaunchCount) {
      sortByChildren.push({
        label: renderLabel("sort_by", item, t("common.launch_count"), "launch_count"),
        key: "order-launch_count",
        props: renderMenuProps("sort_by", item, "launch_count"),
        icon: renderIcon("icon-qidongcishu"),
      });
    }

    const sortOrderChildren = [];

    if (showDefault) {
      sortOrderChildren.push({
        label: renderLabel("sort_order", item, t("common.default"), "default"),
        key: "sort-default",
        props: renderMenuProps("sort_order", item, "default"),
        icon: renderIcon("icon-paixu"),
      });
    }

    sortOrderChildren.push(
      {
        label: renderLabel("sort_order", item, t("common.ascending"), "asc"),
        key: "sort-asc",
        props: renderMenuProps("sort_order", item, "asc"),
        icon: renderIcon("icon-shengxu2"),
      },
      {
        label: renderLabel("sort_order", item, t("common.descending"), "desc"),
        key: "sort-desc",
        props: renderMenuProps("sort_order", item, "desc"),
        icon: renderIcon("icon-jiangxu2"),
      },
    );

    return {
      label: t("common.sortOrder"),
      key: "order",
      icon: renderIcon("icon-paixufangshi"),
      children: [...sortByChildren, { type: "divider", key: "d3" }, ...sortOrderChildren],
    };
  });

  function handleLayoutOrderSelect(key: string): boolean {
    const map: Record<string, () => Promise<void>> = {
      "layout-grid": () => handleLayoutOrderSortChange("grid", "layout"),
      "layout-list": () => handleLayoutOrderSortChange("list", "layout"),
      "order-default": () => handleLayoutOrderSortChange("default", "sort_by", true),
      "order-name": () => handleLayoutOrderSortChange("name", "sort_by", true),
      "order-type": () => handleLayoutOrderSortChange("type", "sort_by", true),
      "order-time": () => handleLayoutOrderSortChange("time", "sort_by", true),
      "order-index": () => handleLayoutOrderSortChange("order", "sort_by", true),
      "order-launch_count": () => handleLayoutOrderSortChange("launch_count", "sort_by", true),
      "sort-default": () => handleLayoutOrderSortChange("default", "sort_order", true),
      "sort-asc": () => handleLayoutOrderSortChange("asc", "sort_order", true),
      "sort-desc": () => handleLayoutOrderSortChange("desc", "sort_order", true),
    };

    const handler = map[key];
    if (handler) {
      void handler();
      return true;
    }
    return false;
  }

  return {
    layoutMenu,
    orderMenu,
    handleLayoutOrderSelect,
  };
}
