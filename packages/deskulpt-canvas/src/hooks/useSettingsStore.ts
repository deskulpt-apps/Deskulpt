import { create } from "zustand";
import { deskulptCore } from "@deskulpt/bindings";

export const useSettingsStore = create<deskulptCore.Settings>(() => ({
  ...window.__DESKULPT_INTERNALS__.initialSettings,
}));
