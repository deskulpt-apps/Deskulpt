import { deskulptCore } from "@deskulpt/bindings";
import { LOGGING_LEVELS, logger } from "@deskulpt/utils";
import { css } from "@emotion/react";
import { Button, Flex, Popover, Select, Text } from "@radix-ui/themes";
import { Dispatch, SetStateAction, memo, useCallback } from "react";
import { LuFolderOpen, LuRepeat } from "react-icons/lu";
import { MdDeleteOutline } from "react-icons/md";
import { toast } from "sonner";

const styles = {
  minLevelSelect: css({
    width: "100px",
    textTransform: "capitalize",
  }),
  minLevelOption: css({
    textTransform: "capitalize",
  }),
};

function formatBytes(bytes: number) {
  const k = 1024;
  if (bytes < 1024) {
    return `${bytes} B`;
  }

  const units = ["KB", "MB", "GB", "TB"] as const;
  let value = bytes;
  let unitIndex = 0;
  while (value >= k && unitIndex < units.length - 1) {
    value /= k;
    unitIndex++;
  }
  return `${value.toFixed(2)} ${units[unitIndex]}`;
}

interface HeaderProps {
  minLevel: deskulptCore.LoggingLevel;
  setMinLevel: Dispatch<SetStateAction<deskulptCore.LoggingLevel>>;
  refreshLogs: () => void;
}

const Header = memo(({ minLevel, setMinLevel, refreshLogs }: HeaderProps) => {
  const openLogsDir = useCallback(() => {
    deskulptCore.commands.open("logs").catch(logger.error);
  }, []);

  const clearLogs = useCallback(() => {
    deskulptCore.commands
      .clearLogs()
      .then((bytes) => {
        toast.success(`Cleaned up ${formatBytes(bytes)} of logs.`);
        refreshLogs();
      })
      .catch(logger.error);
  }, [refreshLogs]);

  return (
    <Flex align="center" gap="2" justify="between">
      <Select.Root
        size="1"
        value={minLevel}
        onValueChange={(value) =>
          setMinLevel(value as deskulptCore.LoggingLevel)
        }
      >
        <Select.Trigger css={styles.minLevelSelect} />
        <Select.Content position="popper">
          {LOGGING_LEVELS.map((level) => (
            <Select.Item key={level} value={level} css={styles.minLevelOption}>
              {level}
            </Select.Item>
          ))}
        </Select.Content>
      </Select.Root>

      <Flex align="center" justify="end" gap="2">
        <Button size="1" variant="surface" onClick={openLogsDir}>
          <LuFolderOpen /> Open
        </Button>
        <Button size="1" variant="surface" onClick={refreshLogs}>
          <LuRepeat /> Refresh
        </Button>

        <Popover.Root>
          <Popover.Trigger>
            <Button size="1" variant="surface" color="ruby">
              <MdDeleteOutline /> Clear
            </Button>
          </Popover.Trigger>
          <Popover.Content size="1" maxWidth="300px">
            <Flex direction="column" gap="3">
              <Text size="1">
                This will permanently delete all log files from disk. Are you
                sure to continue?
              </Text>
              <Flex gap="2" justify="end">
                <Popover.Close>
                  <Button size="1" variant="soft">
                    Cancel
                  </Button>
                </Popover.Close>
                <Popover.Close>
                  <Button
                    size="1"
                    variant="soft"
                    color="ruby"
                    onClick={clearLogs}
                  >
                    Confirm
                  </Button>
                </Popover.Close>
              </Flex>
            </Flex>
          </Popover.Content>
        </Popover.Root>
      </Flex>
    </Flex>
  );
});

export default Header;
