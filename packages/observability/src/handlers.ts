import { addBreadcrumb, captureException, captureMessage } from "./sentry";

export interface ErrorHandlerConfig {
  captureUnhandledExceptions?: boolean;
  captureUnhandledRejections?: boolean;
  captureConsoleErrors?: boolean;
}

/**
 * Install global error handlers for capturing unhandled errors.
 *
 * This should be called early in your application setup to catch all
 * unhandled errors that might not be caught by React error boundaries.
 */
export function installErrorHandlers(config: ErrorHandlerConfig = {}): void {
  const {
    captureUnhandledExceptions = true,
    captureUnhandledRejections = true,
    captureConsoleErrors = true,
  } = config;

  if (captureUnhandledExceptions) {
    window.addEventListener("error", (event) => {
      console.error("[Observability] Unhandled error:", event.error);

      if (event.error instanceof Error) {
        captureException(event.error, {
          type: "window.error",
          filename: event.filename,
          lineno: event.lineno,
          colno: event.colno,
        });
      } else {
        captureMessage(`Unhandled error: ${event.message}`, "error");
      }

      // Don't prevent default to maintain normal error handling
    });
  }

  if (captureUnhandledRejections) {
    window.addEventListener("unhandledrejection", (event) => {
      console.error("[Observability] Unhandled rejection:", event.reason);

      const error =
        event.reason instanceof Error
          ? event.reason
          : new Error(String(event.reason));

      captureException(error, {
        type: "unhandledrejection",
      });

      // Prevent default to avoid uncaught error
      event.preventDefault();
    });
  }

  if (captureConsoleErrors) {
    const originalError = console.error;
    console.error = function (...args: Parameters<typeof originalError>) {
      // Always call original
      originalError.apply(console, args);

      // Extract first argument if it's an Error or string
      const firstArg = args[0];
      if (firstArg instanceof Error) {
        captureException(firstArg, {
          type: "console.error",
          args: args.slice(1),
        });
      } else if (
        typeof firstArg === "string" &&
        firstArg.toLowerCase().includes("error")
      ) {
        const message = args
          .map((arg) => (typeof arg === "string" ? arg : JSON.stringify(arg)))
          .join(" ");

        captureMessage(message, "error");
      }
    };
  }

  console.debug("[Observability] Global error handlers installed");
}

/**
 * Convert a JavaScript Error to a serializable object.
 * Useful for sending error details to backends.
 */
export function serializeError(error: unknown): Record<string, any> {
  if (error instanceof Error) {
    return {
      name: error.name,
      message: error.message,
      stack: error.stack,
    };
  }

  if (typeof error === "string") {
    return {
      message: error,
    };
  }

  return {
    message: String(error),
    raw: error,
  };
}

/**
 * Create an error context object for breadcrumbs.
 */
export function createErrorContext(
  error: unknown,
  additionalContext?: Record<string, any>,
): Record<string, any> {
  return {
    error: serializeError(error),
    ...additionalContext,
    timestamp: new Date().toISOString(),
  };
}

/**
 * Log an error with breadcrumb tracking.
 */
export function logError(
  error: unknown,
  context?: string,
  additionalData?: Record<string, any>,
): void {
  const errorContext = createErrorContext(error, additionalData);

  addBreadcrumb(context || "Error logged", errorContext, "error");

  console.error(`[${context || "Error"}]`, error, additionalData);
}

/**
 * Log a warning with breadcrumb tracking.
 */
export function logWarning(
  message: string,
  context?: string,
  additionalData?: Record<string, any>,
): void {
  addBreadcrumb(message, additionalData, context || "warning");

  console.warn(`[${context || "Warning"}]`, message, additionalData);
}

/**
 * Log an info message with breadcrumb tracking.
 */
export function logInfo(
  message: string,
  context?: string,
  additionalData?: Record<string, any>,
): void {
  addBreadcrumb(message, additionalData, context || "info");

  console.info(`[${context || "Info"}]`, message, additionalData);
}
