import React from "react";
import {
  Box,
  Code,
  Flex,
  Heading,
  Separator,
  Strong,
  Text,
} from "@deskulpt-test/ui";

export default function Widget() {
  return (
    <Flex
      direction="column"
      gap="4"
      p="5"
      style={{
        inset: 0,
        position: "fixed",
        backgroundColor: "white",
        color: "#1a1a1a",
        overflowY: "auto",
        pointerEvents: "auto",
      }}
    >
      <Heading size="6" mb="2">
        Welcome to Deskulpt
      </Heading>

      <Text as="p" size="2" style={{ lineHeight: 1.5 }}>
        New users just drop widget folders into the widgets directory Deskulpt
        watches. Each widget is a folder with a{" "}
        <Code>deskulpt.widget.json</Code> manifest and its source files.
      </Text>

      <Text as="p" size="2" style={{ lineHeight: 1.5 }}>
        <Strong>Important:</Strong> Put that folder directly under the widgets
        root, not nested.
      </Text>

      <Separator size="4" style={{ backgroundColor: "#e0e0e0" }} />

      <Heading size="4">How to find the widgets folder</Heading>

      <Box>
        <Heading size="3" mb="1">
          From the app
        </Heading>
        <Text as="p" size="2" style={{ lineHeight: 1.5 }}>
          Use the command/shortcut that opens the widgets folder (look for
          &ldquo;Open widgets folder&rdquo; in the UI).
        </Text>
      </Box>

      <Box>
        <Heading size="3" mb="1">
          On disk
        </Heading>
        <Flex direction="column" gap="2">
          <Text size="2" style={{ lineHeight: 1.5 }}>
            <Strong>Dev/debug build:</Strong> <Code>target/debug/widgets</Code>
          </Text>
          <Text size="2" style={{ lineHeight: 1.5 }}>
            <Strong>Packaged app:</Strong> <Code>Deskulpt/widgets</Code> under
            your user documents/app data.
          </Text>
        </Flex>
      </Box>

      <Separator size="4" style={{ backgroundColor: "#e0e0e0" }} />

      <Text as="p" size="2" style={{ lineHeight: 1.5 }}>
        After you add a widget folder, refresh or restart to load it. Remove the
        folder to uninstall.
      </Text>
    </Flex>
  );
}
