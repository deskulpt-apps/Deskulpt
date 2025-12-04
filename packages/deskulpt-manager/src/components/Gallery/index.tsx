import { Box, Flex } from "@radix-ui/themes";
import { ScrollArea } from "@radix-ui/themes/dist/cjs/index.js";
import { useVirtualizer } from "@tanstack/react-virtual";
import { memo, useCallback, useEffect, useRef, useState } from "react";
import Header from "./Header";
import WidgetCard from "./WidgetCard";
import { deskulptWidgets } from "@deskulpt/bindings";
import { logger } from "@deskulpt/utils";

const Gallery = memo(() => {
  const parentRef = useRef<HTMLDivElement>(null);
  const [widgets, setWidgets] = useState<deskulptWidgets.RegistryEntry[]>([]);

  const refresh = useCallback(() => {
    deskulptWidgets.commands
      .fetchRegistryIndex()
      .then((index) => {
        setWidgets(index.widgets);
      })
      .catch(logger.error);
  }, []);

  useEffect(refresh, [refresh]);

  const rowVirtualizer = useVirtualizer({
    count: widgets.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 108,
    overscan: 10,
  });

  const virtualItems = rowVirtualizer.getVirtualItems();

  return (
    <Flex height="100%" direction="column" px="1" gap="3">
      <Header refresh={refresh} />

      <Flex flexGrow="1" minHeight="0">
        <ScrollArea ref={parentRef} scrollbars="vertical" type="scroll">
          <Box
            width="100%"
            position="relative"
            style={{ height: rowVirtualizer.getTotalSize() }}
          >
            {virtualItems.map((row) => {
              const entry = widgets[row.index];
              return (
                <Box
                  key={row.key}
                  position="absolute"
                  top="0"
                  left="0"
                  right="0"
                  style={{
                    height: `${row.size}px`,
                    transform: `translateY(${row.start}px)`,
                  }}
                >
                  {entry !== undefined && <WidgetCard entry={entry} />}
                </Box>
              );
            })}
          </Box>
        </ScrollArea>
      </Flex>
    </Flex>
  );
});

export default Gallery;
