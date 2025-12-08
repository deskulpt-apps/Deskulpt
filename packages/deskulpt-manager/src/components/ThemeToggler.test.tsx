import { beforeEach, describe, expect, it, vi } from "vitest";
import {
  renderWithProviders,
  screen,
  userEvent,
} from "../test-utils/test-helpers";
import { mockInvoke } from "../test-utils/tauri-mocks";
import ThemeToggler from "./ThemeToggler";

describe("Theme Toggler", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders sun icon for light theme", () => {
    renderWithProviders(<ThemeToggler theme="light" />);

    const button = screen.getByTitle("Toggle theme");
    expect(button).toBeDefined();
  });

  it("renders moon icon for dark theme", () => {
    renderWithProviders(<ThemeToggler theme="dark" />);

    const button = screen.getByTitle("Toggle theme");
    expect(button).toBeDefined();
  });

  it("calls update command when clicked in light theme", async () => {
    const user = userEvent.setup();
    const updateSpy = vi.fn(() => null);
    mockInvoke.register("plugin:deskulpt-settings|update", updateSpy);

    renderWithProviders(<ThemeToggler theme="light" />);

    const button = screen.getByTitle("Toggle theme");
    await user.click(button);

    expect(updateSpy).toHaveBeenCalledWith({
      patch: { theme: "dark" },
    });
  });

  it("calls update command when clicked in dark theme", async () => {
    const user = userEvent.setup();
    const updateSpy = vi.fn(() => null);
    mockInvoke.register("plugin:deskulpt-settings|update", updateSpy);

    renderWithProviders(<ThemeToggler theme="dark" />);

    const button = screen.getByTitle("Toggle theme");
    await user.click(button);

    expect(updateSpy).toHaveBeenCalledWith({
      patch: { theme: "light" },
    });
  });
});
