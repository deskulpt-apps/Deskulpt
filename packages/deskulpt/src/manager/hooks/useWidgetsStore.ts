import { create } from "zustand";
import { WidgetCatalog } from "../../bindings";

export const useWidgetsStore = create<WidgetCatalog>(() => ({}));
