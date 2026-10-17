import { useState, type FormEvent } from "react";
import { AppButton } from "../../components/AppButton";
import { Logo } from "../../components/Logo";
import { TextInput } from "../../components/TextInput";
import { createLibraryWithFolder, errorMessage, type LibraryDto } from "../../ipc/client";

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
    <main className="flex h-full flex-col items-center justify-center p-8">
      <div className="flex w-full max-w-sm flex-col items-center gap-6 text-center">
        <Logo className="size-14 text-accent" />
        <div className="space-y-2">
          <h1 className="text-2xl font-bold tracking-tight">Create your library</h1>
          <p className="text-fg-muted">
            Name it, then choose the folder where you keep your assets. You can create a new one
            from the folder picker.
          </p>
          <p className="text-xs text-fg-muted">
            Parallaxe indexes your folders. It never moves, renames or modifies your files.
          </p>
        </div>
        <form onSubmit={submit} className="flex w-full flex-col gap-4">
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
      </div>
    </main>
  );
}
