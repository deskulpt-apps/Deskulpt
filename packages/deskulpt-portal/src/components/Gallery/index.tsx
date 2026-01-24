import { Box, Flex, ScrollArea, Spinner, Text } from "@radix-ui/themes";
import { useVirtualizer } from "@tanstack/react-virtual";
import { useEffect, useRef } from "react";
import Header from "./Header";
import WidgetCard from "./WidgetCard";
import WidgetPreview from "./WidgetPreview";
import WidgetVersionPicker from "./WidgetVersionPicker";
import { useWidgetsGalleryStore } from "../../hooks";

const Gallery = () => {
  const parentRef = useRef<HTMLDivElement>(null);

  const numWidgets = useWidgetsGalleryStore((state) => state.widgets.length);
  const isFetching = useWidgetsGalleryStore((state) => state.isFetching);
  const refresh = useWidgetsGalleryStore((state) => state.refresh);

  useEffect(() => {
    refresh();
  }, [refresh]);

  const rowVirtualizer = useVirtualizer({
    count: numWidgets,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 100,
    overscan: 5,
  });

  return (
    <Flex height="100%" direction="column" px="1" gap="3">
      <Header refresh={refresh} />

      {isFetching ? (
        <Flex
          height="100%"
          width="100%"
          align="center"
          justify="center"
          gap="3"
          pb="9"
        >
          <Spinner size="2" />
          <Text size="2">Loading...</Text>
        </Flex>
      ) : (
        <Flex flexGrow="1" minHeight="0">
          <ScrollArea ref={parentRef} scrollbars="vertical" type="scroll">
            <Box
              width="100%"
              position="relative"
              style={{ height: rowVirtualizer.getTotalSize() }}
            >
              {rowVirtualizer.getVirtualItems().map((row) => {
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
                    <WidgetCard index={row.index} />
                  </Box>
                );
              })}
            </Box>
          </ScrollArea>
        </Flex>
      )}

      <WidgetPreview />
      <WidgetVersionPicker />
    </Flex>
  );
};

export default Gallery;
