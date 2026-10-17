import { AppButton } from "./AppButton";

type ErrorPanelProps = {
  message: string;
  onRetry?: () => void;
};

export function ErrorPanel({ message, onRetry }: ErrorPanelProps) {
  return (
    <div
      role="alert"
      className="flex items-center justify-between gap-4 rounded-md border border-danger/40 bg-danger/10 px-4 py-3 text-sm"
    >
      <p>{message}</p>
      {onRetry && (
        <AppButton variant="secondary" onClick={onRetry}>
          Try again
        </AppButton>
      )}
    </div>
  );
}
