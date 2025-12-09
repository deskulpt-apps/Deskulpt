import { ReactElement } from "react";
import { RenderOptions, RenderResult, render } from "@testing-library/react";
import { Theme as RadixTheme } from "@radix-ui/themes";
import { Toaster } from "sonner";
import { useSettingsStore } from "../hooks/useSettingsStore";
import { useWidgetsStore } from "../hooks/useWidgetsStore";
import type { deskulptSettings } from "@deskulpt/bindings";
import type { deskulptWidgets } from "@deskulpt/bindings";

/**
 * Default test settings.
 */
const defaultSettings: deskulptSettings.Settings = {
  theme: "light",
  canvasImode: "auto",
  shortcuts: {},
  widgets: {},
};

/**
 * Helper to create a valid widget catalog entry.
 */
export function createWidgetCatalogEntry(
  id: string,
  manifest?: deskulptWidgets.WidgetManifest,
): deskulptWidgets.WidgetCatalog {
  return {
    [id]: manifest
      ? { type: "ok", content: manifest }
      : { type: "err", content: "Widget not found" },
  };
}

/**
 * Options for rendering components in tests.
 */
interface TestRenderOptions extends Omit<RenderOptions, "wrapper"> {
  /**
   * Initial settings state.
   */
  initialSettings?: Partial<deskulptSettings.Settings>;
  /**
   * Initial widget catalog state.
   */
  initialWidgetCatalog?: deskulptWidgets.WidgetCatalog;
  /**
   * Theme appearance.
   */
  theme?: "light" | "dark";
}

/**
 * Test wrapper component that provides all necessary context.
 */
function TestWrapper({
  children,
  initialSettings = {},
  initialWidgetCatalog = {},
  theme = "light",
}: {
  children: React.ReactNode;
  initialSettings?: Partial<deskulptSettings.Settings>;
  initialWidgetCatalog?: deskulptWidgets.WidgetCatalog;
  theme?: "light" | "dark";
}) {
  // Initialize stores with test data
  useSettingsStore.setState({
    ...defaultSettings,
    ...initialSettings,
    theme: theme || initialSettings.theme || defaultSettings.theme,
  });
  useWidgetsStore.setState(initialWidgetCatalog);

  return (
    <RadixTheme appearance={theme} accentColor="indigo" grayColor="slate">
      <Toaster position="bottom-center" theme={theme} />
      {children}
    </RadixTheme>
  );
}

/**
 * Custom render function that includes all necessary providers.
 */
export function renderWithProviders(
  ui: ReactElement,
  options: TestRenderOptions = {},
): RenderResult {
  const { initialSettings, initialWidgetCatalog, theme, ...renderOptions } =
    options;

  return render(ui, {
    wrapper: ({ children }) => (
      <TestWrapper
        initialSettings={initialSettings}
        initialWidgetCatalog={initialWidgetCatalog}
        theme={theme}
      >
        {children}
      </TestWrapper>
    ),
    ...renderOptions,
  });
}

/**
 * Re-export everything from React Testing Library.
 */
export * from "@testing-library/react";
export { default as userEvent } from "@testing-library/user-event";
