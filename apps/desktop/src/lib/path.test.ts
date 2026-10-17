import { describe, expect, it } from "vitest";
import { splitPath } from "./path";

describe("splitPath", () => {
  it.each([
    ["/Users/me/Art", "Art", "/Users/me/"],
    ["/Users/me/Art/", "Art", "/Users/me/"],
    ["C:\\Users\\me\\Art", "Art", "C:\\Users\\me\\"],
    ["\\\\?\\C:\\Assets", "Assets", "\\\\?\\C:\\"],
    ["/", "/", ""],
    ["Art", "Art", ""],
  ])("splits %s", (path, name, parent) => {
    expect(splitPath(path)).toEqual({ name, parent });
  });
});
