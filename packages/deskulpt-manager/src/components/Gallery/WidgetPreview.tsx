import { deskulptWidgets } from "@deskulpt/bindings";
import {
  Dialog,
  Flex,
  IconButton,
  Link,
  ScrollArea,
  Separator,
  Text,
  VisuallyHidden,
} from "@radix-ui/themes";
import { formatBytes } from "@deskulpt/utils";
import { css } from "@emotion/react";
import { LuCalendar, LuCode, LuPackage, LuX } from "react-icons/lu";
import WidgetManifest from "../WidgetManifest";

const styles = {
  previewScrollArea: css({
    "[data-radix-scroll-area-viewport] > div": { width: "100%" },
  }),
};

interface WidgetPreviewProps {
  preview?: deskulptWidgets.RegistryWidgetPreview;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

const WidgetPreview = ({ preview, open, onOpenChange }: WidgetPreviewProps) => {
  if (preview === undefined) {
    return null;
  }

  const { id, size, created, git, ...manifest } = preview;
  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Content size="1" aria-labelledby={undefined} asChild>
        <Flex minWidth="85vw" maxHeight="80vh" direction="column" gap="2">
          <VisuallyHidden asChild>
            <Dialog.Title>Preview: Widget {id}</Dialog.Title>
          </VisuallyHidden>

          <Flex align="center" justify="between" gap="3">
            <Text size="2" weight="medium" truncate>
              {id}
            </Text>
            <Flex align="center" gap="3" flexShrink="0">
              <Flex align="center" gap="2">
                <LuPackage color="var(--gray-a10)" />
                <Text size="1" color="gray">
                  {formatBytes(size)}
                </Text>
              </Flex>
              {created !== undefined && (
                <Flex align="center" gap="2">
                  <LuCalendar color="var(--gray-a10)" />
                  <Text size="1" color="gray">
                    {new Date(created).toLocaleDateString()}
                  </Text>
                </Flex>
              )}
              <Separator orientation="vertical" />
              {git !== undefined && (
                <IconButton size="1" variant="ghost" asChild>
                  <Link href={git} title="Source code">
                    <LuCode size={16} />
                  </Link>
                </IconButton>
              )}
              <Dialog.Close>
                <IconButton size="1" variant="ghost" color="ruby">
                  <LuX size={16} />
                </IconButton>
              </Dialog.Close>
            </Flex>
          </Flex>

          <ScrollArea
            scrollbars="vertical"
            type="scroll"
            size="1"
            css={styles.previewScrollArea}
            asChild
          >
            <Flex minHeight="0">
              <WidgetManifest manifest={manifest} />
            </Flex>
          </ScrollArea>
        </Flex>
      </Dialog.Content>
    </Dialog.Root>
  );
};

export default WidgetPreview;
