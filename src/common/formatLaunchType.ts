import { t } from "@/i18n";

const TYPE_MAP: Record<LaunchType, () => string> = {
  directory: () => t("format.folder"),
  file: () => t("format.file"),
  url: () => t("format.website"),
  alias: () => t("format.alias"),
  apps: () => t("format.apps"),
};

export function formatLaunchType(type: LaunchType) {
  return TYPE_MAP[type]?.() || t("format.unknown");
}
