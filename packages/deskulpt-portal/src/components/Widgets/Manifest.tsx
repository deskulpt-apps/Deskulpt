import { Badge, Box, Button, Code, Flex, ScrollArea } from "@radix-ui/themes";
import { useWidgetsStore } from "../../hooks";
import WidgetManifest from "../WidgetManifest";
import { LuFolderOpen, LuRepeat } from "react-icons/lu";
import { DeskulptCore, DeskulptWidgets } from "@deskulpt/bindings";
import { logger } from "@deskulpt/utils";

interface ManifestProps {
  id: string;
}

const Manifest = ({ id }: ManifestProps) => {
  const widget = useWidgetsStore((state) => state[id]);
  const isLoaded = widget?.settings.isLoaded ?? false;

  const toggleIsLoaded = () => {
    DeskulptWidgets.Commands.updateSettings(id, { isLoaded: !isLoaded });
  };

  return (
    <Flex direction="column" gap="2" pl="2">
      <Flex align="center" justify="between">
        <Badge color={widget?.manifest.type === "ok" ? "gray" : "ruby"}>
          {id}
        </Badge>
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
            onClick={() =>
              DeskulptWidgets.Commands.refresh(id).catch(logger.error)
            }
            disabled={!isLoaded}
          >
            <LuRepeat /> Refresh
          </Button>
          <Button
            title="Open this widget folder"
            size="1"
            variant="surface"
            onClick={() =>
              DeskulptCore.Commands.open({ widget: id }).catch(logger.error)
            }
          >
            <LuFolderOpen /> Edit
          </Button>
        </Flex>
      </Flex>

      <ScrollArea asChild>
        <Box height="200px" pr="3" pb="3">
          {widget?.manifest.type === "ok" ? (
            <WidgetManifest manifest={widget.manifest.content} />
          ) : (
            <Box pl="2" m="0" asChild>
              <pre>
                <Code size="2" variant="ghost">
                  {widget?.manifest.content ?? "Widget not found."}
                </Code>
              </pre>
            </Box>
          )}
        </Box>
      </ScrollArea>
    </Flex>
  );
};

export default Manifest;
