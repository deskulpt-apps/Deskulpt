import { useEffect } from "react";
import { commands } from "@deskulpt/bindings";

export function useInitialRescan() {
  useEffect(() => {
    commands.core.rescanWidgets().catch(console.error);
  }, []);
}
