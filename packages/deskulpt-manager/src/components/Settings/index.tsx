import { Box, Flex, ScrollArea, Table } from "@radix-ui/themes";
import { memo } from "react";
import Shortcut from "./Shortcut";
import SectionTable from "./SectionTable";

const Settings = memo(() => {
  return (
    <ScrollArea asChild>
      <Box height="420px" mt="1" pl="1" pr="3">
        <Flex direction="column" gap="4">
          <SectionTable title="Keyboard Shortcuts">
            <Table.Row align="center">
              <Table.RowHeaderCell>
                Toggle canvas interaction mode
              </Table.RowHeaderCell>
              <Table.Cell>
                <Shortcut shortcutKey="toggleCanvasImode" />
              </Table.Cell>
            </Table.Row>
            <Table.Row align="center">
              <Table.RowHeaderCell>Open manager</Table.RowHeaderCell>
              <Table.Cell>
                <Shortcut shortcutKey="openManager" />
              </Table.Cell>
            </Table.Row>
          </SectionTable>
        </Flex>
      </Box>
    </ScrollArea>
  );
});

export default Settings;
