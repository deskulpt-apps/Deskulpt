import { create } from "zustand";
import { deskulptWidgets } from "@deskulpt/bindings";

export const useWidgetsStore = create<deskulptWidgets.WidgetCatalog>(
  () => ({}),
);
