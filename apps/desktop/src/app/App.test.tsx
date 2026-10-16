import { render, screen } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { App } from "./App";

const getAppInfo = vi.fn();

vi.mock("../ipc/client", () => ({
  getAppInfo: () => getAppInfo(),
}));

describe("App", () => {
  beforeEach(() => {
    getAppInfo.mockReset();
  });

  it("renders the product promise", () => {
    getAppInfo.mockResolvedValue({ name: "Parallaxe", version: "0.0.0", coreVersion: "0.0.0" });
    render(<App />);

    expect(screen.getByRole("heading", { name: "Parallaxe" })).toBeInTheDocument();
    expect(screen.getByText(/never moves, renames or modifies/)).toBeInTheDocument();
  });

  it("shows versions returned by the backend", async () => {
    getAppInfo.mockResolvedValue({ name: "Parallaxe", version: "0.0.1", coreVersion: "0.0.2" });
    render(<App />);

    expect(await screen.findByText("v0.0.1 · core 0.0.2")).toBeInTheDocument();
  });

  it("falls back when the backend is unreachable", async () => {
    getAppInfo.mockRejectedValue(new Error("no backend"));
    render(<App />);

    expect(await screen.findByText("Development build")).toBeInTheDocument();
  });
});
