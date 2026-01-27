import { DeskulptLogs } from "@deskulpt/bindings";
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
    acc[level] = (message: unknown, meta?: Record<string, unknown>) => {
      const payload =
        typeof message === "string" ? meta : { __message: message, ...meta };

      DeskulptLogs.Commands.log(
        level,
        String(message),
        serialize(payload),
      ).catch((error) => {
        console.error("Logger error:", error);
      });
    };
    return acc;
  },
  {} as {
    [L in DeskulptLogs.Level]: (
      message: unknown,
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
