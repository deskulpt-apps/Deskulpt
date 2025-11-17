import { deskulptCore } from "@deskulpt/bindings";
import { stringifyError } from "./stringifyError";

type LogLevel = "trace" | "debug" | "info" | "warn" | "error";
type LogCommand = (
  level: LogLevel,
  message: string,
  fields?: Record<string, unknown> | null,
) => Promise<void>;

let cachedCommand: LogCommand | null | undefined;

function getLogCommand(): LogCommand | null {
  if (cachedCommand !== undefined) {
    return cachedCommand;
  }
  const command = (deskulptCore.commands as any).log;
  cachedCommand = typeof command === "function" ? command : null;
  return cachedCommand ?? null;
}

export function logDiagnosticsEvent(
  level: LogLevel,
  message: string,
  fields?: Record<string, unknown>,
) {
  const command = getLogCommand();
  if (!command) {
    return Promise.resolve();
  }
  return command(level, message, fields ?? null);
}

export function setupDiagnosticsLogging(source: string) {
  if (typeof window === "undefined") {
    return;
  }
  const command = getLogCommand();
  if (!command) {
    return;
  }

  const globalWindow = window as typeof window & {
    __deskulptDiagnosticsInstalled__?: boolean;
  };
  if (globalWindow.__deskulptDiagnosticsInstalled__) {
    return;
  }
  globalWindow.__deskulptDiagnosticsInstalled__ = true;

  const emit = (
    level: LogLevel,
    message: string,
    extra?: Record<string, unknown>,
  ) => {
    void command(level, message, { source, ...extra }).catch(() => {});
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

  const overrideConsole = (method: keyof Console, level: LogLevel) => {
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
