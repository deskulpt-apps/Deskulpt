import { Box, Flex, ScrollArea, Switch, Table } from "@radix-ui/themes";
import { memo, useCallback } from "react";
import { deskulptCore } from "@deskulpt/bindings";
import Shortcut from "./Shortcut";
import SectionTable from "./SectionTable";
import { useSettingsStore } from "../../hooks";

const Settings = memo(() => {
  const { enable_telemetry } = useSettingsStore((state) => state.settings);

  const handleTelemetryChange = useCallback((checked: boolean) => {
    deskulptCore.commands.updateSettings({
      enableTelemetry: checked,
    });
  }, []);

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

          <SectionTable title="Privacy & Telemetry">
            <Table.Row align="center">
              <Table.RowHeaderCell>
                <Box>
                  <div style={{ fontWeight: 500 }}>Help improve Deskulpt</div>
                  <div
                    style={{
                      fontSize: "12px",
                      color: "var(--gray-11)",
                      marginTop: "4px",
                    }}
                  >
                    Enable crash reporting and telemetry (opt-in)
                  </div>
                </Box>
              </Table.RowHeaderCell>
              <Table.Cell>
                <Switch
                  checked={enable_telemetry || false}
                  onCheckedChange={handleTelemetryChange}
                />
              </Table.Cell>
            </Table.Row>
            {enable_telemetry && (
              <Table.Row>
                <Table.Cell colSpan={2}>
                  <Box
                    style={{
                      fontSize: "12px",
                      color: "var(--gray-11)",
                      padding: "8px",
                      backgroundColor: "var(--gray-2)",
                      borderRadius: "4px",
                      marginTop: "8px",
                    }}
                  >
                    <strong>What we collect:</strong>
                    <ul
                      style={{
                        margin: "4px 0 0 0",
                        paddingLeft: "16px",
                      }}
                    >
                      <li>Crash reports and error messages</li>
                      <li>Performance metrics</li>
                      <li>Feature usage (anonymized)</li>
                    </ul>
                    <strong style={{ marginTop: "8px", display: "block" }}>
                      What we don&apos;t collect:
                    </strong>
                    <ul
                      style={{
                        margin: "4px 0 0 0",
                        paddingLeft: "16px",
                      }}
                    >
                      <li>Personal information</li>
                      <li>Widget configurations</li>
                      <li>File contents or paths</li>
                    </ul>
                  </Box>
                </Table.Cell>
              </Table.Row>
            )}
          </SectionTable>
        </Flex>
      </Box>
    </ScrollArea>
  );
});

export default Settings;
