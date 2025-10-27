import React, { ReactNode } from "react";
import { addBreadcrumb, captureException } from "./sentry";

export interface ErrorBoundaryProps {
  children: ReactNode;
  fallback?: ReactNode | ((error: Error) => ReactNode);
  onError?: (error: Error, errorInfo: React.ErrorInfo) => void;
  captureError?: boolean;
  context?: string;
}

interface ErrorBoundaryState {
  hasError: boolean;
  error: Error | null;
}

/**
 * Error boundary component that captures errors and integrates with Sentry.
 *
 * Usage:
 * ```tsx
 * <ErrorBoundary context="MyComponent">
 *   <MyComponent />
 * </ErrorBoundary>
 * ```
 */
export class ErrorBoundary extends React.Component<
  ErrorBoundaryProps,
  ErrorBoundaryState
> {
  constructor(props: ErrorBoundaryProps) {
    super(props);
    this.state = {
      hasError: false,
      error: null,
    };
  }

  static getDerivedStateFromError(error: Error): ErrorBoundaryState {
    return {
      hasError: true,
      error,
    };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo): void {
    const { onError, captureError = true, context } = this.props;

    // Add breadcrumb for error boundary catch
    addBreadcrumb(
      `Error caught by boundary${context ? `: ${context}` : ""}`,
      {
        componentStack: errorInfo.componentStack,
        context,
      },
      "error-boundary",
    );

    // Capture with Sentry
    if (captureError) {
      captureException(error, {
        type: "error-boundary",
        context,
        componentStack: errorInfo.componentStack,
      });
    }

    // Call custom handler if provided
    if (onError) {
      onError(error, errorInfo);
    }

    console.error(
      `[ErrorBoundary${context ? `: ${context}` : ""}]`,
      error,
      errorInfo,
    );
  }

  handleReset = (): void => {
    this.setState({
      hasError: false,
      error: null,
    });
  };

  render(): ReactNode {
    if (this.state.hasError && this.state.error) {
      const { fallback } = this.props;

      // If custom fallback provided, use it
      if (fallback) {
        if (typeof fallback === "function") {
          return (fallback as (error: Error) => ReactNode)(this.state.error);
        }
        return fallback;
      }

      // Default fallback UI
      return (
        <DefaultErrorFallback
          error={this.state.error}
          onReset={this.handleReset}
        />
      );
    }

    return this.props.children;
  }
}

/**
 * Default fallback UI for errors.
 */
interface DefaultErrorFallbackProps {
  error: Error;
  onReset: () => void;
}

function DefaultErrorFallback({
  error,
  onReset,
}: DefaultErrorFallbackProps): ReactNode {
  return (
    <div
      style={{
        padding: "20px",
        margin: "10px",
        border: "1px solid #ff6b6b",
        borderRadius: "4px",
        backgroundColor: "#ffe0e0",
        color: "#c92a2a",
        fontFamily: "system-ui, -apple-system, sans-serif",
      }}
    >
      <h2 style={{ margin: "0 0 10px 0" }}>Something went wrong</h2>
      <details
        style={{
          marginBottom: "10px",
          whiteSpace: "pre-wrap",
          wordBreak: "break-word",
          fontSize: "12px",
          backgroundColor: "rgba(0, 0, 0, 0.05)",
          padding: "10px",
          borderRadius: "3px",
          fontFamily: "monospace",
        }}
      >
        <summary style={{ cursor: "pointer", marginBottom: "10px" }}>
          Error details
        </summary>
        <p style={{ margin: "10px 0 0 0" }}>
          <strong>{error.name}:</strong> {error.message}
          {error.stack && (
            <>
              <br />
              <br />
              <strong>Stack trace:</strong>
              <br />
              {error.stack}
            </>
          )}
        </p>
      </details>
      <button
        onClick={onReset}
        style={{
          padding: "8px 16px",
          backgroundColor: "#c92a2a",
          color: "white",
          border: "none",
          borderRadius: "3px",
          cursor: "pointer",
          fontWeight: "bold",
        }}
      >
        Try again
      </button>
    </div>
  );
}

/**
 * Hook to use error boundary functionality in functional components.
 *
 * Usage:
 * ```tsx
 * const { error, resetError } = useErrorHandler();
 * ```
 */
export function useErrorHandler(): {
  error: Error | null;
  resetError: () => void;
  captureError: (error: Error) => void;
} {
  const [error, setError] = React.useState<Error | null>(null);

  return {
    error,
    resetError: () => setError(null),
    captureError: (err: Error) => {
      captureException(err);
      setError(err);
    },
  };
}
