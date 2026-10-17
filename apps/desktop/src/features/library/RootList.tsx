import type { RootDto } from "../../ipc/client";

const statusLabel: Record<RootDto["status"], string> = {
  available: "Available",
  offline: "Offline",
};

type RootListProps = {
  roots: RootDto[];
};

export function RootList({ roots }: RootListProps) {
  return (
    <ul aria-label="Folders" className="divide-y divide-border rounded-lg border border-border">
      {roots.map((root) => (
        <li key={root.id} className="flex items-center justify-between gap-4 px-4 py-3">
          <span className="truncate font-mono text-xs select-text" title={root.path}>
            {root.path}
          </span>
          <span
            className={
              root.status === "available"
                ? "shrink-0 text-xs text-success"
                : "shrink-0 text-xs text-warning"
            }
          >
            {statusLabel[root.status]}
          </span>
        </li>
      ))}
    </ul>
  );
}
