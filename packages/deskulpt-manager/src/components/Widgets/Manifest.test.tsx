import { describe, expect, it } from "vitest";
import { renderWithProviders, screen } from "../../test-utils/test-helpers";
import Manifest from "./Manifest";
import { createWidgetCatalogEntry } from "../../test-utils/test-helpers";

describe("Widget Manifest", () => {
  it("renders manifest information for valid widget", () => {
    const catalog = createWidgetCatalogEntry("widget-1", {
      name: "Test Widget",
      version: "1.0.0",
      license: "MIT",
      description: "A test widget",
      authors: ["Test Author"],
    });

    renderWithProviders(<Manifest id="widget-1" />, {
      initialWidgetCatalog: catalog,
    });

    expect(screen.getByText("Test Widget")).toBeInTheDocument();
    expect(screen.getByText("1.0.0")).toBeInTheDocument();
    expect(screen.getByText("MIT")).toBeInTheDocument();
    expect(screen.getByText("A test widget")).toBeInTheDocument();
    expect(screen.getByText("Test Author")).toBeInTheDocument();
  });

  it("renders error message for invalid widget", () => {
    const catalog = {
      "widget-1": { type: "err", content: "Widget not found" },
    };

    renderWithProviders(<Manifest id="widget-1" />, {
      initialWidgetCatalog: catalog,
    });

    expect(screen.getByText("Widget not found")).toBeInTheDocument();
  });

  it("handles widget with minimal manifest", () => {
    const catalog = createWidgetCatalogEntry("widget-1", {
      name: "Minimal Widget",
    });

    renderWithProviders(<Manifest id="widget-1" />, {
      initialWidgetCatalog: catalog,
    });

    expect(screen.getByText("Minimal Widget")).toBeInTheDocument();
  });

  it("handles multiple authors", () => {
    const catalog = createWidgetCatalogEntry("widget-1", {
      name: "Test Widget",
      authors: ["Author 1", { name: "Author 2", email: "author2@example.com" }],
    });

    renderWithProviders(<Manifest id="widget-1" />, {
      initialWidgetCatalog: catalog,
    });

    expect(screen.getByText(/Author 1.*Author 2/)).toBeInTheDocument();
  });
});
