import { useEffect } from "react";
import { events } from "@deskulpt/bindings";
import { useSettingsStore } from "./useSettingsStore";

export function useUpdateSettingsListener() {
  useEffect(() => {
    const unlisten = events.updateSettings.listen((event) => {
      useSettingsStore.setState(() => event.payload, true);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
