import { deskulptCore } from "@deskulpt/bindings";
import { css } from "@emotion/react";
import { Code, Flex, Spinner, Text } from "@radix-ui/themes";

const styles = {
  row: css({
    borderBottom: "1px solid var(--gray-a5)",
  }),
  levelBadge: css({
    width: "45px",
    textAlign: "center",
  }),
  messageContainer: css({
    whiteSpace: "nowrap",
    overflow: "hidden",
  }),
  message: css({
    overflow: "hidden",
    textOverflow: "ellipsis",
  }),
};

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

function getLevelColor(level: string) {
  switch (level.toUpperCase()) {
    case "DEBUG":
      return "violet";
    case "INFO":
      return "indigo";
    case "WARN":
      return "amber";
    case "ERROR":
      return "ruby";
    default:
      return "gray";
  }
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

  return (
    <Flex
      position="absolute"
      top="0"
      left="1"
      right="1"
      align="center"
      pb="1"
      css={styles.row}
      style={{ transform: `translateY(${translateY}px)` }}
    >
      <Flex width="100px" flexShrink="0">
        <Text size="1">{formatTimestamp(entry.timestamp)}</Text>
      </Flex>
      <Flex width="60px" flexShrink="0">
        <Code
          size="1"
          color={getLevelColor(entry.level)}
          css={styles.levelBadge}
        >
          {entry.level}
        </Code>
      </Flex>
      <Flex flexGrow="1" css={styles.messageContainer}>
        <Text
          size="1"
          title={`${entry.message}\n\n${JSON.stringify(entry.raw)}`}
          css={styles.message}
        >
          {entry.message}
        </Text>
      </Flex>
    </Flex>
  );
};

export default Entry;
