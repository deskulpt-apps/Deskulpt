import { events } from "@deskulpt/bindings";
import { useSettingsStore } from "./useSettingsStore";
import { createSetupTaskHook } from "@deskulpt/utils";

export const useUpdateSettingsListener = createSetupTaskHook({
  task: `event:${events.updateSettings.name}`,
  onMount: () =>
    events.updateSettings.listen((event) => {
      useSettingsStore.setState(() => event.payload, true);
    }),
  onUnmount: (unlisten) => unlisten.then((f) => f()).catch(console.error),
});
