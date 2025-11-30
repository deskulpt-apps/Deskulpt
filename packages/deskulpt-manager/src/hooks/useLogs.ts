import { deskulptCore } from "@deskulpt/bindings";
import { useCallback, useEffect, useRef, useState } from "react";

interface UseLogsProps {
  minLevel: string;
  pageSize: number;
}

export function useLogs({ minLevel, pageSize }: UseLogsProps) {
  const fetchIdRef = useRef(0); // Used for preventing race conditions

  const [entries, setEntries] = useState<deskulptCore.LogEntry[]>([]);
  const [cursor, setCursor] = useState<deskulptCore.LogCursor | null>(null);
  const [hasMore, setHasMore] = useState<boolean>(false);
  const [isFetching, setIsFetching] = useState<boolean>(false);

  const fetchLogs = useCallback(
    async (cursor: deskulptCore.LogCursor | null, replace: boolean) => {
      // Increment ID to invalidate previous fetches; before any state updates,
      // we check if the ID is still current, and if not, we abort because there
      // must have been a newer fetch
      const fetchId = ++fetchIdRef.current;
      setIsFetching(true);

      if (replace) {
        setEntries([]);
        setCursor(null);
        setHasMore(false);
      }

      try {
        const page = await deskulptCore.commands.fetchLogs(
          pageSize,
          cursor,
          minLevel as deskulptCore.LoggingLevel,
        );
        if (fetchId === fetchIdRef.current) {
          setEntries((prev) =>
            replace ? page.entries : [...prev, ...page.entries],
          );
          setCursor(page.cursor);
          setHasMore(page.hasMore);
        }
      } finally {
        if (fetchId === fetchIdRef.current) {
          setIsFetching(false);
        }
      }
    },
    [minLevel, pageSize],
  );

  const fetchMore = useCallback(async () => {
    if (!isFetching && hasMore && cursor !== null) {
      await fetchLogs(cursor, false);
    }
  }, [isFetching, hasMore, cursor, fetchLogs]);

  const refresh = useCallback(() => {
    fetchLogs(null, true);
  }, [fetchLogs]);

  useEffect(refresh, [refresh]); // Initial refresh

  return {
    entries,
    hasMore,
    isFetching,
    fetchMore,
    refresh,
  };
}
