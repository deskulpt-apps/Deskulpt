import {
  Box,
  Button,
  Code,
  CodeProps,
  Flex,
  ScrollArea,
  Select,
  Spinner,
  Text,
} from "@radix-ui/themes";
import { memo, useCallback, useEffect, useRef, useState } from "react";
import { LuFolderOpen, LuRepeat } from "react-icons/lu";
import { MdDeleteOutline } from "react-icons/md";
import { deskulptCore } from "@deskulpt/bindings";
import { logger } from "@deskulpt/utils";
import { useVirtualizer } from "@tanstack/react-virtual";

const PAGE_SIZE = 50;

const LOGGING_LEVELS = ["trace", "debug", "info", "warn", "error"] as const;
const DEFAULT_MIN_LEVEL = "info";

function formatTimestamp(timestamp: string) {
  const date = new Date(timestamp);

  const dateString = date.toLocaleDateString(undefined, {
    month: "2-digit",
    day: "2-digit",
  });
  const timeString = date.toLocaleTimeString(undefined, {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });

  return `${dateString} ${timeString}`;
}

const Logs = memo(() => {
  const parentRef = useRef<HTMLDivElement>(null);
  const [isFetching, setIsFetching] = useState<boolean>(false);
  const [minLevelFilter, setMinLevelFilter] = useState(DEFAULT_MIN_LEVEL);

  const [entries, setEntries] = useState<deskulptCore.LogEntry[]>([]);
  const [cursor, setCursor] = useState<deskulptCore.LogCursor | null>(null);
  const [hasMore, setHasMore] = useState<boolean>(false);

  // Used for triggering refresh, the value has no specific meaning
  const [refreshIndex, setRefreshIndex] = useState(0);

  useEffect(() => {
    let active = true; // Prevent race conditions

    const fetchInitial = async () => {
      setIsFetching(true);
      setEntries([]);
      setCursor(null);
      setHasMore(false);

      try {
        const page = await deskulptCore.commands.fetchLogs(
          PAGE_SIZE,
          null,
          minLevelFilter as deskulptCore.LoggingLevel,
        );
        if (active) {
          setEntries(page.entries);
          setCursor(page.cursor);
          setHasMore(page.hasMore);
        }
      } finally {
        if (active) {
          setIsFetching(false);
        }
      }
    };

    fetchInitial();
    return () => {
      active = false;
    };
  }, [minLevelFilter, refreshIndex]);

  const fetchMore = useCallback(async () => {
    if (!hasMore || isFetching || cursor === null) {
      return;
    }

    setIsFetching(true);
    try {
      // await new Promise((resolve) => setTimeout(resolve, 1000));
      const page = await deskulptCore.commands.fetchLogs(
        PAGE_SIZE,
        cursor,
        minLevelFilter as deskulptCore.LoggingLevel,
      );
      setEntries((prev) => [...prev, ...page.entries]);
      setCursor(page.cursor ?? null);
      setHasMore(page.hasMore);
    } finally {
      setIsFetching(false);
    }
  }, [cursor, hasMore, isFetching, minLevelFilter]);

  const rowVirtualizer = useVirtualizer({
    count: entries.length + (isFetching ? 1 : 0),
    getScrollElement: () => parentRef.current,
    estimateSize: () => 28,
    overscan: 10,
  });

  const virtualItems = rowVirtualizer.getVirtualItems();

  useEffect(() => {
    if (virtualItems.length === 0) {
      return;
    }
    const lastItem = virtualItems.at(-1)!;
    if (lastItem.index >= entries.length - 1 && hasMore && !isFetching) {
      fetchMore();
    }
  }, [virtualItems, entries.length, fetchMore, hasMore, isFetching]);

  const openLogsDir = useCallback(() => {
    deskulptCore.commands.open("logs").catch(logger.error);
  }, []);

  const refreshLogs = useCallback(() => {
    setRefreshIndex((prev) => prev + 1);
  }, []);

  const clearLogs = useCallback(() => {
    deskulptCore.commands.clearLogs().then(refreshLogs);
  }, [refreshLogs]);

  return (
    <Flex height="100%" direction="column" px="1" gap="3">
      <Flex align="center" gap="2" justify="between">
        <Select.Root
          size="1"
          value={minLevelFilter}
          onValueChange={setMinLevelFilter}
        >
          <Select.Trigger
            css={{ width: "100px", textTransform: "capitalize" }}
          />
          <Select.Content position="popper">
            {LOGGING_LEVELS.map((value) => (
              <Select.Item
                key={value}
                value={value}
                css={{ textTransform: "capitalize" }}
              >
                {value}
              </Select.Item>
            ))}
          </Select.Content>
        </Select.Root>

        <Flex align="center" justify="end" gap="2">
          <Button size="1" variant="surface" onClick={openLogsDir}>
            <LuFolderOpen /> Open
          </Button>
          <Button size="1" variant="surface" onClick={refreshLogs}>
            <LuRepeat /> Refresh
          </Button>
          <Button size="1" variant="surface" color="ruby" onClick={clearLogs}>
            <MdDeleteOutline /> Clear
          </Button>
        </Flex>
      </Flex>

      <Flex flexGrow="1" minHeight="0">
        <ScrollArea
          ref={parentRef}
          scrollbars="vertical"
          css={{ height: "100%" }}
        >
          <Box
            width="100%"
            position="relative"
            style={{ height: rowVirtualizer.getTotalSize() }}
          >
            {virtualItems.map((row) => {
              if (row.index >= entries.length) {
                return (
                  <Flex
                    key={row.key}
                    position="absolute"
                    top="0"
                    left="1"
                    right="1"
                    align="center"
                    justify="center"
                    py="1"
                    gap="2"
                    style={{ transform: `translateY(${row.start}px)` }}
                  >
                    <Spinner size="1" />
                    <Text size="1" color="gray">
                      Loading...
                    </Text>
                  </Flex>
                );
              }

              const entry = entries[row.index]!;

              let levelColor: CodeProps["color"] = "gray";
              switch (entry.level.toUpperCase()) {
                case "DEBUG":
                  levelColor = "violet";
                  break;
                case "INFO":
                  levelColor = "indigo";
                  break;
                case "WARN":
                  levelColor = "amber";
                  break;
                case "ERROR":
                  levelColor = "ruby";
                  break;
              }

              return (
                <Flex
                  key={row.key}
                  position="absolute"
                  top="0"
                  left="1"
                  right="1"
                  align="center"
                  pb="1"
                  style={{
                    transform: `translateY(${row.start}px)`,
                    borderBottom: "1px solid var(--gray-a5)",
                  }}
                >
                  <Flex width="100px" flexShrink="0">
                    <Text size="1">{formatTimestamp(entry.timestamp)}</Text>
                  </Flex>
                  <Flex width="60px" flexShrink="0">
                    <Code
                      size="1"
                      color={levelColor}
                      css={{ width: "45px", textAlign: "center" }}
                    >
                      {entry.level}
                    </Code>
                  </Flex>
                  <Flex
                    flexGrow="1"
                    css={{ whiteSpace: "nowrap", overflow: "hidden" }}
                  >
                    <Text
                      size="1"
                      title={`${entry.message}\n\n${JSON.stringify(entry.raw)}`}
                      css={{ overflow: "hidden", textOverflow: "ellipsis" }}
                    >
                      {entry.message}
                    </Text>
                  </Flex>
                </Flex>
              );
            })}
          </Box>
        </ScrollArea>
      </Flex>
    </Flex>
  );
});

export default Logs;
