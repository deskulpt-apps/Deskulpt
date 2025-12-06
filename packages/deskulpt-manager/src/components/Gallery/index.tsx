import { Box, Flex, ScrollArea, Spinner, Text } from "@radix-ui/themes";
import { useVirtualizer } from "@tanstack/react-virtual";
import { memo, useCallback, useEffect, useRef, useState } from "react";
import { deskulptWidgets } from "@deskulpt/bindings";
import { logger } from "@deskulpt/utils";
import { toast } from "sonner";
import Header from "./Header";
import WidgetCard from "./WidgetCard";
import WidgetPreview from "./WidgetPreview";

const Gallery = memo(() => {
  const parentRef = useRef<HTMLDivElement>(null);

  const [widgets, setWidgets] = useState<deskulptWidgets.RegistryEntry[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [isPreviewOpen, setIsPreviewOpen] = useState(false);
  const [preview, setPreview] = useState<
    deskulptWidgets.RegistryWidgetPreview | undefined
  >();

  const showPreview = useCallback(
    (preview: deskulptWidgets.RegistryWidgetPreview) => {
      setIsPreviewOpen(true);
      setPreview(preview);
    },
    [],
  );

  const refresh = useCallback(async () => {
    setWidgets([]);
    setIsLoading(true);
    // await new Promise((resolve) => setTimeout(resolve, 1000));
    try {
      const index = await deskulptWidgets.commands.fetchRegistryIndex();
      setWidgets(index.widgets);
    } catch (error) {
      logger.error(error);
      toast.error("Failed to load widgets gallery");
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    refresh();
  }, [refresh]);

  const rowVirtualizer = useVirtualizer({
    count: widgets.length,
    getScrollElement: () => parentRef.current,
    estimateSize: () => 100,
    overscan: 5,
  });

  return (
    <Flex height="100%" direction="column" px="1" gap="3">
      <Header refresh={refresh} />

      {isLoading ? (
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
                    {entry !== undefined && (
                      <WidgetCard entry={entry} showPreview={showPreview} />
                    )}
                  </Box>
                );
              })}
            </Box>
          </ScrollArea>
        </Flex>
      )}

      <WidgetPreview
        preview={preview}
        open={isPreviewOpen}
        onOpenChange={setIsPreviewOpen}
      />
    </Flex>
  );
});

export default Gallery;
