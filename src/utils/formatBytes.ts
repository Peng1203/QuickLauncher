/**
 * 格式化字节大小
 * @param bytes 字节数
 * @param decimals 保留小数位
 */
export function formatBytes(bytes: number, decimals = 2): string {
  if (!bytes || bytes <= 0) {
    return '0 B';
  }

  const units = ['B', 'KB', 'MB', 'GB', 'TB', 'PB'];

  const base = 1024;

  const index = Math.floor(Math.log(bytes) / Math.log(base));

  const size = bytes / Math.pow(base, index);

  return `${size.toFixed(decimals)} ${units[index]}`;
}
