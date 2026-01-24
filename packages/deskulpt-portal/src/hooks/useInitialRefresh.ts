import { deskulptWidgets } from "@deskulpt/bindings";
import { logger } from "@deskulpt/utils";
import { useEffect } from "react";

export const useInitialRefresh = () => {
  useEffect(() => {
    deskulptWidgets.commands.refreshAll().catch(logger.error);
  }, []);
};
