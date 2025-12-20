import { Box, Button, Flex, ScrollArea, Select, Table } from "@radix-ui/themes";
import { memo, useCallback } from "react";
import { LuSquarePen } from "react-icons/lu";
import CanvasImode from "./CanvasImode";
import Shortcut from "./Shortcut";
import SectionTable from "./SectionTable";
import { deskulptCore, deskulptSettings } from "@deskulpt/bindings";
import { logger } from "@deskulpt/utils";
import { useSettingsStore } from "../../hooks";

const Settings = memo(() => {
  const hotReloadEnabled = useSettingsStore((state) => state.hotReloadEnabled);

  const openSettingsJson = useCallback(() => {
    deskulptCore.commands.open("settings").catch(logger.error);
  }, []);

  const onHotReloadChange = useCallback((value: string) => {
    const enabled = value === "on";
    deskulptSettings.commands
      .update({ hotReloadEnabled: enabled })
      .catch(logger.error);
  }, []);

  const hotReloadValue = hotReloadEnabled ? "on" : "off";

  return (
    <Flex direction="column" gap="4" px="1">
      <ScrollArea asChild>
        <Box height="380px">
          <Flex direction="column" gap="4">
            <SectionTable title="Basics">
              <Table.Row align="center">
                <Table.RowHeaderCell>
                  Canvas interaction mode
                </Table.RowHeaderCell>
                <Table.Cell justify="end">
                  <CanvasImode />
                </Table.Cell>
              </Table.Row>
              <Table.Row align="center">
                <Table.RowHeaderCell>Enable hot reload</Table.RowHeaderCell>
                <Table.Cell justify="end">
                  <Select.Root
                    size="1"
                    value={hotReloadValue}
                    onValueChange={onHotReloadChange}
                  >
                    <Select.Trigger />
                    <Select.Content>
                      <Select.Item value="on">On</Select.Item>
                      <Select.Item value="off">Off</Select.Item>
                    </Select.Content>
                  </Select.Root>
                </Table.Cell>
              </Table.Row>
            </SectionTable>
            <SectionTable title="Keyboard Shortcuts">
              <Table.Row align="center">
                <Table.RowHeaderCell>
                  Toggle canvas interaction mode
                </Table.RowHeaderCell>
                <Table.Cell>
                  <Shortcut action="toggleCanvasImode" />
                </Table.Cell>
              </Table.Row>
              <Table.Row align="center">
                <Table.RowHeaderCell>Open manager</Table.RowHeaderCell>
                <Table.Cell>
                  <Shortcut action="openManager" />
                </Table.Cell>
              </Table.Row>
            </SectionTable>
          </Flex>
        </Box>
      </ScrollArea>

      <Button size="2" variant="soft" color="gray" onClick={openSettingsJson}>
        <LuSquarePen /> Edit in settings.json
      </Button>
    </Flex>
  );
});

export default Settings;
