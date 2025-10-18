import { useEffect } from "react";
import { events } from "../../bindings";
import { useWidgetsStore } from "./useWidgetsStore";

export function useUpdateWidgetCatalogListener() {
  useEffect(() => {
    const unlisten = events.updateWidgetCatalog.listen((event) => {
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
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
