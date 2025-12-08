import { beforeEach, describe, expect, it, vi } from "vitest";
import {
  renderWithProviders,
  screen,
  userEvent,
  waitFor,
} from "../../test-utils/test-helpers";
import { mockInvoke } from "../../test-utils/tauri-mocks";
import Settings from "./index";

describe("Settings Tab", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders canvas interaction mode selector", async () => {
    renderWithProviders(<Settings />);

    await waitFor(() => {
      expect(screen.getByText("Canvas interaction mode")).toBeInTheDocument();
    });
  });

  it("renders keyboard shortcuts section", async () => {
    renderWithProviders(<Settings />);

    await waitFor(() => {
      expect(
        screen.getByText("Toggle canvas interaction mode"),
      ).toBeInTheDocument();
      expect(screen.getByText("Open manager")).toBeInTheDocument();
    });
  });

  it("calls open command when edit settings.json button is clicked", async () => {
    const user = userEvent.setup();
    const openSpy = vi.fn(() => null);
    mockInvoke.register("plugin:deskulpt-core|open", openSpy);

    renderWithProviders(<Settings />);

    await waitFor(() => {
      expect(screen.getByText("Edit in settings.json")).toBeInTheDocument();
    });

    const editButton = screen.getByText("Edit in settings.json");
    await user.click(editButton);

    expect(openSpy).toHaveBeenCalledWith({ target: "settings" });
  });
});
