import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import {
  DeepReadonly,
  enforceOpenNewTab,
  setupGlobalLoggingHooks,
} from "@deskulpt/utils";
import { deskulptSettings } from "@deskulpt/bindings";
import App from "./App";
import "@radix-ui/themes/styles.css";
import "./custom.css";

declare global {
  interface Window {
    readonly __DESKULPT_INTERNALS__: {
      readonly apisWrapper: string;
      readonly initialSettings: DeepReadonly<deskulptSettings.Settings>;
    };
  }
}

enforceOpenNewTab();
setupGlobalLoggingHooks();

createRoot(document.querySelector("#root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
