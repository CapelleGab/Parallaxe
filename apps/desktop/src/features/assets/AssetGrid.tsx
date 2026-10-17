import { useVirtualizer } from "@tanstack/react-virtual";
import { useEffect, useLayoutEffect, useRef, useState } from "react";
import type { AssetDto } from "../../ipc/client";
import { AssetTile, TilePlaceholder } from "./AssetTile";

const MIN_TILE = 128;
const GAP = 16;
const ROW_GAP = 24;
const CAPTION = 44;

type AssetGridProps = {
  total: number;
  /** The asset at `index`, or undefined while its page is loading. */
  assetAt: (index: number) => AssetDto | undefined;
  /** Called with the index range on screen, so the caller can load it. */
  onVisibleRange: (start: number, end: number) => void;
};

/** Only the rows on screen exist in the page, so 100,000 files scroll like 100. */
export function AssetGrid({ total, assetAt, onVisibleRange }: AssetGridProps) {
  const scrollRef = useRef<HTMLDivElement>(null);
  const [width, setWidth] = useState(0);

  useLayoutEffect(() => {
    const element = scrollRef.current;
    if (!element) return;
    const observer = new ResizeObserver(([entry]) => {
      if (entry) setWidth(entry.contentRect.width);
    });
    observer.observe(element);
    return () => observer.disconnect();
  }, []);

  const columns = Math.max(1, Math.floor((width + GAP) / (MIN_TILE + GAP)));
  const tileWidth = width > 0 ? (width - GAP * (columns - 1)) / columns : MIN_TILE;
  const rowHeight = tileWidth + CAPTION + ROW_GAP;

  // TanStack Virtual returns fresh functions on purpose, so the React Compiler skips this component
  // eslint-disable-next-line react-hooks/incompatible-library
  const virtualizer = useVirtualizer({
    count: Math.ceil(total / columns),
    getScrollElement: () => scrollRef.current,
    estimateSize: () => rowHeight,
    overscan: 3,
    initialRect: { width: 1024, height: 768 },
  });

  useEffect(() => {
    virtualizer.measure();
  }, [virtualizer, rowHeight]);

  const rows = virtualizer.getVirtualItems();
  const firstRow = rows[0]?.index ?? 0;
  const lastRow = rows.at(-1)?.index ?? 0;

  useEffect(() => {
    if (total === 0) return;
    onVisibleRange(firstRow * columns, Math.min(total, (lastRow + 1) * columns));
  }, [firstRow, lastRow, columns, total, onVisibleRange]);

  return (
    <div ref={scrollRef} className="min-h-0 flex-1 overflow-auto px-10 pb-10">
      <div
        role="list"
        aria-label="Files"
        className="relative w-full"
        style={{ height: virtualizer.getTotalSize() }}
      >
        {rows.map((row) => (
          <div
            key={row.key}
            role="presentation"
            className="absolute top-0 left-0 grid w-full"
            style={{
              transform: `translateY(${row.start}px)`,
              gridTemplateColumns: `repeat(${columns}, minmax(0, 1fr))`,
              columnGap: GAP,
            }}
          >
            {Array.from({ length: columns }, (_, column) => {
              const index = row.index * columns + column;
              if (index >= total) return null;
              const asset = assetAt(index);
              return asset ? (
                <AssetTile key={asset.id} asset={asset} />
              ) : (
                <TilePlaceholder key={`placeholder-${index}`} />
              );
            })}
          </div>
        ))}
      </div>
    </div>
  );
}
