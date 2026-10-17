import { useEffect, useState } from "react";
import { AppButton } from "../../components/AppButton";
import { ErrorPanel } from "../../components/ErrorPanel";
import { addRoot, errorMessage, listRoots, type LibraryDto, type RootDto } from "../../ipc/client";
import { RootList } from "./RootList";

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

  const addButton = (
    <AppButton onClick={add} loading={adding}>
      Add folder
    </AppButton>
  );

  return (
    <main className="flex h-full flex-col">
      <header className="flex h-12 shrink-0 items-center justify-between gap-4 border-b border-border px-4">
        <h1 className="truncate text-base font-semibold" title={library.name}>
          {library.name}
        </h1>
        {roots && roots.length > 0 && addButton}
      </header>

      <section className="flex flex-1 flex-col gap-4 overflow-auto p-4">
        {error && <ErrorPanel message={error} />}
        {roots && roots.length === 0 && (
          <div className="m-auto flex max-w-sm flex-col items-center gap-3 text-center">
            <h2 className="text-lg font-semibold">Add your first folder</h2>
            <p className="text-fg-muted">
              Parallaxe will index it where it is. Nothing is copied or moved.
            </p>
            {addButton}
          </div>
        )}
        {roots && roots.length > 0 && <RootList roots={roots} />}
      </section>
    </main>
  );
}
