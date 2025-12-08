import { beforeEach, describe, expect, it, vi } from "vitest";
import {
  renderWithProviders,
  screen,
  userEvent,
} from "../../test-utils/test-helpers";
import { mockInvoke } from "../../test-utils/tauri-mocks";
import GlobalActions from "./GlobalActions";

describe("Global Actions", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders refresh and open buttons", () => {
    renderWithProviders(<GlobalActions />);

    expect(screen.getByTitle("Refresh current widgets")).toBeInTheDocument();
    expect(screen.getByTitle("Open widgets directory")).toBeInTheDocument();
  });

  it("calls refreshAll command when refresh button is clicked", async () => {
    const user = userEvent.setup();
    const refreshSpy = vi.fn(() => null);
    mockInvoke.register("plugin:deskulpt-widgets|refresh_all", refreshSpy);

    renderWithProviders(<GlobalActions />);

    const refreshButton = screen.getByTitle("Refresh current widgets");
    await user.click(refreshButton);

    expect(refreshSpy).toHaveBeenCalled();
  });

  it("calls open command when open button is clicked", async () => {
    const user = userEvent.setup();
    const openSpy = vi.fn(() => null);
    mockInvoke.register("plugin:deskulpt-core|open", openSpy);

    renderWithProviders(<GlobalActions />);

    const openButton = screen.getByTitle("Open widgets directory");
    await user.click(openButton);

    expect(openSpy).toHaveBeenCalledWith({ target: "widgets" });
  });
});
