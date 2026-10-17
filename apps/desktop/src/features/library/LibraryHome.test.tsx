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

function root(
  id: string,
  path: string,
  status: RootDto["status"] = "available",
  isPrimary = false,
): RootDto {
  return { id, libraryId: library.id, path, status, isPrimary, lastSeenAt: null };
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
      root("r1", "/data/textures", "available", true),
      root("r2", "/Volumes/External/sounds", "offline"),
    ]);
    render(<LibraryHome library={library} />);

    const items = within(await screen.findByRole("list", { name: "Folders" })).getAllByRole(
      "listitem",
    );
    expect(items[0]).toHaveTextContent("textures");
    expect(items[0]).toHaveTextContent("Library folder in /data/");
    expect(items[0]).toHaveTextContent("Available");
    expect(items[1]).toHaveTextContent("External folder in /Volumes/External/");
    expect(items[1]).toHaveTextContent("Offline");
    expect(screen.getByText("2 folders")).toBeInTheDocument();
  });

  it("adds the folder picked by the user", async () => {
    ipc.listRoots.mockResolvedValue([]);
    ipc.addRoot.mockResolvedValue(root("r1", "/data/textures"));
    render(<LibraryHome library={library} />);

    await userEvent.click(await screen.findByRole("button", { name: "Add external folder" }));

    expect(ipc.addRoot).toHaveBeenCalledWith("lib-1");
    const list = await screen.findByRole("list", { name: "Folders" });
    expect(within(list).getByText("textures")).toBeInTheDocument();
    expect(screen.getByText("1 folder")).toBeInTheDocument();
    expect(
      screen.queryByRole("heading", { name: "Add your first folder" }),
    ).not.toBeInTheDocument();
  });

  it("does nothing when the picker is cancelled", async () => {
    ipc.listRoots.mockResolvedValue([]);
    ipc.addRoot.mockResolvedValue(null);
    render(<LibraryHome library={library} />);

    await userEvent.click(await screen.findByRole("button", { name: "Add external folder" }));

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

    await userEvent.click(await screen.findByRole("button", { name: "Add external folder" }));

    expect(await screen.findByRole("alert")).toHaveTextContent("This item already exists.");
    expect(
      within(screen.getByRole("list", { name: "Folders" })).getByText("data"),
    ).toBeInTheDocument();
  });

  it("reports when folders cannot be loaded", async () => {
    ipc.listRoots.mockRejectedValue(new Error("boom"));
    render(<LibraryHome library={library} />);

    expect(await screen.findByRole("alert")).toHaveTextContent("Something went wrong");
  });

  it("shows folders in the sidebar and flags offline ones", async () => {
    ipc.listRoots.mockResolvedValue([
      root("r1", "/data/textures", "available", true),
      root("r2", "/Volumes/External/sounds", "offline"),
    ]);
    render(<LibraryHome library={library} />);

    const sidebar = await screen.findByRole("navigation", { name: "Library folders" });
    expect(within(sidebar).getByText("textures")).toBeInTheDocument();
    expect(within(sidebar).getByText("sounds").parentElement).toHaveTextContent("Offline");
  });
});
