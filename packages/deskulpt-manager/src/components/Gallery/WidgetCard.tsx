import { deskulptWidgets } from "@deskulpt/bindings";
import { logger } from "@deskulpt/utils";
import {
  Button,
  Card,
  Code,
  DropdownMenu,
  Flex,
  Heading,
  IconButton,
  Text,
} from "@radix-ui/themes";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { useCallback, useState } from "react";
import { LuCopy, LuDownload, LuEllipsis } from "react-icons/lu";
import { toast } from "sonner";

interface WidgetCardProps {
  entry: deskulptWidgets.RegistryEntry;
}

const WidgetCard = ({ entry }: WidgetCardProps) => {
  const [isInstalling, setIsInstalling] = useState(false);

  const latestRelease = entry.releases.at(0);

  const fullId = `@${entry.handle}.${entry.id}`;
  const authorsRepr = entry.authors
    .map((author) => (typeof author === "string" ? author : author.name))
    .join(", ");

  const installLatestRelease = useCallback(async () => {
    if (latestRelease === undefined) {
      return;
    }

    setIsInstalling(true);
    try {
      await deskulptWidgets.commands.install(
        entry.handle,
        entry.id,
        latestRelease.digest,
      );
      toast.success(`Installed widget: ${fullId} (v${latestRelease.version})`);
    } catch (error) {
      logger.error(error);
      toast.error(
        `Failed to install widget: ${fullId} (v${latestRelease.version})`,
      );
    } finally {
      setIsInstalling(false);
    }
  }, [entry, latestRelease, fullId]);

  const copyWidgetId = useCallback(() => {
    writeText(fullId).then(() => toast.success("Copied to clipboard."));
  }, [fullId]);

  return (
    <Card variant="surface" size="2">
      <Flex justify="between" gap="3">
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

        <Flex
          direction="column"
          align="end"
          justify="between"
          gap="2"
          flexGrow="0"
          flexShrink="0"
        >
          <Button
            size="1"
            variant="surface"
            disabled={latestRelease === undefined}
            onClick={installLatestRelease}
            loading={isInstalling}
          >
            <LuDownload /> {latestRelease ? "Install" : "Unavailable"}
          </Button>
          <Flex pr="1">
            <DropdownMenu.Root>
              <DropdownMenu.Trigger>
                <IconButton size="1" variant="ghost">
                  <LuEllipsis size="16" />
                </IconButton>
              </DropdownMenu.Trigger>
              <DropdownMenu.Content
                size="1"
                variant="soft"
                color="gray"
                align="end"
              >
                <DropdownMenu.Item onClick={copyWidgetId}>
                  <LuCopy /> Copy widget ID
                </DropdownMenu.Item>
              </DropdownMenu.Content>
            </DropdownMenu.Root>
          </Flex>
        </Flex>
      </Flex>
    </Card>
  );
};

export default WidgetCard;
