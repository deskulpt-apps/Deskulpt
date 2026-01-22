import {
  Box,
  Button,
  Dialog,
  Flex,
  ScrollArea,
  Text,
  VisuallyHidden,
} from "@radix-ui/themes";
import { css } from "@emotion/react";
import { useWidgetsGalleryStore } from "../../hooks";
import { deskulptWidgets } from "@deskulpt/bindings";
import { logger } from "@deskulpt/utils";
import { toast } from "sonner";

const styles = {
  previewScrollArea: css({
    "[data-radix-scroll-area-viewport] > div": { width: "100%" },
  }),
  dataListRoot: css({
    gap: "var(--space-2)",
  }),
};

const WidgetVersionPicker = () => {
  const data = useWidgetsGalleryStore((state) => state.versionPickerData);
  const isOpen = useWidgetsGalleryStore((state) => state.isVersionPickerOpen);
  const openPreview = useWidgetsGalleryStore((state) => state.openPreview);

  const onOpenChange = (open: boolean) => {
    if (!open) {
      useWidgetsGalleryStore.setState({
        isVersionPickerOpen: false,
        versionPickerData: undefined,
      });
    }
  };

  const onSelect = async (release: deskulptWidgets.RegistryEntryRelease) => {
    if (data === undefined) {
      return;
    }
    onOpenChange(false);
    const reference = {
      handle: data.handle,
      id: data.id,
      digest: release.digest,
    };

    try {
      const previewData = await deskulptWidgets.commands.preview(reference);
      openPreview({
        reference,
        version: release.version,
        preview: previewData,
      });
    } catch (error) {
      logger.error(error);
      toast.error("Failed to load preview.");
    }
  };

  if (data === undefined) {
    return null;
  }

  return (
    <Dialog.Root open={isOpen} onOpenChange={onOpenChange}>
      <Dialog.Content size="1" asChild>
        <Flex width="400px" maxHeight="300px" direction="column" gap="2">
          <VisuallyHidden asChild>
            <Dialog.Title>Widget Version Picker</Dialog.Title>
          </VisuallyHidden>

          <Dialog.Description size="2" weight="medium">
            Click to view a different version:
          </Dialog.Description>
          <ScrollArea
            scrollbars="vertical"
            type="scroll"
            size="1"
            css={styles.previewScrollArea}
            asChild
          >
            <Box minHeight="0">
              <Flex direction="column" gap="2" py="1" pl="2" pr="4">
                {data.releases.map((release) => {
                  const dateRepr = new Date(
                    release.publishedAt,
                  ).toLocaleDateString(undefined, {
                    year: "numeric",
                    month: "2-digit",
                    day: "2-digit",
                  });
                  return (
                    <Button
                      key={release.version}
                      variant="ghost"
                      size="1"
                      color="gray"
                      onClick={() => onSelect(release)}
                      asChild
                    >
                      <Flex align="start" justify="between" gap="3">
                        <Text size="2" truncate highContrast>
                          {release.version}
                        </Text>
                        <Text size="2">{dateRepr}</Text>
                      </Flex>
                    </Button>
                  );
                })}
              </Flex>
            </Box>
          </ScrollArea>
        </Flex>
      </Dialog.Content>
    </Dialog.Root>
  );
};

export default WidgetVersionPicker;
