import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { AppButton } from "./AppButton";
import { ErrorPanel } from "./ErrorPanel";
import { TextInput } from "./TextInput";

describe("AppButton", () => {
  it("is a plain button by default", () => {
    render(<AppButton>Save</AppButton>);
    expect(screen.getByRole("button", { name: "Save" })).toHaveAttribute("type", "button");
  });

  it("is disabled and busy while loading", async () => {
    const onClick = vi.fn();
    render(
      <AppButton loading onClick={onClick}>
        Save
      </AppButton>,
    );
    const button = screen.getByRole("button", { name: "Save" });

    expect(button).toBeDisabled();
    expect(button).toHaveAttribute("aria-busy", "true");
    await userEvent.click(button);
    expect(onClick).not.toHaveBeenCalled();
  });
});

describe("TextInput", () => {
  it("links its label, hint and error", () => {
    render(<TextInput label="Library name" hint="Up to 64 characters" error="Required" />);
    const input = screen.getByLabelText("Library name");

    expect(input).toHaveAccessibleDescription("Up to 64 characters Required");
    expect(input).toHaveAttribute("aria-invalid", "true");
  });

  it("is valid without error", () => {
    render(<TextInput label="Library name" />);
    expect(screen.getByLabelText("Library name")).not.toHaveAttribute("aria-invalid");
  });
});

describe("ErrorPanel", () => {
  it("announces the message", () => {
    render(<ErrorPanel message="The catalog could not be read." />);
    expect(screen.getByRole("alert")).toHaveTextContent("The catalog could not be read.");
    expect(screen.queryByRole("button")).not.toBeInTheDocument();
  });

  it("offers a retry when possible", async () => {
    const onRetry = vi.fn();
    render(<ErrorPanel message="Failed" onRetry={onRetry} />);

    await userEvent.click(screen.getByRole("button", { name: "Try again" }));
    expect(onRetry).toHaveBeenCalledOnce();
  });
});
