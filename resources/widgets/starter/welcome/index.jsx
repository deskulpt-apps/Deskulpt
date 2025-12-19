import { Avatar, Flex, Heading, Link, Text } from "@deskulpt-test/ui";

function Welcome() {
  return (
    <Flex
      direction="column"
      height="100%"
      width="100%"
      overflowY="auto"
      p="3"
      gap="2"
      css={{ backgroundColor: "var(--gray-surface)", scrollbarWidth: "none" }}
    >
      <Flex align="center" gap="2">
        <Avatar src="/deskulpt.svg" fallback="D" size="1" />
        <Heading size="3">Welcome to Deskulpt!</Heading>
      </Flex>
      <Text size="1">
        Click the Deskulpt icon in the system tray to reveal the Deskulpt
        manager interface, where you can manage your widgets, adjust Deskulpt
        settings, and more.
      </Text>
      <Text size="1">
        Switch to the &quot;Gallery&quot; tab in the manager to explore and
        install widgets from the official gallery.
      </Text>
      <Text size="1">
        Click the folder icon in the bottom left of the manager to open the
        widgets base directory where all your widgets should live.
      </Text>
      <Text size="1">
        Check out{" "}
        <Link href="https://deskulpt-apps.github.io/">our website</Link> for
        more information.
      </Text>
    </Flex>
  );
}

export default Welcome;
