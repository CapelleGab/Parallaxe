import { beforeEach, describe, expect, it, vi } from "vitest";
import {
  addRoot,
  createLibrary,
  errorMessage,
  getAppInfo,
  isIpcError,
  listLibraries,
  listRoots,
  type IpcError,
} from "./client";

const invoke = vi.hoisted(() => vi.fn());

vi.mock("@tauri-apps/api/core", () => ({ invoke }));

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
    ["library_create", () => createLibrary("Games"), { name: "Games" }],
    ["library_list", () => listLibraries(), undefined],
    ["root_list", () => listRoots("lib-1"), { libraryId: "lib-1" }],
    ["library_add_root", () => addRoot("lib-1"), { libraryId: "lib-1" }],
  ])("calls %s with the expected arguments", async (command, run, args) => {
    await run();
    expect(invoke).toHaveBeenCalledWith(command, args);
  });

  it("passes backend errors through untouched", async () => {
    invoke.mockRejectedValue(conflict);
    await expect(createLibrary("Games")).rejects.toEqual(conflict);
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
