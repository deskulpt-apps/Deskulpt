import {
  Box,
  DataList,
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
import { LuCodeXml, LuExternalLink, LuPackage, LuX } from "react-icons/lu";
import WidgetManifest from "../WidgetManifest";
import WidgetPrimaryActions from "./WidgetPrimaryActions";
import { useWidgetsGalleryStore } from "../../hooks";
import { useCallback } from "react";

const styles = {
  previewScrollArea: css({
    "[data-radix-scroll-area-viewport] > div": { width: "100%" },
  }),
  dataListRoot: css({
    gap: "var(--space-2)",
  }),
};

const WidgetPreview = () => {
  const data = useWidgetsGalleryStore((state) => state.previewData);
  const isOpen = useWidgetsGalleryStore((state) => state.isPreviewOpen);

  const onOpenChange = useCallback((open: boolean) => {
    if (!open) {
      useWidgetsGalleryStore.setState({
        isPreviewOpen: false,
        previewData: undefined,
      });
    }
  }, []);

  if (data === undefined) {
    return null;
  }
  const { reference, version, preview } = data;

  return (
    <Dialog.Root open={isOpen} onOpenChange={onOpenChange}>
      <Dialog.Content size="1" aria-labelledby={undefined} asChild>
        <Flex minWidth="85vw" maxHeight="80vh" direction="column" gap="2">
          <VisuallyHidden asChild>
            <Dialog.Title>Widget Preview: {preview.id}</Dialog.Title>
          </VisuallyHidden>

          <Flex align="center" justify="between" gap="3">
            <Text size="2" weight="medium" truncate>
              {preview.id}
            </Text>
            <Flex align="center" gap="3" flexShrink="0">
              <WidgetPrimaryActions reference={reference} version={version} />
              {preview.git !== undefined && (
                <IconButton size="1" variant="ghost" asChild>
                  <Link href={preview.git}>
                    <LuCodeXml size={16} />
                  </Link>
                </IconButton>
              )}
              <IconButton size="1" variant="ghost" asChild>
                <Link href={preview.registryUrl}>
                  <LuPackage size={16} />
                </Link>
              </IconButton>
              <IconButton size="1" variant="ghost" asChild>
                <Link href="https://github.com/deskulpt-apps/widgets">
                  <LuExternalLink size={16} />
                </Link>
              </IconButton>
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
            <Box minHeight="0">
              <Flex direction="column" gap="3">
                <WidgetManifest manifest={preview} />
                <Separator size="4" />

                <DataList.Root size="2" css={styles.dataListRoot}>
                  {preview.created !== undefined && (
                    <DataList.Item>
                      <DataList.Label minWidth="88px">Published</DataList.Label>
                      <DataList.Value>
                        <Flex align="center" gap="1">
                          {new Date(preview.created).toLocaleString(undefined, {
                            year: "numeric",
                            month: "2-digit",
                            day: "2-digit",
                            hour: "2-digit",
                            minute: "2-digit",
                            second: "2-digit",
                          })}
                        </Flex>
                      </DataList.Value>
                    </DataList.Item>
                  )}
                  <DataList.Item>
                    <DataList.Label minWidth="88px">
                      Package Size
                    </DataList.Label>
                    <DataList.Value>{formatBytes(preview.size)}</DataList.Value>
                  </DataList.Item>
                </DataList.Root>
              </Flex>
            </Box>
          </ScrollArea>
        </Flex>
      </Dialog.Content>
    </Dialog.Root>
  );
};

export default WidgetPreview;
