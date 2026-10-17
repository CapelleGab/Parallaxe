import { useCallback, useEffect, useRef, useState } from "react";
import { AppButton } from "../../components/AppButton";
import { ErrorPanel } from "../../components/ErrorPanel";
import { PlusIcon } from "../../components/icons";
import {
  addRoot,
  cancelScan,
  errorMessage,
  listAssets,
  listRoots,
  startScan,
  type AssetDto,
  type AssetPageDto,
  type LibraryDto,
  type RootDto,
  type ScanProgressDto,
} from "../../ipc/client";
import { cx } from "../../lib/cx";
import { formatCount } from "../../lib/format";
import { isMac } from "../../lib/platform";
import { AssetGrid } from "../assets/AssetGrid";
import { ContactSheet } from "./ContactSheet";
import { SidebarFolders } from "./SidebarFolders";

const PAGE_SIZE = 200;
const NO_PROGRESS: ScanProgressDto = { files: 0, skipped: 0, unreadable: 0 };

type LibraryHomeProps = {
  library: LibraryDto;
};

export function LibraryHome({ library }: LibraryHomeProps) {
  const [roots, setRoots] = useState<RootDto[] | null>(null);
  const [pages, setPages] = useState<Record<number, AssetDto[]>>({});
  const [total, setTotal] = useState<number | null>(null);
  const [scans, setScans] = useState<Record<string, ScanProgressDto>>({});
  const [adding, setAdding] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const autoScanned = useRef(false);
  const loadingPages = useRef(new Set<number>());
  // bumped on every reload so answers to older requests are ignored
  const generation = useRef(0);

  const applyFirstPage = useCallback((page: AssetPageDto) => {
    generation.current += 1;
    loadingPages.current.clear();
    setPages({ 0: page.items });
    setTotal(page.total);
  }, []);

  const reloadAssets = useCallback(async () => {
    applyFirstPage(await listAssets(library.id, 0, PAGE_SIZE));
  }, [library.id, applyFirstPage]);

  const loadRange = useCallback(
    (start: number, end: number) => {
      const requested = generation.current;
      for (let page = Math.floor(start / PAGE_SIZE); page * PAGE_SIZE < end; page += 1) {
        if (pages[page] || loadingPages.current.has(page)) continue;
        loadingPages.current.add(page);
        listAssets(library.id, page * PAGE_SIZE, PAGE_SIZE)
          .then((result) => {
            if (requested !== generation.current) return;
            setPages((current) => ({ ...current, [page]: result.items }));
            setTotal(result.total);
          })
          .catch((failure: unknown) => setError(errorMessage(failure)))
          .finally(() => loadingPages.current.delete(page));
      }
    },
    [library.id, pages],
  );

  const assetAt = useCallback(
    (index: number) => pages[Math.floor(index / PAGE_SIZE)]?.[index % PAGE_SIZE],
    [pages],
  );

  const scanRoot = useCallback(
    async (root: RootDto) => {
      setScans((current) => ({ ...current, [root.id]: NO_PROGRESS }));
      try {
        const result = await startScan(root.id, (progress) =>
          setScans((current) => ({ ...current, [root.id]: progress })),
        );
        const status = result.outcome === "offline" ? "offline" : "available";
        setRoots(
          (current) => current?.map((r) => (r.id === root.id ? { ...r, status } : r)) ?? null,
        );
      } catch (failure) {
        setError(errorMessage(failure));
      } finally {
        setScans((current) => withoutKey(current, root.id));
      }
      await reloadAssets().catch((failure: unknown) => setError(errorMessage(failure)));
    },
    [reloadAssets],
  );

  const scanAll = useCallback(
    async (list: RootDto[]) => {
      setError(null);
      for (const root of list) await scanRoot(root);
    },
    [scanRoot],
  );

  useEffect(() => {
    let current = true;
    Promise.all([listRoots(library.id), listAssets(library.id, 0, PAGE_SIZE)])
      .then(([loadedRoots, page]) => {
        if (!current) return;
        setRoots(loadedRoots);
        applyFirstPage(page);
        // a library that was never indexed, typically one just created
        if (page.total === 0 && loadedRoots.length > 0 && !autoScanned.current) {
          autoScanned.current = true;
          void scanAll(loadedRoots);
        }
      })
      .catch((failure: unknown) => current && setError(errorMessage(failure)));
    return () => {
      current = false;
    };
  }, [library.id, scanAll, applyFirstPage]);

  async function add() {
    setAdding(true);
    setError(null);
    try {
      const root = await addRoot(library.id);
      if (root) {
        setRoots((previous) => [...(previous ?? []), root]);
        void scanRoot(root);
      }
    } catch (failure) {
      setError(errorMessage(failure));
    } finally {
      setAdding(false);
    }
  }

  function cancel() {
    for (const rootId of Object.keys(scans)) {
      void cancelScan(rootId).catch((failure: unknown) => setError(errorMessage(failure)));
    }
  }

  const scanning = Object.keys(scans).length > 0;
  const filesFound = Object.values(scans).reduce((sum, progress) => sum + progress.files, 0);

  return (
    <div className="flex h-full">
      <aside className="flex w-60 shrink-0 flex-col border-r border-border bg-canvas">
        <div data-tauri-drag-region className={cx("shrink-0", isMac ? "h-14" : "h-4")} />
        <p className="truncate px-5 pb-4 font-display text-lg font-semibold" title={library.name}>
          {library.name}
        </p>
        {roots && <SidebarFolders roots={roots} scanning={Object.keys(scans)} />}
        <div className="mt-auto p-3">
          <AppButton
            variant="secondary"
            onClick={add}
            loading={adding}
            className="w-full justify-start"
          >
            <PlusIcon className="size-4" />
            Add external folder
          </AppButton>
        </div>
      </aside>

      <main className="flex min-w-0 flex-1 flex-col bg-surface">
        <div data-tauri-drag-region className="h-14 shrink-0" />
        <div className="flex shrink-0 flex-col gap-6 px-10 pb-8">
          <header className="flex flex-wrap items-end justify-between gap-4">
            <div>
              <h1 className="font-display text-5xl font-semibold tracking-tight [font-stretch:85%]">
                {library.name}
              </h1>
              {roots && total !== null && (
                <p className="mt-1 text-fg-muted">
                  {formatCount(total, "file")} in {formatCount(roots.length, "folder")}
                </p>
              )}
            </div>
            {scanning ? (
              <div className="flex items-center gap-4">
                <p role="status" className="text-sm text-fg-muted">
                  Scanning… {formatCount(filesFound, "file")} found
                </p>
                <AppButton variant="secondary" onClick={cancel}>
                  Cancel
                </AppButton>
              </div>
            ) : (
              roots &&
              roots.length > 0 && (
                <AppButton variant="secondary" onClick={() => void scanAll(roots)}>
                  Rescan
                </AppButton>
              )
            )}
          </header>

          {error && <ErrorPanel message={error} />}

          {roots && roots.length === 0 && (
            <div className="flex flex-col items-start gap-3">
              <h2 className="font-display text-xl font-semibold">Add your first folder</h2>
              <p className="max-w-md text-fg-muted">
                Parallaxe will index it where it is. Nothing is copied or moved.
              </p>
            </div>
          )}

          {total === 0 && scanning && <ContactSheet />}

          {total === 0 && !scanning && roots && roots.length > 0 && (
            <div className="flex flex-col items-start gap-3">
              <h2 className="font-display text-xl font-semibold">No files yet</h2>
              <p className="max-w-md text-fg-muted">
                Put files in your library folder, then rescan.
              </p>
            </div>
          )}
        </div>

        {total !== null && total > 0 && (
          <AssetGrid total={total} assetAt={assetAt} onVisibleRange={loadRange} />
        )}
      </main>
    </div>
  );
}

function withoutKey<T>(record: Record<string, T>, key: string): Record<string, T> {
  return Object.fromEntries(Object.entries(record).filter(([entry]) => entry !== key));
}
