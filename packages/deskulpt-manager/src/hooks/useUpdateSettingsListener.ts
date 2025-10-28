import { deskulptCore } from "@deskulpt/bindings";
import { useSettingsStore } from "./useSettingsStore";
import { createSetupTaskHook } from "@deskulpt/utils";

export const useUpdateSettingsListener = createSetupTaskHook({
  task: `event:${deskulptCore.events.updateSettings.name}`,
  onMount: () =>
    deskulptCore.events.updateSettings.listen((event) => {
      useSettingsStore.setState(() => event.payload, true);
    }),
  onUnmount: (unlisten) => unlisten.then((f) => f()).catch(console.error),
});
