import { beforeEach, describe, expect, it, vi } from "vitest";
import {
  renderWithProviders,
  screen,
  userEvent,
  waitFor,
} from "../../test-utils/test-helpers";
import { mockInvoke } from "../../test-utils/tauri-mocks";
import Logs from "./index";

describe("Logs Tab", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders empty state when no logs are available", async () => {
    mockInvoke.register("plugin:deskulpt-core|fetch_logs", () => ({
      entries: [],
      cursor: null,
      hasMore: false,
    }));

    renderWithProviders(<Logs />);

    await waitFor(() => {
      expect(screen.getByText("No log entries found")).toBeInTheDocument();
    });
  });

  it("allows changing minimum log level", async () => {
    const user = userEvent.setup();
    const fetchLogsSpy = vi.fn(() => ({
      entries: [],
      cursor: null,
      hasMore: false,
    }));

    mockInvoke.register("plugin:deskulpt-core|fetch_logs", fetchLogsSpy);

    renderWithProviders(<Logs />);

    // Wait for initial render
    await waitFor(() => {
      expect(screen.getByText("No log entries found")).toBeInTheDocument();
    });

    // Mock scrollIntoView to prevent errors
    const scrollIntoViewMock = vi.fn();
    Element.prototype.scrollIntoView = scrollIntoViewMock;

    // Find and interact with the level selector
    const levelSelect = screen.getByRole("combobox");
    await user.click(levelSelect);

    // Wait for the dropdown to open and select a different level
    await waitFor(
      async () => {
        const warnOption = screen.getByText("warn");
        if (warnOption) {
          await user.click(warnOption);
        }
      },
      { timeout: 2000 },
    );

    // Verify fetch_logs was called again with new level
    await waitFor(
      () => {
        expect(fetchLogsSpy).toHaveBeenCalledTimes(2);
      },
      { timeout: 3000 },
    );
  });
});
