import { deskulptCore } from "@deskulpt/bindings";
import { useWidgetsStore } from "./useWidgetsStore";
import { createSetupTaskHook } from "@deskulpt/utils";

export const useUpdateWidgetCatalogListener = createSetupTaskHook({
  task: `event:${deskulptCore.events.updateWidgetCatalog.name}`,
  onMount: () =>
    deskulptCore.events.updateWidgetCatalog.listen((event) => {
      useWidgetsStore.setState(() => event.payload, true);
    }),
  onUnmount: (unlisten) => unlisten.then((f) => f()).catch(console.error),
});
