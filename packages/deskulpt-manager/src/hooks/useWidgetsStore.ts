import { create } from "zustand";
import { WidgetCatalog } from "@deskulpt/bindings";

export const useWidgetsStore = create<WidgetCatalog>(() => ({}));
