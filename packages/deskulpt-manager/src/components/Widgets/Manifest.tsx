import { Badge, Box, Button, Code, Flex, ScrollArea } from "@radix-ui/themes";
import { useSettingsStore, useWidgetsStore } from "../../hooks";
import { memo, useCallback } from "react";
import WidgetManifest from "../WidgetManifest";
import { LuFolderOpen, LuRepeat } from "react-icons/lu";
import {
  deskulptCore,
  deskulptSettings,
  deskulptWidgets,
} from "@deskulpt/bindings";
import { logger } from "@deskulpt/utils";

interface ManifestProps {
  id: string;
}

const Manifest = memo(({ id }: ManifestProps) => {
  const manifest = useWidgetsStore((state) => state[id]);
  const isLoaded = useSettingsStore(
    (state) => state.widgets[id]?.isLoaded ?? false,
  );

  const toggleIsLoaded = useCallback(() => {
    deskulptSettings.commands.update({
      widgets: { [id]: { isLoaded: !isLoaded } },
    });
  }, [id, isLoaded]);

  const refreshAction = useCallback(() => {
    deskulptWidgets.commands.refresh(id).catch(logger.error);
  }, [id]);

  const openAction = useCallback(() => {
    deskulptCore.commands.open({ widget: id }).catch(logger.error);
  }, [id]);

  return (
    <Flex direction="column" gap="2" pl="2">
      <Flex align="center" justify="between">
        <Badge color={manifest?.type === "ok" ? "gray" : "ruby"}>{id}</Badge>
        <Flex align="center" gap="2">
          <Button
            size="1"
            variant="surface"
            color={isLoaded ? "gray" : undefined}
            onClick={toggleIsLoaded}
          >
            {isLoaded ? "Unload" : "Load"}
          </Button>
          <Button
            title="Refresh this widget"
            size="1"
            variant="surface"
            onClick={refreshAction}
            disabled={!isLoaded}
          >
            <LuRepeat /> Refresh
          </Button>
          <Button
            title="Open this widget folder"
            size="1"
            variant="surface"
            onClick={openAction}
          >
            <LuFolderOpen /> Edit
          </Button>
        </Flex>
      </Flex>

      <ScrollArea asChild>
        <Box height="200px" pr="3" pb="3">
          {manifest?.type === "ok" ? (
            <WidgetManifest manifest={manifest.content} />
          ) : (
            <Box pl="2" m="0" asChild>
              <pre>
                <Code size="2" variant="ghost">
                  {manifest?.content ?? "Widget not found."}
                </Code>
              </pre>
            </Box>
          )}
        </Box>
      </ScrollArea>
    </Flex>
  );
});

export default Manifest;
