import { DeskulptSettings } from "@deskulpt/bindings";
import { useSettingsStore } from "./useSettingsStore";
import { logger } from "@deskulpt/utils";
import { useEffect } from "react";

export const useUpdateSettingsListener = () => {
  useEffect(() => {
    const unlisten = DeskulptSettings.Events.update.listen((event) => {
      useSettingsStore.setState(() => event.payload, true);
    });

    return () => {
      unlisten.then((f) => f()).catch(logger.error);
    };
  }, []);
};
