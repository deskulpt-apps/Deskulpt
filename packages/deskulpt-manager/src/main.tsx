import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { DeepReadonly, enforceOpenNewTab } from "@deskulpt/utils";
import { deskulptCore } from "@deskulpt/bindings";
import { initSentry, installErrorHandlers } from "@deskulpt/observability";
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

// Initialize observability before rendering
const isDevelopment = import.meta.env.DEV;
const settings = window.__DESKULPT_INTERNALS__.initialSettings;

initSentry({
  enabled: settings.enableTelemetry,
  dsn: import.meta.env.VITE_SENTRY_DSN,
  environment: isDevelopment ? "development" : "production",
  release: import.meta.env.VITE_APP_VERSION || "0.1.0",
});

installErrorHandlers({
  captureUnhandledExceptions: true,
  captureUnhandledRejections: true,
  captureConsoleErrors: true,
});

console.debug("[Manager] Observability initialized");

createRoot(document.querySelector("#root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
