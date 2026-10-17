import { useState, type ComponentType } from "react";
import { AudioIcon, CubeIcon, FileIcon, ImageIcon, VideoIcon } from "../../components/icons";
import type { AssetDto } from "../../ipc/client";
import { cx } from "../../lib/cx";
import { formatBytes } from "../../lib/format";
import { thumbnailUrl } from "./thumbnail";

const kindIcon: Record<AssetDto["kind"], ComponentType<{ className?: string }>> = {
  image: ImageIcon,
  audio: AudioIcon,
  video: VideoIcon,
  model3d: CubeIcon,
  other: FileIcon,
};

const unavailableLabel: Partial<Record<AssetDto["status"], string>> = {
  missing: "Missing",
  root_offline: "Offline",
  error: "Unreadable",
};

export function AssetTile({ asset }: { asset: AssetDto }) {
  const [thumbnailFailed, setThumbnailFailed] = useState(false);
  const Icon = kindIcon[asset.kind];
  const unavailable = unavailableLabel[asset.status];
  const thumbnail = thumbnailFailed ? null : thumbnailUrl(asset);

  return (
    <div role="listitem" className={cx("flex min-w-0 flex-col gap-2", unavailable && "opacity-50")}>
      <div className="relative flex aspect-square items-center justify-center overflow-hidden rounded-md border border-border bg-elevated">
        {thumbnail ? (
          <img
            src={thumbnail}
            alt=""
            loading="lazy"
            decoding="async"
            draggable={false}
            onError={() => setThumbnailFailed(true)}
            className="size-full object-contain p-2"
          />
        ) : (
          <Icon className="size-9 text-fg-muted" />
        )}
        {asset.ext && (
          <span className="absolute bottom-2 left-2 rounded-xs bg-surface px-1.5 py-0.5 text-[11px] text-fg-muted">
            {asset.ext}
          </span>
        )}
      </div>
      <div className="min-w-0">
        <p className="truncate text-sm" title={asset.relPath}>
          {asset.filename}
        </p>
        <p className="text-xs text-fg-muted">{unavailable ?? formatBytes(asset.sizeBytes)}</p>
      </div>
    </div>
  );
}

/** Stands in for a tile whose data is still loading. */
export function TilePlaceholder() {
  return (
    <div aria-hidden="true" className="flex flex-col gap-2">
      <div className="aspect-square rounded-md border border-border bg-elevated/40" />
      <div className="h-4 w-2/3 rounded-xs bg-elevated/40" />
    </div>
  );
}
