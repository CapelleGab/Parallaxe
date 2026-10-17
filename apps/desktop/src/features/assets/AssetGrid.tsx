import type { ComponentType } from "react";
import { AudioIcon, CubeIcon, FileIcon, ImageIcon, VideoIcon } from "../../components/icons";
import type { AssetDto } from "../../ipc/client";
import { cx } from "../../lib/cx";
import { formatBytes } from "../../lib/format";

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

type AssetGridProps = {
  assets: AssetDto[];
};

export function AssetGrid({ assets }: AssetGridProps) {
  return (
    <ul
      aria-label="Files"
      className="grid grid-cols-[repeat(auto-fill,minmax(8rem,1fr))] gap-x-4 gap-y-6"
    >
      {assets.map((asset) => (
        <AssetTile key={asset.id} asset={asset} />
      ))}
    </ul>
  );
}

function AssetTile({ asset }: { asset: AssetDto }) {
  const Icon = kindIcon[asset.kind];
  const unavailable = unavailableLabel[asset.status];

  return (
    <li className={cx("flex min-w-0 flex-col gap-2", unavailable && "opacity-50")}>
      <div className="relative flex aspect-square items-center justify-center rounded-md border border-border bg-elevated">
        <Icon className="size-9 text-fg-muted" />
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
    </li>
  );
}
