import { deskulptWidgets } from "@deskulpt/bindings";
import { useWidgetsStore } from "./useWidgetsStore";
import { createSetupTaskHook } from "@deskulpt/utils";

export const useUpdateWidgetsListener = createSetupTaskHook({
  task: `event:${deskulptWidgets.events.update.name}`,
  onMount: () =>
    deskulptWidgets.events.update.listen((event) => {
      useWidgetsStore.setState(() => event.payload, true);
    }),
  onUnmount: (unlisten) => unlisten.then((f) => f()).catch(console.error),
});
