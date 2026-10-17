import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import type { IpcError, LibraryDto } from "../ipc/client";
import { App } from "./App";

const ipc = vi.hoisted(() => ({
  listLibraries: vi.fn(),
  createLibrary: vi.fn(),
  listRoots: vi.fn(),
  addRoot: vi.fn(),
}));

vi.mock("../ipc/client", async (importOriginal) => ({
  ...(await importOriginal<typeof import("../ipc/client")>()),
  ...ipc,
}));

const games: LibraryDto = { id: "lib-1", name: "Game assets", createdAt: 0 };

describe("App", () => {
  beforeEach(() => {
    Object.values(ipc).forEach((mock) => mock.mockReset());
    ipc.listRoots.mockResolvedValue([]);
  });

  it("asks for a library name on first run", async () => {
    ipc.listLibraries.mockResolvedValue([]);
    render(<App />);

    expect(await screen.findByRole("heading", { name: "Create your library" })).toBeInTheDocument();
    expect(screen.getByText(/never moves, renames or modifies/)).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Create library" })).toBeDisabled();
  });

  it("creates the library and opens it", async () => {
    ipc.listLibraries.mockResolvedValue([]);
    ipc.createLibrary.mockResolvedValue(games);
    render(<App />);

    await userEvent.type(await screen.findByLabelText("Library name"), "Game assets");
    await userEvent.click(screen.getByRole("button", { name: "Create library" }));

    expect(ipc.createLibrary).toHaveBeenCalledWith("Game assets");
    expect(await screen.findByRole("heading", { name: "Game assets" })).toBeInTheDocument();
  });

  it("shows why a library could not be created", async () => {
    const invalid: IpcError = {
      code: "INVALID_ARGUMENT",
      message: "name is longer than 64 characters",
      retryable: false,
    };
    ipc.listLibraries.mockResolvedValue([]);
    ipc.createLibrary.mockRejectedValue(invalid);
    render(<App />);

    await userEvent.type(await screen.findByLabelText("Library name"), "x");
    await userEvent.click(screen.getByRole("button", { name: "Create library" }));

    expect(await screen.findByText("name is longer than 64 characters")).toBeInTheDocument();
    expect(screen.getByLabelText("Library name")).toHaveAttribute("aria-invalid", "true");
  });

  it("opens the existing library directly", async () => {
    ipc.listLibraries.mockResolvedValue([games]);
    render(<App />);

    expect(await screen.findByRole("heading", { name: "Game assets" })).toBeInTheDocument();
    expect(screen.queryByLabelText("Library name")).not.toBeInTheDocument();
  });

  it("lets the user retry when the catalog cannot be read", async () => {
    ipc.listLibraries.mockRejectedValueOnce({
      code: "DATABASE_ERROR",
      message: "The catalog could not be read or written.",
      retryable: true,
    });
    ipc.listLibraries.mockResolvedValueOnce([games]);
    render(<App />);

    expect(await screen.findByRole("alert")).toHaveTextContent("could not be read");
    await userEvent.click(screen.getByRole("button", { name: "Try again" }));
    expect(await screen.findByRole("heading", { name: "Game assets" })).toBeInTheDocument();
  });
});
