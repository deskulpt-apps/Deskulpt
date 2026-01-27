import { create } from "zustand";
import { DeskulptSettings } from "@deskulpt/bindings";

export const useSettingsStore = create<DeskulptSettings.Settings>(() => ({
  ...window.__DESKULPT_INTERNALS__.initialSettings,
}));
