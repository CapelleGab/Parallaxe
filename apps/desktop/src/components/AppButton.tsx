import type { ButtonHTMLAttributes } from "react";
import { cx } from "../lib/cx";

type AppButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: "primary" | "secondary";
  loading?: boolean;
};

const variants = {
  primary: "bg-accent text-on-accent hover:bg-accent/90",
  secondary: "border border-border bg-surface text-fg hover:bg-elevated",
};

export function AppButton({
  variant = "primary",
  loading = false,
  disabled,
  className,
  type = "button",
  children,
  ...props
}: AppButtonProps) {
  return (
    <button
      {...props}
      type={type}
      disabled={disabled || loading}
      aria-busy={loading || undefined}
      className={cx(
        "inline-flex h-9 items-center justify-center gap-2 rounded-md px-4 text-sm font-medium transition-colors duration-150 disabled:cursor-not-allowed disabled:opacity-50",
        variants[variant],
        className,
      )}
    >
      {children}
    </button>
  );
}
