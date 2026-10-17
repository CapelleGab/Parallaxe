import { FolderIcon, LinkedFolderIcon } from "../../components/icons";
import type { RootDto } from "../../ipc/client";
import { cx } from "../../lib/cx";
import { splitPath } from "../../lib/path";

const statusLabel: Record<RootDto["status"], string> = {
  available: "Available",
  offline: "Offline",
};

type RootListProps = {
  roots: RootDto[];
};

export function RootList({ roots }: RootListProps) {
  return (
    <ul aria-label="Folders" className="flex flex-col">
      {roots.map((root) => {
        const { name, parent } = splitPath(root.path);
        const Icon = root.isPrimary ? FolderIcon : LinkedFolderIcon;
        return (
          <li
            key={root.id}
            className="grid grid-cols-[auto_1fr_auto] items-center gap-x-4 border-b border-border py-4 last:border-b-0"
          >
            <Icon className="size-6 text-fg-muted" />
            <div className="min-w-0">
              <p className="truncate font-medium" title={root.path}>
                {name}
              </p>
              <p className="truncate text-xs text-fg-muted select-text" title={root.path}>
                {root.isPrimary ? "Library folder" : "External folder"} in {parent || name}
              </p>
            </div>
            <span
              className={cx(
                "flex items-center gap-2 text-xs",
                root.status === "available" ? "text-success" : "text-warning",
              )}
            >
              <span aria-hidden="true" className="size-1.5 rounded-full bg-current" />
              {statusLabel[root.status]}
            </span>
          </li>
        );
      })}
    </ul>
  );
}
