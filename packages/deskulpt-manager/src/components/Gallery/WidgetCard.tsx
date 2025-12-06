import { deskulptWidgets } from "@deskulpt/bindings";
import { Card, Code, Flex, Heading, Text } from "@radix-ui/themes";
import { useWidgetsStore } from "../../hooks";
import WidgetPrimaryActions from "./WidgetPrimaryActions";
import WidgetSecondaryActions from "./WidgetSecondaryActions";

interface WidgetCardProps {
  entry: deskulptWidgets.RegistryEntry;
  showPreview: (preview: deskulptWidgets.RegistryWidgetPreview) => void;
}

const WidgetCard = ({ entry, showPreview }: WidgetCardProps) => {
  const id = `@${entry.handle}.${entry.id}`;
  const localWidget = useWidgetsStore((state) => state[id]);

  const authorsRepr = entry.authors
    .map((author) => (typeof author === "string" ? author : author.name))
    .join(", ");

  let action;
  let widget: deskulptWidgets.RegistryWidgetReference | undefined;

  const latestRelease = entry.releases.at(0);
  if (latestRelease !== undefined) {
    widget = {
      handle: entry.handle,
      id: entry.id,
      digest: latestRelease.digest,
    };
    if (localWidget === undefined) {
      action = "install" as const;
    } else if (
      localWidget?.type === "ok" &&
      localWidget.content.version === latestRelease.version
    ) {
      action = "uninstall" as const;
    } else {
      action = "upgrade" as const;
    }
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
              {entry.name}
            </Heading>
            {latestRelease !== undefined && (
              <Code size="1" color="gray" variant="ghost" truncate>
                v{latestRelease.version}
              </Code>
            )}
          </Flex>
          <Text size="1" truncate>
            {entry.description}
          </Text>
          <Text size="1" color="gray" truncate>
            {authorsRepr}
          </Text>
        </Flex>

        {action !== undefined && widget !== undefined && (
          <Flex
            direction="column"
            align="end"
            justify="between"
            gap="2"
            flexGrow="0"
            flexShrink="0"
          >
            <WidgetPrimaryActions action={action} widget={widget} />
            <WidgetSecondaryActions widget={widget} showPreview={showPreview} />
          </Flex>
        )}
      </Flex>
    </Card>
  );
};

export default WidgetCard;
