import "@testing-library/jest-dom/vitest";
import { cleanup } from "@testing-library/react";
import { afterEach } from "vitest";

// jsdom has no layout: give every element a fixed size so virtualized lists render rows
const WIDTH = 1000;
const HEIGHT = 800;

Object.defineProperties(HTMLElement.prototype, {
  clientWidth: { configurable: true, get: () => WIDTH },
  clientHeight: { configurable: true, get: () => HEIGHT },
});

class FixedResizeObserver {
  constructor(private readonly callback: ResizeObserverCallback) {}
  observe(target: Element) {
    const size = [{ inlineSize: WIDTH, blockSize: HEIGHT }];
    const entry = {
      target,
      contentRect: DOMRect.fromRect({ width: WIDTH, height: HEIGHT }),
      borderBoxSize: size,
      contentBoxSize: size,
    } as unknown as ResizeObserverEntry;
    this.callback([entry], this as unknown as ResizeObserver);
  }
  unobserve() {}
  disconnect() {}
}
globalThis.ResizeObserver = FixedResizeObserver as unknown as typeof ResizeObserver;

afterEach(() => {
  cleanup();
});
