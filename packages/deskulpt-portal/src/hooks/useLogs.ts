import { DeskulptLogs } from "@deskulpt/bindings";
import { useCallback, useEffect, useRef, useState } from "react";

interface UseLogsProps {
  minLevel: string;
  pageSize: number;
}

export function useLogs({ minLevel, pageSize }: UseLogsProps) {
  const fetchIdRef = useRef(0); // Used for preventing race conditions

  const [entries, setEntries] = useState<DeskulptLogs.Entry[]>([]);
  const [cursor, setCursor] = useState<DeskulptLogs.Cursor | null>(null);
  const [isFetching, setIsFetching] = useState<boolean>(false);

  const fetchLogs = useCallback(
    async (cursor: DeskulptLogs.Cursor | null, replace: boolean) => {
      // Increment ID to invalidate previous fetches; before any state updates,
      // we check if the ID is still current, and if not, we abort because there
      // must have been a newer fetch
      const fetchId = ++fetchIdRef.current;
      setIsFetching(true);

      if (replace) {
        setEntries([]);
        setCursor(null);
      }

      try {
        const page = await DeskulptLogs.Commands.read(
          pageSize,
          minLevel as DeskulptLogs.Level,
          cursor,
        );
        if (fetchId === fetchIdRef.current) {
          setEntries((prev) =>
            replace ? page.entries : [...prev, ...page.entries],
          );
          setCursor(page.cursor);
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
    if (!isFetching && cursor !== null) {
      await fetchLogs(cursor, false);
    }
  }, [isFetching, cursor, fetchLogs]);

  const refresh = useCallback(() => {
    fetchLogs(null, true);
  }, [fetchLogs]);

  useEffect(refresh, [refresh]); // Initial refresh

  return {
    entries,
    hasMore: cursor !== null,
    isFetching,
    fetchMore,
    refresh,
  };
}
