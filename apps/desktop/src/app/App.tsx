import { useEffect, useState } from "react";
import { ErrorPanel } from "../components/ErrorPanel";
import { FirstRun } from "../features/library/FirstRun";
import { LibraryHome } from "../features/library/LibraryHome";
import { errorMessage, listLibraries, type LibraryDto } from "../ipc/client";

type State =
  | { status: "loading" }
  | { status: "failed"; message: string }
  | { status: "ready"; library: LibraryDto | null };

export function App() {
  const [state, setState] = useState<State>({ status: "loading" });
  const [attempt, setAttempt] = useState(0);

  useEffect(() => {
    let current = true;
    listLibraries()
      .then((libraries) => current && setState({ status: "ready", library: libraries[0] ?? null }))
      .catch(
        (failure: unknown) =>
          current && setState({ status: "failed", message: errorMessage(failure) }),
      );
    // ignore a response that arrives after a newer attempt or an unmount
    return () => {
      current = false;
    };
  }, [attempt]);

  function retry() {
    setState({ status: "loading" });
    setAttempt((n) => n + 1);
  }

  switch (state.status) {
    case "loading":
      return <main aria-busy="true" className="h-full" />;
    case "failed":
      return (
        <main className="flex h-full items-center justify-center p-8">
          <div className="w-full max-w-md">
            <ErrorPanel message={state.message} onRetry={retry} />
          </div>
        </main>
      );
    case "ready":
      return state.library ? (
        <LibraryHome library={state.library} />
      ) : (
        <FirstRun onCreated={(library) => setState({ status: "ready", library })} />
      );
  }
}
