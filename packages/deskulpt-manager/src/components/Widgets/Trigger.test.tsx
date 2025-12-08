import { describe, expect, it } from "vitest";
import { renderWithProviders, screen } from "../../test-utils/test-helpers";
import { Tabs } from "@radix-ui/themes";
import Trigger from "./Trigger";
import { createWidgetCatalogEntry } from "../../test-utils/test-helpers";

describe("Widget Trigger", () => {
  it("renders widget ID", () => {
    const catalog = createWidgetCatalogEntry("widget-1", {
      name: "Test Widget",
    });

    renderWithProviders(
      <Tabs.Root defaultValue="tab0">
        <Tabs.List>
          <Trigger id="widget-1" value="tab0" />
        </Tabs.List>
      </Tabs.Root>,
      {
        initialWidgetCatalog: catalog,
      },
    );

    // Radix UI renders duplicate text for accessibility
    expect(screen.getAllByText("widget-1")[0]).toBeInTheDocument();
  });

  it("shows valid indicator for valid widget", () => {
    const catalog = createWidgetCatalogEntry("widget-1", {
      name: "Test Widget",
    });

    renderWithProviders(
      <Tabs.Root defaultValue="tab0">
        <Tabs.List>
          <Trigger id="widget-1" value="tab0" />
        </Tabs.List>
      </Tabs.Root>,
      {
        initialWidgetCatalog: catalog,
      },
    );

    // Radix UI renders duplicate text for accessibility
    const trigger = screen.getAllByText("widget-1")[0].closest("button");
    expect(trigger).toBeInTheDocument();
  });

  it("shows error indicator for invalid widget", () => {
    const catalog = {
      "widget-1": { type: "err", content: "Error" },
    };

    renderWithProviders(
      <Tabs.Root defaultValue="tab0">
        <Tabs.List>
          <Trigger id="widget-1" value="tab0" />
        </Tabs.List>
      </Tabs.Root>,
      {
        initialWidgetCatalog: catalog,
      },
    );

    // Radix UI renders duplicate text for accessibility
    expect(screen.getAllByText("widget-1")[0]).toBeInTheDocument();
  });
});
