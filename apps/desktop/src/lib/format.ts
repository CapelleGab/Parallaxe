const count = new Intl.NumberFormat("en");
const decimal = new Intl.NumberFormat("en", { maximumFractionDigits: 1 });
const units = ["KiB", "MiB", "GiB", "TiB"];

export function formatCount(value: number, singular: string, plural = `${singular}s`): string {
  return `${count.format(value)} ${value === 1 ? singular : plural}`;
}

/** Binary units, as file managers count them: 1 KiB = 1,024 bytes. */
export function formatBytes(bytes: number): string {
  if (bytes < 1024) return formatCount(bytes, "byte");
  let value = bytes / 1024;
  let unit = 0;
  while (value >= 1024 && unit < units.length - 1) {
    value /= 1024;
    unit += 1;
  }
  return `${decimal.format(value)} ${units[unit]}`;
}
