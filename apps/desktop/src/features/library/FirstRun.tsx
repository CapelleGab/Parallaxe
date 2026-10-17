import { useState, type FormEvent } from "react";
import { AppButton } from "../../components/AppButton";
import { TextInput } from "../../components/TextInput";
import { createLibraryWithFolder, errorMessage, type LibraryDto } from "../../ipc/client";
import { ParallaxStage } from "./ParallaxStage";

const MAX_NAME_CHARS = 64;

type FirstRunProps = {
  onCreated: (library: LibraryDto) => void;
};

export function FirstRun({ onCreated }: FirstRunProps) {
  const [name, setName] = useState("");
  const [pending, setPending] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function submit(event: FormEvent) {
    event.preventDefault();
    setPending(true);
    setError(null);
    try {
      const library = await createLibraryWithFolder(name);
      if (library) onCreated(library);
    } catch (failure) {
      setError(errorMessage(failure));
    } finally {
      setPending(false);
    }
  }

  return (
    <div className="flex h-full flex-col bg-surface">
      <div data-tauri-drag-region className="h-14 shrink-0" />
      <main className="grid flex-1 items-center gap-16 overflow-auto px-12 pb-14 lg:grid-cols-[1.1fr_1fr] lg:px-20">
        <div className="hidden justify-center lg:flex">
          <ParallaxStage label={name} />
        </div>
        <div className="flex max-w-sm flex-col gap-8">
          <div className="space-y-3">
            <h1 className="font-display text-5xl leading-none font-semibold tracking-tight [font-stretch:85%]">
              Create your library
            </h1>
            <p className="text-base text-fg-muted">
              Name it, then choose the folder where you keep your assets. You can create a new one
              from the folder picker.
            </p>
          </div>
          <form onSubmit={submit} className="flex flex-col gap-4">
            <TextInput
              label="Library name"
              placeholder="Game assets"
              value={name}
              maxLength={MAX_NAME_CHARS}
              autoFocus
              required
              error={error}
              onChange={(event) => setName(event.target.value)}
            />
            <AppButton type="submit" loading={pending} disabled={name.trim() === ""}>
              Choose folder…
            </AppButton>
          </form>
          <p className="text-sm text-fg-muted">
            Parallaxe indexes your folders. It never moves, renames or modifies your files.
          </p>
        </div>
      </main>
    </div>
  );
}
