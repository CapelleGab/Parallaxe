import { render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import type { LibraryDto, RootDto } from "../../ipc/client";
import { LibraryHome } from "./LibraryHome";

const ipc = vi.hoisted(() => ({ listRoots: vi.fn(), addRoot: vi.fn() }));

vi.mock("../../ipc/client", async (importOriginal) => ({
  ...(await importOriginal<typeof import("../../ipc/client")>()),
  ...ipc,
}));

const library: LibraryDto = { id: "lib-1", name: "Game assets", createdAt: 0 };

function root(id: string, path: string, status: RootDto["status"] = "available"): RootDto {
  return { id, libraryId: library.id, path, status, lastSeenAt: null };
}

describe("LibraryHome", () => {
  beforeEach(() => {
    ipc.listRoots.mockReset();
    ipc.addRoot.mockReset();
  });

  it("invites to add a first folder", async () => {
    ipc.listRoots.mockResolvedValue([]);
    render(<LibraryHome library={library} />);

    expect(
      await screen.findByRole("heading", { name: "Add your first folder" }),
    ).toBeInTheDocument();
    expect(screen.getByText(/Nothing is copied or moved/)).toBeInTheDocument();
    expect(ipc.listRoots).toHaveBeenCalledWith("lib-1");
  });

  it("lists folders with their status", async () => {
    ipc.listRoots.mockResolvedValue([
      root("r1", "/data/textures"),
      root("r2", "/Volumes/External/sounds", "offline"),
    ]);
    render(<LibraryHome library={library} />);

    const items = within(await screen.findByRole("list", { name: "Folders" })).getAllByRole(
      "listitem",
    );
    expect(items[0]).toHaveTextContent("/data/textures");
    expect(items[0]).toHaveTextContent("Available");
    expect(items[1]).toHaveTextContent("Offline");
  });

  it("adds the folder picked by the user", async () => {
    ipc.listRoots.mockResolvedValue([]);
    ipc.addRoot.mockResolvedValue(root("r1", "/data/textures"));
    render(<LibraryHome library={library} />);

    await userEvent.click(await screen.findByRole("button", { name: "Add folder" }));

    expect(ipc.addRoot).toHaveBeenCalledWith("lib-1");
    expect(await screen.findByText("/data/textures")).toBeInTheDocument();
    expect(
      screen.queryByRole("heading", { name: "Add your first folder" }),
    ).not.toBeInTheDocument();
  });

  it("does nothing when the picker is cancelled", async () => {
    ipc.listRoots.mockResolvedValue([]);
    ipc.addRoot.mockResolvedValue(null);
    render(<LibraryHome library={library} />);

    await userEvent.click(await screen.findByRole("button", { name: "Add folder" }));

    expect(
      await screen.findByRole("heading", { name: "Add your first folder" }),
    ).toBeInTheDocument();
    expect(screen.queryByRole("alert")).not.toBeInTheDocument();
  });

  it("explains why a folder was refused", async () => {
    ipc.listRoots.mockResolvedValue([root("r1", "/data")]);
    ipc.addRoot.mockRejectedValue({
      code: "CONFLICT",
      message: "This item already exists.",
      retryable: false,
    });
    render(<LibraryHome library={library} />);

    await userEvent.click(await screen.findByRole("button", { name: "Add folder" }));

    expect(await screen.findByRole("alert")).toHaveTextContent("This item already exists.");
    expect(screen.getByText("/data")).toBeInTheDocument();
  });

  it("reports when folders cannot be loaded", async () => {
    ipc.listRoots.mockRejectedValue(new Error("boom"));
    render(<LibraryHome library={library} />);

    expect(await screen.findByRole("alert")).toHaveTextContent("Something went wrong");
  });
});
