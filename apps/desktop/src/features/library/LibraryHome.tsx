import { useEffect, useState } from "react";
import { AppButton } from "../../components/AppButton";
import { ErrorPanel } from "../../components/ErrorPanel";
import { PlusIcon } from "../../components/icons";
import { addRoot, errorMessage, listRoots, type LibraryDto, type RootDto } from "../../ipc/client";
import { cx } from "../../lib/cx";
import { isMac } from "../../lib/platform";
import { ContactSheet } from "./ContactSheet";
import { RootList } from "./RootList";
import { SidebarFolders } from "./SidebarFolders";

type LibraryHomeProps = {
  library: LibraryDto;
};

export function LibraryHome({ library }: LibraryHomeProps) {
  const [roots, setRoots] = useState<RootDto[] | null>(null);
  const [adding, setAdding] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let current = true;
    listRoots(library.id)
      .then((loaded) => current && setRoots(loaded))
      .catch((failure: unknown) => current && setError(errorMessage(failure)));
    return () => {
      current = false;
    };
  }, [library.id]);

  async function add() {
    setAdding(true);
    setError(null);
    try {
      const root = await addRoot(library.id);
      if (root) setRoots((previous) => [...(previous ?? []), root]);
    } catch (failure) {
      setError(errorMessage(failure));
    } finally {
      setAdding(false);
    }
  }

  const folderCount = roots?.length ?? 0;

  return (
    <div className="flex h-full">
      <aside className="flex w-60 shrink-0 flex-col border-r border-border bg-canvas">
        <div data-tauri-drag-region className={cx("shrink-0", isMac ? "h-14" : "h-4")} />
        <p className="truncate px-5 pb-4 font-display text-lg font-semibold" title={library.name}>
          {library.name}
        </p>
        {roots && <SidebarFolders roots={roots} />}
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
        <div className="mx-auto flex w-full max-w-3xl flex-1 flex-col gap-8 overflow-auto px-10 pb-10">
          <header>
            <h1 className="font-display text-5xl font-semibold tracking-tight [font-stretch:85%]">
              {library.name}
            </h1>
            {roots && (
              <p className="mt-1 text-fg-muted">
                {folderCount === 1 ? "1 folder" : `${folderCount} folders`}
              </p>
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

          {roots && roots.length > 0 && <RootList roots={roots} />}

          {roots && roots.length > 0 && (
            <section
              aria-labelledby="assets-heading"
              className="flex flex-col gap-3 border-t border-border pt-8"
            >
              <h2 id="assets-heading" className="font-display text-xl font-semibold">
                Nothing indexed yet
              </h2>
              <p className="max-w-md text-fg-muted">
                Your folders are saved. Browsing and search arrive with indexing, in the next
                version of Parallaxe.
              </p>
              <div className="mt-4">
                <ContactSheet />
              </div>
            </section>
          )}
        </div>
      </main>
    </div>
  );
}
