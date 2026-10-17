import { beforeEach, describe, expect, it, vi } from "vitest";
import {
  addRoot,
  cancelScan,
  listAssets,
  startScan,
  createLibraryWithFolder,
  errorMessage,
  getAppInfo,
  isIpcError,
  listLibraries,
  listRoots,
  type IpcError,
} from "./client";

const invoke = vi.hoisted(() => vi.fn());

vi.mock("@tauri-apps/api/core", () => ({
  invoke,
  Channel: class {
    onmessage: (message: unknown) => void = () => {};
  },
}));

const conflict: IpcError = {
  code: "CONFLICT",
  message: "This item already exists.",
  retryable: false,
};

describe("ipc client", () => {
  beforeEach(() => {
    invoke.mockReset();
    invoke.mockResolvedValue(null);
  });

  it.each([
    ["app_info", () => getAppInfo(), undefined],
    ["library_create_with_folder", () => createLibraryWithFolder("Games"), { name: "Games" }],
    ["library_list", () => listLibraries(), undefined],
    ["root_list", () => listRoots("lib-1"), { libraryId: "lib-1" }],
    ["library_add_root", () => addRoot("lib-1"), { libraryId: "lib-1" }],
    ["scan_cancel", () => cancelScan("root-1"), { rootId: "root-1" }],
    [
      "asset_list",
      () => listAssets("lib-1", 200, 100),
      { libraryId: "lib-1", offset: 200, limit: 100 },
    ],
  ])("calls %s with the expected arguments", async (command, run, args) => {
    await run();
    expect(invoke).toHaveBeenCalledWith(command, args);
  });

  it("streams scan progress through a channel", async () => {
    const onProgress = vi.fn();
    await startScan("root-1", onProgress);

    const [command, args] = invoke.mock.calls[0] as [
      string,
      { rootId: string; onProgress: { onmessage: (p: unknown) => void } },
    ];
    expect(command).toBe("scan_start");
    expect(args.rootId).toBe("root-1");

    args.onProgress.onmessage({ files: 3, skipped: 0, unreadable: 0 });
    expect(onProgress).toHaveBeenCalledWith({ files: 3, skipped: 0, unreadable: 0 });
  });

  it("passes backend errors through untouched", async () => {
    invoke.mockRejectedValue(conflict);
    await expect(createLibraryWithFolder("Games")).rejects.toEqual(conflict);
  });

  it("wraps unexpected rejections in an Error", async () => {
    invoke.mockRejectedValue("command library_nope not found");
    await expect(listLibraries()).rejects.toThrow("command library_nope not found");
  });
});

describe("isIpcError", () => {
  it("recognizes backend errors", () => {
    expect(isIpcError(conflict)).toBe(true);
  });

  it.each([null, undefined, "oops", 42, { code: 1, message: "x" }, new Error("x")])(
    "rejects %p",
    (value) => {
      expect(isIpcError(value)).toBe(false);
    },
  );
});

describe("errorMessage", () => {
  it("uses the backend message", () => {
    expect(errorMessage(conflict)).toBe("This item already exists.");
  });

  it("hides unexpected details", () => {
    expect(errorMessage(new Error("SQLITE_BUSY at /Users/me/secret"))).toBe(
      "Something went wrong. Please try again.",
    );
  });
});
