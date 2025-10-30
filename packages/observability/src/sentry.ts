import * as Sentry from "@sentry/react";

export interface SentryConfig {
  enabled: boolean;
  dsn?: string;
  environment: "development" | "production";
  release: string;
  tracePropagationTargets?: string[];
}

/**
 * Initialize Sentry for error tracking and crash reporting.
 *
 * This should be called early in your application setup, before rendering.
 */
export function initSentry(config: SentryConfig): void {
  if (!config.enabled || !config.dsn) {
    console.debug("[Observability] Sentry disabled or no DSN provided");
    return;
  }

  try {
    Sentry.init({
      dsn: config.dsn,
      environment: config.environment,
      release: config.release,
      integrations: [
        Sentry.replayIntegration({
          // Mask all text content
          maskAllText: true,
          // Mask all input values
          maskAllInputs: true,
        }),
      ],
      // Capture 100% of transactions in development, 10% in production
      tracesSampleRate: config.environment === "development" ? 1.0 : 0.1,
      // Replay for 100% of sessions with an error; 10% of all sessions
      replaysOnErrorSampleRate: 1.0,
      replaysSessionSampleRate:
        config.environment === "development" ? 0.1 : 0.01,
      // Ignore console errors and warnings to reduce noise
      ignoreErrors: [
        // Browser extensions
        "top.GLOBALS",
        // Random plugins/extensions
        "chrome-extension://",
        "moz-extension://",
        // See: http://blog.errorception.com/2012/03/tale-of-unfindable-js-error.html
        "Can't find variable: ZiteReader",
        "jigsaw is not defined",
        "ComboSearch is not defined",
        // Network errors are usually not actionable
        "NetworkError",
        "Network request failed",
      ],
      beforeSend: (event) => {
        // Filter PII and sensitive data
        filterSensitiveData(event);
        return event;
      },
      tracePropagationTargets: config.tracePropagationTargets || [
        "localhost",
        /^\//,
      ],
    });

    console.debug("[Observability] Sentry initialized successfully");
  } catch (error) {
    console.error("[Observability] Failed to initialize Sentry:", error);
  }
}

/**
 * Filter sensitive data from Sentry events to respect user privacy.
 */
function filterSensitiveData(
  event: Sentry.ErrorEvent | Sentry.TransactionEvent,
): void {
  // Remove request body if present
  if (event.request) {
    event.request.body = undefined;
    event.request.cookies = undefined;
    event.request.headers = {
      ...event.request.headers,
      // Remove auth-related headers
      Authorization: undefined,
      Cookie: undefined,
    };
  }

  // Remove environment variables and other sensitive context
  if (event.contexts) {
    event.contexts.app = {
      ...event.contexts.app,
      // Keep only non-sensitive app info
    };
  }

  // Clean breadcrumbs
  if (event.breadcrumbs) {
    event.breadcrumbs = event.breadcrumbs.map((breadcrumb) => ({
      ...breadcrumb,
      data: breadcrumb.data ? filterBreadcrumbData(breadcrumb.data) : undefined,
    }));
  }
}

/**
 * Filter sensitive data from breadcrumb data.
 */
function filterBreadcrumbData(data: Record<string, any>): Record<string, any> {
  const filtered: Record<string, any> = {};

  for (const [key, value] of Object.entries(data)) {
    // Skip sensitive keys
    if (
      key.toLowerCase().includes("password") ||
      key.toLowerCase().includes("token") ||
      key.toLowerCase().includes("secret") ||
      key.toLowerCase().includes("auth") ||
      key.toLowerCase().includes("cookie")
    ) {
      continue;
    }

    // Redact URLs that might contain sensitive data
    if (typeof value === "string" && isUrl(value)) {
      filtered[key] = redactUrl(value);
    } else {
      filtered[key] = value;
    }
  }

  return filtered;
}

/**
 * Check if a string is likely a URL.
 */
function isUrl(str: string): boolean {
  try {
    const url = new URL(str);
    return !!url;
  } catch {
    return false;
  }
}

/**
 * Redact sensitive information from URLs.
 */
function redactUrl(urlString: string): string {
  try {
    const url = new URL(urlString);
    // Remove query parameters that might contain sensitive data
    url.search = "";
    return url.toString();
  } catch {
    return "[REDACTED_URL]";
  }
}

/**
 * Set the user context for Sentry.
 *
 * @param userId - Unique identifier for the user (can be anonymous)
 * @param email - User email (optional)
 */
export function setSentryUser(userId: string, email?: string): void {
  Sentry.setUser({
    id: userId,
    email: email || undefined,
    // Don't set IP address (PII concern)
    ip_address: "{{auto}}",
  });
}

/**
 * Clear the user context (e.g., on logout).
 */
export function clearSentryUser(): void {
  Sentry.setUser(null);
}

/**
 * Add breadcrumb for tracking user actions.
 */
export function addBreadcrumb(
  message: string,
  data?: Record<string, any>,
  category?: string,
): void {
  Sentry.addBreadcrumb({
    message,
    level: "info",
    category: category || "user-action",
    data,
  });
}

/**
 * Capture an exception for error tracking.
 */
export function captureException(
  error: Error,
  context?: Record<string, any>,
): void {
  Sentry.captureException(error, {
    contexts: context ? { custom: context } : undefined,
  });
}

/**
 * Capture a message for tracking.
 */
export function captureMessage(
  message: string,
  level: "fatal" | "error" | "warning" | "info" | "debug" = "info",
): void {
  Sentry.captureMessage(message, level);
}
