import { render, screen, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import type { AssetDto } from "../../ipc/client";
import { AssetGrid } from "./AssetGrid";

function asset(overrides: Partial<AssetDto>): AssetDto {
  return {
    id: "a1",
    rootId: "r1",
    relPath: "textures/oak.png",
    filename: "oak.png",
    ext: "png",
    kind: "image",
    sizeBytes: 30_828_134,
    mtimeMs: null,
    status: "available",
    isFavorite: false,
    ...overrides,
  };
}

describe("AssetGrid", () => {
  it("shows name, size and extension", () => {
    render(<AssetGrid assets={[asset({})]} />);
    const tile = within(screen.getByRole("list", { name: "Files" })).getByRole("listitem");

    expect(tile).toHaveTextContent("oak.png");
    expect(tile).toHaveTextContent("29.4 MiB");
    expect(tile).toHaveTextContent("png");
    expect(screen.getByTitle("textures/oak.png")).toBeInTheDocument();
  });

  it.each([
    ["missing", "Missing"],
    ["root_offline", "Offline"],
    ["error", "Unreadable"],
  ] as const)("says when a file is %s instead of its size", (status, label) => {
    render(<AssetGrid assets={[asset({ status })]} />);
    expect(screen.getByRole("listitem")).toHaveTextContent(label);
    expect(screen.getByRole("listitem")).not.toHaveTextContent("MiB");
  });

  it("omits the extension badge for files without one", () => {
    render(<AssetGrid assets={[asset({ ext: "", filename: "LICENSE", kind: "other" })]} />);
    expect(screen.getByRole("listitem")).toHaveTextContent("LICENSE");
    expect(screen.getByRole("listitem")).not.toHaveTextContent("png");
  });
});
