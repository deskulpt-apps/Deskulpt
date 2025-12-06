export function formatBytes(bytes: number) {
  const k = 1024;
  if (bytes < 1024) {
    return `${bytes} B`;
  }

  const units = ["KB", "MB", "GB", "TB"] as const;
  let value = bytes;
  let unitIndex = -1;
  while (value >= k && unitIndex < units.length - 1) {
    value /= k;
    unitIndex++;
  }
  return `${value.toFixed(2)} ${units[unitIndex]}`;
}
