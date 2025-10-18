import { useEffect } from "react";
import { events } from "../../bindings";
import { useWidgetsStore } from "./useWidgetsStore";

export function useUpdateWidgetCatalogListener() {
  useEffect(() => {
    const unlisten = events.updateWidgetCatalog.listen((event) => {
      useWidgetsStore.setState(() => event.payload, true);
    });

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
