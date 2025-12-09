import { beforeEach, describe, expect, it, vi } from "vitest";
import { act, renderHook, waitFor } from "@testing-library/react";
import { mockInvoke } from "../test-utils/tauri-mocks";
import { useLogs } from "./useLogs";

describe("useLogs", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("fetches logs on mount", async () => {
    const mockEntries = [
      {
        timestamp: "2024-01-01T00:00:00Z",
        level: "INFO",
        message: "Test log",
        raw: {},
      },
    ];

    mockInvoke.register("plugin:deskulpt-core|fetch_logs", () => ({
      entries: mockEntries,
      cursor: null,
      hasMore: false,
    }));

    const { result } = renderHook(() =>
      useLogs({ minLevel: "info", pageSize: 100 }),
    );

    await waitFor(() => {
      expect(result.current.entries).toHaveLength(1);
      expect(result.current.entries[0]?.message).toBe("Test log");
    });
  });

  it("handles pagination with cursor", async () => {
    const firstPage = [
      {
        timestamp: "2024-01-01T00:00:00Z",
        level: "INFO",
        message: "Log 1",
        raw: {},
      },
    ];

    const secondPage = [
      {
        timestamp: "2024-01-01T00:01:00Z",
        level: "INFO",
        message: "Log 2",
        raw: {},
      },
    ];

    let callCount = 0;
    mockInvoke.register("plugin:deskulpt-core|fetch_logs", () => {
      callCount++;
      if (callCount === 1) {
        return {
          entries: firstPage,
          cursor: { path: "/logs/app.log", offset: 100 },
          hasMore: true,
        };
      }
      return {
        entries: secondPage,
        cursor: null,
        hasMore: false,
      };
    });

    const { result } = renderHook(() =>
      useLogs({ minLevel: "info", pageSize: 100 }),
    );

    await waitFor(() => {
      expect(result.current.entries).toHaveLength(1);
    });

    // Fetch more
    act(() => {
      result.current.fetchMore();
    });

    await waitFor(() => {
      expect(result.current.entries).toHaveLength(2);
    });
  });

  it("refreshes logs", async () => {
    const mockEntries = [
      {
        timestamp: "2024-01-01T00:00:00Z",
        level: "INFO",
        message: "Test log",
        raw: {},
      },
    ];

    mockInvoke.register("plugin:deskulpt-core|fetch_logs", () => ({
      entries: mockEntries,
      cursor: null,
      hasMore: false,
    }));

    const { result } = renderHook(() =>
      useLogs({ minLevel: "info", pageSize: 100 }),
    );

    await waitFor(() => {
      expect(result.current.entries).toHaveLength(1);
    });

    // Refresh
    act(() => {
      result.current.refresh();
    });

    await waitFor(() => {
      expect(result.current.isFetching).toBe(false);
    });
  });
});
