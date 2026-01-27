import { DeskulptWidgets } from "@deskulpt/bindings";
import { Card, Code, Flex, Heading, Text } from "@radix-ui/themes";
import { useWidgetsGalleryStore } from "../../hooks";
import WidgetPrimaryActions from "./WidgetPrimaryActions";
import WidgetSecondaryActions from "./WidgetSecondaryActions";

interface WidgetCardProps {
  index: number;
}

const WidgetCard = ({ index }: WidgetCardProps) => {
  const widget = useWidgetsGalleryStore((state) => state.widgets[index]!);

  const authorsRepr = widget.authors
    .map((author) => (typeof author === "string" ? author : author.name))
    .join(", ");

  const latestRelease = widget.releases.at(0);
  let reference: DeskulptWidgets.RegistryWidgetReference | undefined;
  if (latestRelease !== undefined) {
    reference = {
      handle: widget.handle,
      id: widget.id,
      digest: latestRelease.digest,
    };
  }

  return (
    <Card variant="surface" size="1">
      <Flex justify="between" px="1" gap="3">
        <Flex
          direction="column"
          gap="2"
          flexGrow="1"
          flexShrink="1"
          minWidth="0"
        >
          <Flex gap="3" align="center">
            <Heading size="2" weight="medium" truncate>
              {widget.name}
            </Heading>
            {latestRelease !== undefined && (
              <Code size="1" color="gray" variant="ghost" truncate>
                v{latestRelease.version}
              </Code>
            )}
          </Flex>
          <Text size="1" truncate>
            {widget.description}
          </Text>
          <Text size="1" color="gray" truncate>
            {authorsRepr}
          </Text>
        </Flex>

        {reference !== undefined && latestRelease !== undefined && (
          <Flex
            direction="column"
            align="end"
            justify="between"
            gap="2"
            flexGrow="0"
            flexShrink="0"
          >
            <WidgetPrimaryActions
              reference={reference}
              version={latestRelease.version}
            />
            <WidgetSecondaryActions
              reference={reference}
              version={latestRelease.version}
              releases={widget.releases}
            />
          </Flex>
        )}
      </Flex>
    </Card>
  );
};

export default WidgetCard;
