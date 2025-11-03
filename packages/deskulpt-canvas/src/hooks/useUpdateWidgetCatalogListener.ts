import { deskulptWidgets } from "@deskulpt/bindings";
import { useWidgetsStore } from "./useWidgetsStore";
import { createSetupTaskHook } from "@deskulpt/utils";

export const useUpdateWidgetCatalogListener = createSetupTaskHook({
  task: `event:${deskulptWidgets.events.update.name}`,
  onMount: () =>
    deskulptWidgets.events.update.listen((event) => {
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
    }),
  onUnmount: (unlisten) => unlisten.then((f) => f()).catch(console.error),
});
