import { FolderIcon, LinkedFolderIcon } from "../../components/icons";
import type { RootDto } from "../../ipc/client";
import { splitPath } from "../../lib/path";

type SidebarFoldersProps = {
  roots: RootDto[];
};

export function SidebarFolders({ roots }: SidebarFoldersProps) {
  return (
    <nav aria-label="Library folders" className="flex flex-col gap-0.5 px-3">
      {roots.map((root) => {
        const Icon = root.isPrimary ? FolderIcon : LinkedFolderIcon;
        return (
          <div
            key={root.id}
            title={root.path}
            className="flex h-8 items-center gap-2.5 rounded-md px-2 text-sm text-fg-muted"
          >
            <Icon className="size-4 shrink-0" />
            <span className="truncate">{splitPath(root.path).name}</span>
            {root.status === "offline" && (
              <span className="ml-auto shrink-0 text-xs text-warning">Offline</span>
            )}
          </div>
        );
      })}
    </nav>
  );
}
