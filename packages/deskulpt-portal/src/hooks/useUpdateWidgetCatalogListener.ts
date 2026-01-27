import { DeskulptWidgets } from "@deskulpt/bindings";
import { useWidgetsStore } from "./useWidgetsStore";
import { logger } from "@deskulpt/utils";
import { useEffect } from "react";

export const useUpdateWidgetCatalogListener = () => {
  useEffect(() => {
    const unlisten = DeskulptWidgets.Events.update.listen((event) => {
      useWidgetsStore.setState(() => event.payload, true);
    });

    return () => {
      unlisten.then((f) => f()).catch(logger.error);
    };
  }, []);
};
