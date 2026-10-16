import { describe, expect, it, vi } from "vitest";
import { getAppInfo } from "./client";

const invoke = vi.hoisted(() => vi.fn());

vi.mock("@tauri-apps/api/core", () => ({ invoke }));

describe("getAppInfo", () => {
  it("calls the app_info command", async () => {
    invoke.mockResolvedValue({ name: "Parallaxe", version: "0.0.0", coreVersion: "0.0.0" });

    await expect(getAppInfo()).resolves.toEqual({
      name: "Parallaxe",
      version: "0.0.0",
      coreVersion: "0.0.0",
    });
    expect(invoke).toHaveBeenCalledWith("app_info");
  });
});
