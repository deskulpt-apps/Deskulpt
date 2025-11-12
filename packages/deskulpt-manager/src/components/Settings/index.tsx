import { Box, Flex, ScrollArea, Table } from "@radix-ui/themes";
import { memo } from "react";
import CanvasImode from "./CanvasImode";
import Shortcut from "./Shortcut";
import SectionTable from "./SectionTable";

const Settings = memo(() => {
  return (
    <ScrollArea asChild>
      <Box height="420px" mt="1" pl="1" pr="3">
        <Flex direction="column" gap="4">
          <SectionTable title="Basics">
            <Table.Row align="center">
              <Table.RowHeaderCell>Canvas interaction mode</Table.RowHeaderCell>
              <Table.Cell justify="end">
                <CanvasImode />
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
  );
});

export default Settings;
