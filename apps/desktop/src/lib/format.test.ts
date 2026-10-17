import { describe, expect, it } from "vitest";
import { formatBytes, formatCount } from "./format";

describe("formatCount", () => {
  it.each([
    [0, "0 files"],
    [1, "1 file"],
    [1240, "1,240 files"],
  ])("formats %d", (value, expected) => {
    expect(formatCount(value, "file")).toBe(expected);
  });

  it("accepts irregular plurals", () => {
    expect(formatCount(2, "library", "libraries")).toBe("2 libraries");
  });
});

describe("formatBytes", () => {
  it.each([
    [0, "0 bytes"],
    [1, "1 byte"],
    [1023, "1,023 bytes"],
    [1024, "1 KiB"],
    [30_828_134, "29.4 MiB"],
    [5 * 1024 ** 3, "5 GiB"],
    [3 * 1024 ** 5, "3,072 TiB"],
  ])("formats %d", (bytes, expected) => {
    expect(formatBytes(bytes)).toBe(expected);
  });
});
