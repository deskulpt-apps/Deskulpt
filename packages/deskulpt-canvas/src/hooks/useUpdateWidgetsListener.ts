import { DeskulptWidgets } from "@deskulpt/bindings";
import { useWidgetsStore } from "./useWidgetsStore";
import { logger } from "@deskulpt/utils";
import { useEffect } from "react";

export const useUpdateWidgetsListener = () => {
  useEffect(() => {
    const unlisten = DeskulptWidgets.Events.update.listen((event) => {
      const widgets = Object.entries(useWidgetsStore.getState());

      // Clean up widgets that are no longer in the catalog
      const remainingWidgets = widgets.filter(
        ([id, { apisBlobUrl, moduleBlobUrl }]) => {
          if (id in event.payload) {
            return true;
          }
          URL.revokeObjectURL(apisBlobUrl);
          if (moduleBlobUrl !== undefined) {
            URL.revokeObjectURL(moduleBlobUrl);
          }
          return false;
        },
      );

      // Update the settings of existing widgets
      for (const [id, widget] of remainingWidgets) {
        const updatedWidget = event.payload[id];
        if (updatedWidget !== undefined) {
          widget.settings = updatedWidget.settings;
        }
      }

      // Update the store only if there are changes (length match means no
      // removals thus no changes in this case)
      if (remainingWidgets.length !== widgets.length) {
        useWidgetsStore.setState(
          () => Object.fromEntries(remainingWidgets),
          true,
        );
      }
    });

    return () => {
      unlisten.then((f) => f()).catch(logger.error);
    };
  }, []);
};
