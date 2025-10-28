import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { DeepReadonly, enforceOpenNewTab } from "@deskulpt/utils";
import { deskulptCore } from "@deskulpt/bindings";
import App from "./App";
import "@radix-ui/themes/styles.css";
import "./custom.css";

declare global {
  interface Window {
    readonly __DESKULPT_INTERNALS__: {
      readonly initialSettings: DeepReadonly<deskulptCore.Settings>;
    };
  }
}

enforceOpenNewTab();

createRoot(document.querySelector("#root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
