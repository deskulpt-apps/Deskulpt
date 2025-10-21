import WidgetContainer from "./components/WidgetContainer";
import { Toaster } from "sonner";
import { Theme as RadixTheme } from "@radix-ui/themes";
import { useShallow } from "zustand/shallow";
import {
  useRenderWidgetsListener,
  useSettingsStore,
  useShowToastListener,
  useUpdateSettingsListener,
  useUpdateWidgetCatalogListener,
  useWidgetsStore,
} from "./hooks";
import { useCallback, useEffect, useRef } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";

const App = () => {
  const theme = useSettingsStore((state) => state.theme);
  const ids = useWidgetsStore(useShallow((state) => Object.keys(state)));

  useRenderWidgetsListener();
  useShowToastListener();
  useUpdateSettingsListener();
  useUpdateWidgetCatalogListener();

  // Tracks current click-through state so we only log on changes
  const isClickThroughRef = useRef<boolean | null>(null);

  const setCanvasClickThrough = useCallback((isClickThrough: boolean) => {
    if (isClickThroughRef.current !== isClickThrough) {
      isClickThroughRef.current = isClickThrough;
      console.debug(
        isClickThrough
          ? "[canvas] click-through: ON"
          : "[canvas] click-through: OFF",
      );
      getCurrentWindow()
        .setIgnoreCursorEvents(isClickThrough)
        .catch(console.error);
    }
  }, []);

  useEffect(() => {
    const onMouseMove = (ev: MouseEvent) => {
      const target = ev.target as HTMLElement | null;
      const insideAWidget = !!target?.closest('[data-widget-container="true"]');
      // Non-click-through when hovering any widget; click-through otherwise
      setCanvasClickThrough(!insideAWidget);
    };

    // When the cursor leaves the window entirely, revert to click-through
    const onMouseOut = (ev: MouseEvent) => {
      if ((ev.relatedTarget as Node | null) === null) {
        setCanvasClickThrough(true);
      }
    };

    // Initialize to click-through until we hover a widget
    setCanvasClickThrough(true);

    window.addEventListener("mousemove", onMouseMove, { passive: true });
    window.addEventListener("mouseout", onMouseOut, { passive: true });

    return () => {
      window.removeEventListener("mousemove", onMouseMove);
      window.removeEventListener("mouseout", onMouseOut);
    };
  }, [setCanvasClickThrough]);

  return (
    <RadixTheme
      appearance={theme}
      accentColor="indigo"
      grayColor="slate"
      hasBackground={false}
    >
      <Toaster
        position="bottom-right"
        gap={6}
        toastOptions={{
          style: {
            color: "var(--gray-12)",
            borderColor: "var(--gray-6)",
            backgroundColor: "var(--gray-2)",
            padding: "var(--space-2) var(--space-4)",
          },
        }}
      />
      {ids.map((id) => (
        <div key={id} data-widget-container="true">
          <WidgetContainer id={id} />
        </div>
      ))}
    </RadixTheme>
  );
};

export default App;
