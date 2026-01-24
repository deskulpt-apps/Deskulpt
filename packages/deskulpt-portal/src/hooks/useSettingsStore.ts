import { create } from "zustand";
import { deskulptSettings } from "@deskulpt/bindings";

export const useSettingsStore = create<deskulptSettings.Settings>(() => ({
  ...window.__DESKULPT_INTERNALS__.initialSettings,
}));
