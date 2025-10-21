import { create } from "zustand";
import { Settings } from "@deskulpt/bindings";

export const useSettingsStore = create<Settings>(() => ({
  ...window.__DESKULPT_MANAGER_INTERNALS__.initialSettings,
}));
