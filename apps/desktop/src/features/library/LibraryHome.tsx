import type { LibraryDto } from "../../ipc/client";

type LibraryHomeProps = {
  library: LibraryDto;
};

export function LibraryHome({ library }: LibraryHomeProps) {
  return (
    <main className="flex h-full flex-col">
      <header className="flex h-12 items-center border-b border-border px-4">
        <h1 className="truncate text-base font-semibold" title={library.name}>
          {library.name}
        </h1>
      </header>
    </main>
  );
}
