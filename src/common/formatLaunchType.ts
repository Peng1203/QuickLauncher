const TYPE_MAP: Record<LaunchType, string> = {
  directory: '文件夹',
  file: '文件',
  url: '网站',
};

export function formatLaunchType(type: LaunchType) {
  return TYPE_MAP[type] || '未知';
}
