import { render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import type { AssetDto } from "../../ipc/client";
import { AssetGrid } from "./AssetGrid";
import { AssetTile } from "./AssetTile";

vi.mock("@tauri-apps/api/core", () => ({
  convertFileSrc: (path: string, protocol: string) => `${protocol}://localhost/${path}`,
}));

function asset(overrides: Partial<AssetDto> = {}): AssetDto {
  return {
    id: "a1",
    rootId: "r1",
    relPath: "textures/oak.png",
    filename: "oak.png",
    ext: "png",
    kind: "image",
    sizeBytes: 30_828_134,
    mtimeMs: 7,
    status: "available",
    isFavorite: false,
    ...overrides,
  };
}

describe("AssetTile", () => {
  it("shows name, size and extension", () => {
    render(<AssetTile asset={asset({ kind: "model3d", ext: "glb", filename: "gate.glb" })} />);
    const tile = screen.getByRole("listitem");

    expect(tile).toHaveTextContent("gate.glb");
    expect(tile).toHaveTextContent("29.4 MiB");
    expect(tile).toHaveTextContent("glb");
    expect(tile.querySelector("img")).toBeNull();
  });

  it("loads a versioned thumbnail for images", () => {
    render(<AssetTile asset={asset()} />);
    expect(screen.getByRole("listitem").querySelector("img")).toHaveAttribute(
      "src",
      "thumb://localhost/a1?v=30828134-7",
    );
  });

  it("falls back to the type icon when the thumbnail fails", async () => {
    render(<AssetTile asset={asset()} />);
    const image = screen.getByRole("listitem").querySelector("img");
    image?.dispatchEvent(new Event("error"));

    await vi.waitFor(() => expect(screen.getByRole("listitem").querySelector("img")).toBeNull());
  });

  it.each([
    ["missing", "Missing"],
    ["root_offline", "Offline"],
    ["error", "Unreadable"],
  ] as const)("says when a file is %s and shows no thumbnail", (status, label) => {
    render(<AssetTile asset={asset({ status })} />);
    const tile = screen.getByRole("listitem");
    expect(tile).toHaveTextContent(label);
    expect(tile).not.toHaveTextContent("MiB");
    expect(tile.querySelector("img")).toBeNull();
  });

  it("omits the extension badge for files without one", () => {
    render(<AssetTile asset={asset({ ext: "", filename: "LICENSE", kind: "other" })} />);
    expect(screen.getByRole("listitem")).toHaveTextContent("LICENSE");
    expect(screen.getByRole("listitem")).not.toHaveTextContent("png");
  });
});

describe("AssetGrid", () => {
  it("renders only the rows on screen and asks for their range", () => {
    const onVisibleRange = vi.fn();
    render(
      <AssetGrid
        total={100_000}
        assetAt={(index) => asset({ id: `a${index}`, filename: `${index}.png` })}
        onVisibleRange={onVisibleRange}
      />,
    );

    const tiles = within(screen.getByRole("list", { name: "Files" })).getAllByRole("listitem");
    expect(tiles.length).toBeGreaterThan(0);
    expect(tiles.length).toBeLessThan(200);
    expect(onVisibleRange).toHaveBeenCalledWith(0, tiles.length);
  });

  it("shows placeholders for rows still loading", () => {
    render(<AssetGrid total={10} assetAt={() => undefined} onVisibleRange={() => {}} />);
    expect(screen.queryAllByRole("listitem")).toHaveLength(0);
    expect(screen.getByRole("list", { name: "Files" }).children[0]?.children).toHaveLength(7);
  });
});
