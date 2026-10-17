type IconProps = { className?: string };

const base = {
  viewBox: "0 0 20 20",
  fill: "none",
  stroke: "currentColor",
  strokeWidth: 1.5,
  strokeLinecap: "round",
  strokeLinejoin: "round",
  "aria-hidden": true,
} as const;

export function FolderIcon({ className }: IconProps) {
  return (
    <svg {...base} className={className}>
      <path d="M2.75 5.5A1.75 1.75 0 0 1 4.5 3.75h3.1l1.9 2h6a1.75 1.75 0 0 1 1.75 1.75v7A1.75 1.75 0 0 1 15.5 16.25h-11A1.75 1.75 0 0 1 2.75 14.5z" />
    </svg>
  );
}

export function LinkedFolderIcon({ className }: IconProps) {
  return (
    <svg {...base} className={className}>
      <path d="M2.75 5.5A1.75 1.75 0 0 1 4.5 3.75h3.1l1.9 2h6a1.75 1.75 0 0 1 1.75 1.75v7A1.75 1.75 0 0 1 15.5 16.25h-11A1.75 1.75 0 0 1 2.75 14.5z" />
      <path d="M8.5 12.5 11.5 9.5M9 9.5h2.5V12" />
    </svg>
  );
}

export function PlusIcon({ className }: IconProps) {
  return (
    <svg {...base} className={className}>
      <path d="M10 4.5v11M4.5 10h11" />
    </svg>
  );
}
