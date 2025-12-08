import { beforeEach, describe, expect, it, vi } from "vitest";
import {
  renderWithProviders,
  screen,
  userEvent,
} from "../../test-utils/test-helpers";
import { mockInvoke } from "../../test-utils/tauri-mocks";
import Header from "./Header";
import { createWidgetCatalogEntry } from "../../test-utils/test-helpers";
import type { deskulptWidgets } from "@deskulpt/bindings";

describe("Widgets Header", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders widget ID badge", () => {
    const catalog = createWidgetCatalogEntry("widget-1", {
      name: "Test Widget",
    });

    renderWithProviders(<Header id="widget-1" />, {
      initialWidgetCatalog: catalog,
    });

    expect(screen.getByText("ID: widget-1")).toBeInTheDocument();
  });

  it("shows error badge for invalid widgets", () => {
    const catalog: deskulptWidgets.WidgetCatalog = {
      "widget-1": { type: "err", content: "Error" },
    };

    renderWithProviders(<Header id="widget-1" />, {
      initialWidgetCatalog: catalog,
    });

    expect(screen.getByText("ID: widget-1")).toBeInTheDocument();
  });

  it("calls refresh command when refresh button is clicked", async () => {
    const user = userEvent.setup();
    const catalog = createWidgetCatalogEntry("widget-1", {
      name: "Test Widget",
    });

    const refreshSpy = vi.fn(() => null);
    mockInvoke.register("plugin:deskulpt-widgets|refresh", refreshSpy);

    renderWithProviders(<Header id="widget-1" />, {
      initialWidgetCatalog: catalog,
    });

    const refreshButton = screen.getByTitle("Refresh this widget");
    await user.click(refreshButton);

    expect(refreshSpy).toHaveBeenCalledWith({ id: "widget-1" });
  });

  it("calls open command when edit button is clicked", async () => {
    const user = userEvent.setup();
    const catalog = createWidgetCatalogEntry("widget-1", {
      name: "Test Widget",
    });

    const openSpy = vi.fn(() => null);
    mockInvoke.register("plugin:deskulpt-core|open", openSpy);

    renderWithProviders(<Header id="widget-1" />, {
      initialWidgetCatalog: catalog,
    });

    const editButton = screen.getByTitle("Open this widget folder");
    await user.click(editButton);

    expect(openSpy).toHaveBeenCalledWith({ target: { widget: "widget-1" } });
  });
});
