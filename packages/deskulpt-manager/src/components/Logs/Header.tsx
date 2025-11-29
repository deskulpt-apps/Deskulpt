import { deskulptCore } from "@deskulpt/bindings";
import { LOGGING_LEVELS, logger } from "@deskulpt/utils";
import { css } from "@emotion/react";
import { Button, Flex, Select } from "@radix-ui/themes";
import { Dispatch, SetStateAction, memo, useCallback } from "react";
import { LuFolderOpen, LuRepeat } from "react-icons/lu";
import { MdDeleteOutline } from "react-icons/md";

const styles = {
  minLevelSelect: css({
    width: "100px",
    textTransform: "capitalize",
  }),
  minLevelOption: css({
    textTransform: "capitalize",
  }),
};

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
    deskulptCore.commands.clearLogs().catch(logger.error).then(refreshLogs);
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
        <Button size="1" variant="surface" color="ruby" onClick={clearLogs}>
          <MdDeleteOutline /> Clear
        </Button>
      </Flex>
    </Flex>
  );
});

export default Header;
