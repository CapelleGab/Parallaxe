import { act, render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import type {
  AssetDto,
  AssetPageDto,
  LibraryDto,
  RootDto,
  ScanProgressDto,
  ScanResultDto,
} from "../../ipc/client";
import { LibraryHome } from "./LibraryHome";

const ipc = vi.hoisted(() => ({
  listRoots: vi.fn(),
  addRoot: vi.fn(),
  listAssets: vi.fn(),
  startScan: vi.fn(),
  cancelScan: vi.fn(),
}));

vi.mock("../../ipc/client", async (importOriginal) => ({
  ...(await importOriginal<typeof import("../../ipc/client")>()),
  ...ipc,
}));

const library: LibraryDto = { id: "lib-1", name: "Game assets", createdAt: 0 };
const completed: ScanResultDto = {
  outcome: "completed",
  progress: { files: 1, skipped: 0, unreadable: 0 },
  missing: 0,
};

function root(id: string, path: string, status: RootDto["status"] = "available"): RootDto {
  return { id, libraryId: library.id, path, status, isPrimary: id === "r1", lastSeenAt: null };
}

function asset(id: string, filename: string): AssetDto {
  return {
    id,
    rootId: "r1",
    relPath: filename,
    filename,
    ext: "png",
    kind: "image",
    sizeBytes: 2048,
    mtimeMs: null,
    status: "available",
    isFavorite: false,
  };
}

function page(items: AssetDto[], total = items.length): AssetPageDto {
  return { items, total };
}

/** A scan that only ends when the test says so. */
function controllableScan() {
  let report: (progress: ScanProgressDto) => void = () => {};
  let finish: (result: ScanResultDto) => void = () => {};
  ipc.startScan.mockImplementation(
    (_rootId: string, onProgress: (progress: ScanProgressDto) => void) => {
      report = onProgress;
      return new Promise<ScanResultDto>((resolve) => {
        finish = resolve;
      });
    },
  );
  return {
    report: (files: number) => act(() => report({ files, skipped: 0, unreadable: 0 })),
    finish: (result: ScanResultDto = completed) => act(() => finish(result)),
  };
}

describe("LibraryHome", () => {
  beforeEach(() => {
    Object.values(ipc).forEach((mock) => mock.mockReset());
    ipc.startScan.mockResolvedValue(completed);
    ipc.cancelScan.mockResolvedValue(true);
  });

  it("shows the files of an indexed library without rescanning", async () => {
    ipc.listRoots.mockResolvedValue([root("r1", "/art/Game assets")]);
    ipc.listAssets.mockResolvedValue(page([asset("a1", "oak.png"), asset("a2", "pine.png")]));
    render(<LibraryHome library={library} />);

    const grid = await screen.findByRole("list", { name: "Files" });
    expect(within(grid).getAllByRole("listitem")).toHaveLength(2);
    expect(screen.getByText("2 files in 1 folder")).toBeInTheDocument();
    expect(ipc.startScan).not.toHaveBeenCalled();
  });

  it("scans a library that was never indexed and shows live progress", async () => {
    ipc.listRoots.mockResolvedValue([root("r1", "/art/Game assets")]);
    ipc.listAssets.mockResolvedValueOnce(page([]));
    const scan = controllableScan();
    render(<LibraryHome library={library} />);

    await waitFor(() => expect(ipc.startScan).toHaveBeenCalledWith("r1", expect.any(Function)));
    await scan.report(1240);
    expect(screen.getByRole("status")).toHaveTextContent("Scanning… 1,240 files found");
    expect(screen.getByRole("navigation", { name: "Library folders" })).toHaveTextContent(
      "Scanning",
    );

    ipc.listAssets.mockResolvedValue(page([asset("a1", "oak.png")]));
    await scan.finish();

    expect(await screen.findByText("oak.png")).toBeInTheDocument();
    expect(screen.queryByRole("status")).not.toBeInTheDocument();
  });

  it("cancels a running scan", async () => {
    ipc.listRoots.mockResolvedValue([root("r1", "/art/Game assets")]);
    ipc.listAssets.mockResolvedValue(page([]));
    const scan = controllableScan();
    render(<LibraryHome library={library} />);

    await userEvent.click(await screen.findByRole("button", { name: "Cancel" }));
    expect(ipc.cancelScan).toHaveBeenCalledWith("r1");

    await scan.finish({ outcome: "cancelled", progress: { files: 0, skipped: 0, unreadable: 0 } });
    expect(await screen.findByRole("button", { name: "Rescan" })).toBeInTheDocument();
  });

  it("flags a folder found offline", async () => {
    ipc.listRoots.mockResolvedValue([root("r1", "/Volumes/Archive")]);
    ipc.listAssets.mockResolvedValue(page([]));
    ipc.startScan.mockResolvedValue({ outcome: "offline" });
    render(<LibraryHome library={library} />);

    const sidebar = await screen.findByRole("navigation", { name: "Library folders" });
    await waitFor(() => expect(sidebar).toHaveTextContent("Offline"));
  });

  it("rescans every folder on demand", async () => {
    ipc.listRoots.mockResolvedValue([root("r1", "/art/a"), root("r2", "/art/b")]);
    ipc.listAssets.mockResolvedValue(page([asset("a1", "oak.png")]));
    render(<LibraryHome library={library} />);

    await userEvent.click(await screen.findByRole("button", { name: "Rescan" }));

    await waitFor(() => expect(ipc.startScan).toHaveBeenCalledTimes(2));
    expect(ipc.startScan.mock.calls.map(([id]) => id)).toEqual(["r1", "r2"]);
  });

  it("adds an external folder and scans it", async () => {
    ipc.listRoots.mockResolvedValue([root("r1", "/art/Game assets")]);
    ipc.listAssets.mockResolvedValue(page([asset("a1", "oak.png")]));
    ipc.addRoot.mockResolvedValue(root("r2", "/projects/godot"));
    render(<LibraryHome library={library} />);

    await userEvent.click(await screen.findByRole("button", { name: "Add external folder" }));

    expect(await screen.findByText("godot")).toBeInTheDocument();
    await waitFor(() => expect(ipc.startScan).toHaveBeenCalledWith("r2", expect.any(Function)));
  });

  it("does nothing when the folder picker is cancelled", async () => {
    ipc.listRoots.mockResolvedValue([root("r1", "/art/Game assets")]);
    ipc.listAssets.mockResolvedValue(page([asset("a1", "oak.png")]));
    ipc.addRoot.mockResolvedValue(null);
    render(<LibraryHome library={library} />);

    await userEvent.click(await screen.findByRole("button", { name: "Add external folder" }));

    expect(ipc.startScan).not.toHaveBeenCalled();
    expect(screen.queryByRole("alert")).not.toBeInTheDocument();
  });

  it("explains why a folder was refused", async () => {
    ipc.listRoots.mockResolvedValue([root("r1", "/art/Game assets")]);
    ipc.listAssets.mockResolvedValue(page([asset("a1", "oak.png")]));
    ipc.addRoot.mockRejectedValue({
      code: "CONFLICT",
      message: "This item already exists.",
      retryable: false,
    });
    render(<LibraryHome library={library} />);

    await userEvent.click(await screen.findByRole("button", { name: "Add external folder" }));
    expect(await screen.findByRole("alert")).toHaveTextContent("This item already exists.");
  });

  it("loads more files on request", async () => {
    ipc.listRoots.mockResolvedValue([root("r1", "/art/Game assets")]);
    ipc.listAssets.mockResolvedValueOnce(page([asset("a1", "oak.png")], 2));
    ipc.listAssets.mockResolvedValueOnce(page([asset("a2", "pine.png")], 2));
    render(<LibraryHome library={library} />);

    await userEvent.click(await screen.findByRole("button", { name: "Show more" }));

    expect(await screen.findByText("pine.png")).toBeInTheDocument();
    expect(ipc.listAssets).toHaveBeenLastCalledWith("lib-1", 1, 200);
    expect(screen.queryByRole("button", { name: "Show more" })).not.toBeInTheDocument();
  });

  it("explains an empty library folder", async () => {
    ipc.listRoots.mockResolvedValue([root("r1", "/art/Game assets")]);
    ipc.listAssets.mockResolvedValue(page([]));
    render(<LibraryHome library={library} />);

    expect(await screen.findByRole("heading", { name: "No files yet" })).toBeInTheDocument();
  });

  it("invites to add a folder when there is none", async () => {
    ipc.listRoots.mockResolvedValue([]);
    ipc.listAssets.mockResolvedValue(page([]));
    render(<LibraryHome library={library} />);

    expect(
      await screen.findByRole("heading", { name: "Add your first folder" }),
    ).toBeInTheDocument();
    expect(ipc.startScan).not.toHaveBeenCalled();
  });

  it("reports when the library cannot be loaded", async () => {
    ipc.listRoots.mockRejectedValue(new Error("boom"));
    ipc.listAssets.mockResolvedValue(page([]));
    render(<LibraryHome library={library} />);

    expect(await screen.findByRole("alert")).toHaveTextContent("Something went wrong");
  });
});
