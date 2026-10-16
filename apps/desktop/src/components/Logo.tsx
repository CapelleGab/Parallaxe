type LogoProps = {
  className?: string;
};

export function Logo({ className }: LogoProps) {
  return (
    <svg viewBox="0 0 64 64" fill="none" aria-hidden="true" className={className}>
      <rect
        x="6"
        y="14"
        width="34"
        height="34"
        rx="7"
        stroke="currentColor"
        strokeWidth="4"
        opacity=".45"
      />
      <rect x="24" y="16" width="34" height="34" rx="7" stroke="currentColor" strokeWidth="4" />
    </svg>
  );
}
