import { create } from "zustand";
import { DeskulptWidgets } from "@deskulpt/bindings";

export const useWidgetsStore = create<DeskulptWidgets.WidgetCatalog>(
  () => ({}),
);
