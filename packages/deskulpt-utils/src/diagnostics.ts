import { deskulptCore } from "@deskulpt/bindings";
import { stringifyError } from "./stringifyError";
import { LoggingLevel } from "@deskulpt/bindings/src/deskulpt-core";

export function logDiagnosticsEvent(
  level: deskulptCore.LoggingLevel,
  message: string,
  fields?: deskulptCore.JsonValue,
) {
  return deskulptCore.commands.log(level, message, fields ?? null);
}

export function setupDiagnosticsLogging(source: string) {
  const globalWindow = window as typeof window & {
    __deskulptDiagnosticsInstalled__?: boolean;
  };
  if (globalWindow.__deskulptDiagnosticsInstalled__) {
    return;
  }
  globalWindow.__deskulptDiagnosticsInstalled__ = true;

  const emit = (
    level: LoggingLevel,
    message: string,
    extra?: Record<string, unknown>,
  ) => {
    void deskulptCore.commands
      .log(level, message, { source, ...extra })
      .catch(() => {});
  };

  const formatArg = (arg: unknown) => {
    if (typeof arg === "string") {
      return arg;
    }
    if (arg instanceof Error) {
      return stringifyError(arg);
    }
    if (typeof arg === "object" && arg !== null) {
      try {
        return JSON.stringify(arg);
      } catch {
        return String(arg);
      }
    }
    return String(arg);
  };

  const overrideConsole = (
    method: keyof Console,
    level: deskulptCore.LoggingLevel,
  ) => {
    const original = console[method].bind(console) as (
      ...args: unknown[]
    ) => void;
    (console as any)[method] = ((...args: unknown[]) => {
      original(...args);
      const message =
        args.length === 0
          ? `[console.${method}]`
          : args.map((arg) => formatArg(arg)).join(" ");
      emit(level, message, { consoleMethod: method });
    }) as any;
  };

  (
    [
      ["error", "error"],
      ["warn", "warn"],
      ["info", "info"],
      ["log", "info"],
      ["debug", "debug"],
    ] as const
  ).forEach(([method, level]) => overrideConsole(method, level));

  window.addEventListener("error", (event: ErrorEvent) => {
    const message =
      event.message || stringifyError(event.error ?? "Unknown error");
    emit("error", message, {
      type: "error",
      filename: event.filename,
      lineno: event.lineno,
      colno: event.colno,
      stack: event.error instanceof Error ? event.error.stack : undefined,
    });
  });

  window.addEventListener(
    "unhandledrejection",
    (event: PromiseRejectionEvent) => {
      emit("error", stringifyError(event.reason), {
        type: "unhandledrejection",
        reason:
          event.reason instanceof Error
            ? (event.reason.stack ?? event.reason.message)
            : event.reason,
      });
    },
  );
}
