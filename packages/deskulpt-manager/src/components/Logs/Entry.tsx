import { deskulptCore } from "@deskulpt/bindings";
import { Code, CodeProps, Flex, Spinner, Text } from "@radix-ui/themes";

function formatTimestamp(timestamp: string) {
  const date = new Date(timestamp);

  const dateString = date.toLocaleDateString(undefined, {
    month: "2-digit",
    day: "2-digit",
  });
  const timeString = date.toLocaleTimeString(undefined, {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });

  return `${dateString} ${timeString}`;
}

interface EntryProps {
  entry?: deskulptCore.LogEntry;
  translateY: number;
}

const Entry = ({ entry, translateY }: EntryProps) => {
  if (entry === undefined) {
    return (
      <Flex
        position="absolute"
        top="0"
        left="1"
        right="1"
        align="center"
        justify="center"
        py="1"
        gap="2"
        style={{ transform: `translateY(${translateY}px)` }}
      >
        <Spinner size="1" />
        <Text size="1" color="gray">
          Loading...
        </Text>
      </Flex>
    );
  }

  let levelColor: CodeProps["color"] = "gray";
  switch (entry.level.toUpperCase()) {
    case "DEBUG":
      levelColor = "violet";
      break;
    case "INFO":
      levelColor = "indigo";
      break;
    case "WARN":
      levelColor = "amber";
      break;
    case "ERROR":
      levelColor = "ruby";
      break;
  }

  return (
    <Flex
      position="absolute"
      top="0"
      left="1"
      right="1"
      align="center"
      pb="1"
      style={{
        transform: `translateY(${translateY}px)`,
        borderBottom: "1px solid var(--gray-a5)",
      }}
    >
      <Flex width="100px" flexShrink="0">
        <Text size="1">{formatTimestamp(entry.timestamp)}</Text>
      </Flex>
      <Flex width="60px" flexShrink="0">
        <Code
          size="1"
          color={levelColor}
          css={{ width: "45px", textAlign: "center" }}
        >
          {entry.level}
        </Code>
      </Flex>
      <Flex flexGrow="1" css={{ whiteSpace: "nowrap", overflow: "hidden" }}>
        <Text
          size="1"
          title={`${entry.message}\n\n${JSON.stringify(entry.raw)}`}
          css={{ overflow: "hidden", textOverflow: "ellipsis" }}
        >
          {entry.message}
        </Text>
      </Flex>
    </Flex>
  );
};

export default Entry;
