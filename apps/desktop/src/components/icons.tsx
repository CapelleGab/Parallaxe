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

export function ImageIcon({ className }: IconProps) {
  return (
    <svg {...base} className={className}>
      <rect x="3" y="3.5" width="14" height="13" rx="2" />
      <circle cx="7.5" cy="8" r="1.5" />
      <path d="m3.5 14.5 4-4 3 3 2-2 4 4" />
    </svg>
  );
}

export function AudioIcon({ className }: IconProps) {
  return (
    <svg {...base} className={className}>
      <path d="M3 10h1.5M6.5 6.5v7M9.5 3.5v13M12.5 7v6M15.5 9v2M17 10h0" />
    </svg>
  );
}

export function VideoIcon({ className }: IconProps) {
  return (
    <svg {...base} className={className}>
      <rect x="2.75" y="4.5" width="14.5" height="11" rx="2" />
      <path d="m8.5 7.75 4 2.25-4 2.25z" />
    </svg>
  );
}

export function CubeIcon({ className }: IconProps) {
  return (
    <svg {...base} className={className}>
      <path d="m10 2.75 6.5 3.5v7.5L10 17.25l-6.5-3.5v-7.5z" />
      <path d="m3.5 6.25 6.5 3.5 6.5-3.5M10 9.75v7.5" />
    </svg>
  );
}

export function FileIcon({ className }: IconProps) {
  return (
    <svg {...base} className={className}>
      <path d="M5.5 2.75h6l3.75 3.75v10.75H5.5z" />
      <path d="M11.5 2.75V6.5h3.75" />
    </svg>
  );
}
