import { Box, Button, Flex, ScrollArea, Table } from "@radix-ui/themes";
import { memo, useCallback } from "react";
import { LuSquarePen } from "react-icons/lu";
import CanvasImode from "./CanvasImode";
import HotReload from "./HotReload";
import Shortcut from "./Shortcut";
import SectionTable from "./SectionTable";
import { deskulptCore } from "@deskulpt/bindings";
import { logger } from "@deskulpt/utils";

const Settings = memo(() => {
  const openSettingsJson = useCallback(() => {
    deskulptCore.commands.open("settings").catch(logger.error);
  }, []);

  return (
    <Flex direction="column" gap="4" px="1" height="100%">
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
                  <HotReload />
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
