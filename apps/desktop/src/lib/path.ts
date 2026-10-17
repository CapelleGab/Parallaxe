/** Splits a display path into its last segment and the folder that contains it. */
export function splitPath(path: string): { name: string; parent: string } {
  const trimmed = path.replace(/[\\/]+$/, "");
  const index = Math.max(trimmed.lastIndexOf("/"), trimmed.lastIndexOf("\\"));
  if (index < 0) return { name: trimmed || path, parent: "" };
  return { name: trimmed.slice(index + 1) || path, parent: trimmed.slice(0, index + 1) };
}
