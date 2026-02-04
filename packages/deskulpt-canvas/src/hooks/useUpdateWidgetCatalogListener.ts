import { DeskulptWidgets } from "@deskulpt/bindings";
import { useWidgetsStore } from "./useWidgetsStore";
import { logger } from "@deskulpt/utils";
import { useEffect } from "react";

export const useUpdateWidgetCatalogListener = () => {
  useEffect(() => {
    const unlisten = DeskulptWidgets.Events.update.listen(async (event) => {
      const widgets = useWidgetsStore.getState();

      const newWidgets = Object.fromEntries(
        Object.entries(event.payload).map(([id, { settings }]) => {
          return [id, { ...widgets[id], settings }] as const;
        }),
      );

      useWidgetsStore.setState(() => newWidgets, true);

      // Clean up widgets that no longer exist
      for (const [id, widget] of Object.entries(widgets)) {
        if (id in event.payload) {
          continue;
        }
        if (widget.apisBlobUrl !== undefined) {
          URL.revokeObjectURL(widget.apisBlobUrl);
        }
        if (widget.moduleBlobUrl !== undefined) {
          URL.revokeObjectURL(widget.moduleBlobUrl);
        }
      }
    });

    return () => {
      unlisten.then((f) => f()).catch(logger.error);
    };
  }, []);
};
