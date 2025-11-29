import { deskulptCore } from "@deskulpt/bindings";
import { logger } from "@deskulpt/utils";
import { Button, Flex, Select } from "@radix-ui/themes";
import { Dispatch, SetStateAction, memo, useCallback } from "react";
import { LuFolderOpen, LuRepeat } from "react-icons/lu";
import { MdDeleteOutline } from "react-icons/md";

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
        <Select.Trigger css={{ width: "100px", textTransform: "capitalize" }} />
        <Select.Content position="popper">
          <Select.Item value="trace">Trace</Select.Item>
          <Select.Item value="debug">Debug</Select.Item>
          <Select.Item value="info">Info</Select.Item>
          <Select.Item value="warn">Warn</Select.Item>
          <Select.Item value="error">Error</Select.Item>
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
