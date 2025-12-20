import { ReactElement } from "react";
import { RenderOptions, RenderResult, render } from "@testing-library/react";
import { Theme as RadixTheme } from "@radix-ui/themes";
import { Toaster } from "sonner";
import { useSettingsStore } from "../hooks/useSettingsStore";
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
  hotReloadEnabled: false,
};

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
  // Note: useWidgetsStore in canvas is for rendered widget components, not the catalog
  // The catalog is managed separately via events, so we don't set it here

  return (
    <RadixTheme
      appearance={theme}
      accentColor="indigo"
      grayColor="slate"
      hasBackground={false}
    >
      <Toaster position="bottom-right" theme={theme} />
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
