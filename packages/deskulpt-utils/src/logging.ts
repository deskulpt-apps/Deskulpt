import { deskulptCore } from "@deskulpt/bindings";
import { serialize } from "./serialize";

export const LOGGING_LEVELS = [
  "trace",
  "debug",
  "info",
  "warn",
  "error",
] as const;

export const logger = LOGGING_LEVELS.reduce(
  (acc, level) => {
    acc[level] = (message: string, meta?: Record<string, unknown>) => {
      // Declaring message as string does not prevent `any` from being passed
      // here; in that case, we don't want to lose information by directly
      // casting to string type, so we include in the payload for more robust
      // serialization
      const payload =
        typeof message === "string" ? meta : { __message: message, ...meta };

      deskulptCore.commands
        .log(level, String(message), serialize(payload))
        .catch((error) => {
          console.error("Logger error:", error);
        });
    };
    return acc;
  },
  {} as {
    [L in deskulptCore.LoggingLevel]: (
      message: string,
      meta?: Record<string, unknown>,
    ) => void;
  },
);

export function setupGlobalLoggingHooks() {
  window.addEventListener("error", (event) => {
    logger.error(event.message, { type: "uncaught-error", error: event.error });
  });

  window.addEventListener("unhandledrejection", (event) => {
    logger.error(event.reason, { type: "unhandled-rejection" });
  });
}
