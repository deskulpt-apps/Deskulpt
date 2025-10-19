import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { DeepReadonly } from "../types";
import { Settings } from "@deskulpt/bindings";
import { enforceOpenNewTab } from "../utils/enforceOpenNewTab";
import App from "./App";
import "@radix-ui/themes/styles.css";

declare global {
  interface Window {
    readonly __DESKULPT_CANVAS_INTERNALS__: {
      readonly apisWrapper: string;
      readonly initialSettings: DeepReadonly<Settings>;
    };
  }
}

enforceOpenNewTab();

createRoot(document.querySelector("#root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
