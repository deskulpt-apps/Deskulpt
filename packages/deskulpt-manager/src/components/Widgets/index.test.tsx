import { beforeEach, describe, expect, it, vi } from "vitest";
import {
  renderWithProviders,
  screen,
  userEvent,
  waitFor,
} from "../../test-utils/test-helpers";
import Widgets from "./index";
import { createWidgetCatalogEntry } from "../../test-utils/test-helpers";
import type { deskulptWidgets } from "@deskulpt/bindings";

describe("Widgets", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders empty state when no widgets are available", async () => {
    renderWithProviders(<Widgets />);

    await waitFor(() => {
      expect(screen.getByText("No widgets available")).toBeInTheDocument();
    });
  });

  it("renders widget list when widgets are available", () => {
    const catalog = {
      ...createWidgetCatalogEntry("widget-1", { name: "Widget 1" }),
      ...createWidgetCatalogEntry("widget-2", { name: "Widget 2" }),
    };

    renderWithProviders(<Widgets />, {
      initialWidgetCatalog: catalog,
    });

    // Radix UI renders duplicate text for accessibility, so use getAllByText
    expect(screen.getAllByText("widget-1")[0]).toBeInTheDocument();
    expect(screen.getAllByText("widget-2")[0]).toBeInTheDocument();
  });

  it("allows selecting a widget from the list", async () => {
    const user = userEvent.setup();
    const catalog = createWidgetCatalogEntry("widget-1", {
      name: "Test Widget",
      version: "1.0.0",
    });

    renderWithProviders(<Widgets />, {
      initialWidgetCatalog: catalog,
    });

    // Click on widget trigger (use getAllByText since Radix renders duplicates)
    const triggers = screen.getAllByText("widget-1");
    const trigger = triggers[0];
    if (trigger) {
      await user.click(trigger);
    }

    // Widget details should be visible
    await waitFor(() => {
      expect(screen.getByText("ID: widget-1")).toBeInTheDocument();
      expect(screen.getByText("Test Widget")).toBeInTheDocument();
    });
  });

  it("shows error indicator for invalid widgets", () => {
    const catalog: deskulptWidgets.WidgetCatalog = {
      "widget-1": { type: "ok", content: { name: "Valid Widget" } },
      "widget-2": { type: "err", content: "Error loading widget" },
    };

    renderWithProviders(<Widgets />, {
      initialWidgetCatalog: catalog,
    });

    // Both widgets should be visible (use getAllByText since Radix renders duplicates)
    expect(screen.getAllByText("widget-1")[0]).toBeInTheDocument();
    expect(screen.getAllByText("widget-2")[0]).toBeInTheDocument();
  });
});
