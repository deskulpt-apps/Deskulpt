import { create } from "zustand";
import { deskulptCore } from "@deskulpt/bindings";

export const useWidgetsStore = create<deskulptCore.WidgetCatalog>(() => ({}));
