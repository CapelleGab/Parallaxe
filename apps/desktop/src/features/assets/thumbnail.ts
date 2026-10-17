import { convertFileSrc } from "@tauri-apps/api/core";
import type { AssetDto } from "../../ipc/client";

const PREVIEWABLE = new Set(["png", "jpg", "jpeg", "webp", "gif"]);

/** URL of the asset's thumbnail, or null when the backend cannot produce one. */
export function thumbnailUrl(asset: AssetDto): string | null {
  if (asset.kind !== "image" || asset.status !== "available" || !PREVIEWABLE.has(asset.ext)) {
    return null;
  }
  // the version makes a changed file load a fresh thumbnail
  return `${convertFileSrc(asset.id, "thumb")}?v=${asset.sizeBytes}-${asset.mtimeMs ?? 0}`;
}
