import { deskulptSettings } from "@deskulpt/bindings";
import { useSettingsStore } from "./useSettingsStore";
import { createSetupTaskHook, logger } from "@deskulpt/utils";

export const useUpdateSettingsListener = createSetupTaskHook({
  task: `event:${deskulptSettings.events.update.name}`,
  onMount: () =>
    deskulptSettings.events.update.listen((event) => {
      useSettingsStore.setState(() => event.payload, true);
    }),
  onUnmount: (unlisten) => unlisten.then((f) => f()).catch(logger.error),
});
