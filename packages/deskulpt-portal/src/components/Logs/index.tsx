import { Box, Flex, ScrollArea, Text } from "@radix-ui/themes";
import { useEffect, useRef, useState } from "react";
import { DeskulptLogs } from "@deskulpt/bindings";
import { useVirtualizer } from "@tanstack/react-virtual";
import { useLogs } from "../../hooks";
import Header from "./Header";
import Entry from "./Entry";
import { LuLogs } from "react-icons/lu";

const Logs = () => {
  const parentRef = useRef<HTMLDivElement>(null);
  const [minLevel, setMinLevel] = useState<DeskulptLogs.Level>("info");

  const { entries, hasMore, isFetching, fetchMore, refresh } = useLogs({
    minLevel,
    pageSize: 100,
  });

  const rowVirtualizer = useVirtualizer({
    // If we are fetching, add an extra row so that the index is out of range
    // and it will render a loading indicator
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

  return (
    <Flex height="100%" direction="column" px="1" gap="3">
      <Header
        minLevel={minLevel}
        setMinLevel={setMinLevel}
        refreshLogs={refresh}
      />

      <Flex flexGrow="1" minHeight="0">
        <ScrollArea ref={parentRef} scrollbars="vertical" type="scroll">
          {!isFetching && entries.length === 0 ? (
            <Flex
              height="100%"
              width="100%"
              align="center"
              justify="center"
              gap="3"
              pb="9"
            >
              <LuLogs size={20} color="var(--gray-a11)" />
              <Text size="2" weight="medium" color="gray">
                No log entries found
              </Text>
            </Flex>
          ) : (
            <Box
              width="100%"
              position="relative"
              style={{ height: rowVirtualizer.getTotalSize() }}
            >
              {virtualItems.map((row) => (
                <Box
                  key={row.key}
                  position="absolute"
                  top="0"
                  left="1"
                  right="1"
                  style={{
                    height: row.size,
                    transform: `translateY(${row.start}px)`,
                  }}
                >
                  <Entry entry={entries[row.index]} />
                </Box>
              ))}
            </Box>
          )}
        </ScrollArea>
      </Flex>
    </Flex>
  );
};

export default Logs;
