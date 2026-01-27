import { DeskulptWidgets } from "@deskulpt/bindings";
import { logger } from "@deskulpt/utils";
import { useEffect } from "react";

export const useInitialRefresh = () => {
  useEffect(() => {
    DeskulptWidgets.Commands.refreshAll().catch(logger.error);
  }, []);
};
