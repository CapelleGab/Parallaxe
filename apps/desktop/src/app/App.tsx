import { useEffect, useState } from "react";
import { Logo } from "../components/Logo";
import { getAppInfo, type AppInfo } from "../ipc/client";

export function App() {
  const [info, setInfo] = useState<AppInfo | null>(null);

  useEffect(() => {
    getAppInfo()
      .then(setInfo)
      .catch(() => setInfo(null));
  }, []);

  return (
    <main className="flex h-full flex-col items-center justify-center gap-6 p-8 text-center">
      <Logo className="size-16 text-accent" />
      <div className="space-y-2">
        <h1 className="text-3xl font-bold tracking-tight">Parallaxe</h1>
        <p className="max-w-md text-base text-fg-muted">
          Parallaxe indexes your folders. It never moves, renames or modifies your files.
        </p>
      </div>
      <p className="rounded-md border border-border bg-surface px-3 py-1.5 font-mono text-xs text-fg-muted">
        {info ? `v${info.version} · core ${info.coreVersion}` : "Development build"}
      </p>
    </main>
  );
}
