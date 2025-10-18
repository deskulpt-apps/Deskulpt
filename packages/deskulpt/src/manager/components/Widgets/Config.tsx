import { Box, Code, ScrollArea, Table } from "@radix-ui/themes";
import { useWidgetsStore } from "../../hooks";
import { memo } from "react";
import Dependencies from "./Dependencies";
import { css } from "@emotion/react";

const styles = {
  table: css({
    "--table-cell-padding": "var(--space-1) var(--space-2)",
    "--table-cell-min-height": 0,
    "& tr": { "--table-row-box-shadow": "none" },
    "& th": { color: "var(--gray-11)", width: "120px" },
  }),
};

interface ConfigProps {
  id: string;
}

const Config = memo(({ id }: ConfigProps) => {
  const config = useWidgetsStore((state) => state[id]);

  return (
    <ScrollArea asChild>
      <Box height="200px" pr="3" pb="3">
        {config?.type === "ok" ? (
          <Table.Root size="1" layout="fixed" css={styles.table}>
            <Table.Body>
              <Table.Row align="center">
                <Table.RowHeaderCell>Name</Table.RowHeaderCell>
                <Table.Cell>{config.content.name}</Table.Cell>
              </Table.Row>
              <Table.Row align="center">
                <Table.RowHeaderCell>Entry</Table.RowHeaderCell>
                <Table.Cell>{config.content.entry}</Table.Cell>
              </Table.Row>
              <Table.Row align="center">
                <Table.RowHeaderCell>Dependencies</Table.RowHeaderCell>
                <Table.Cell>
                  <Dependencies dependencies={config.content.dependencies} />
                </Table.Cell>
              </Table.Row>
            </Table.Body>
          </Table.Root>
        ) : (
          <Box pl="2" m="0" asChild>
            <pre>
              <Code size="2" variant="ghost">
                {config?.content ?? "Widget not found."}
              </Code>
            </pre>
          </Box>
        )}
      </Box>
    </ScrollArea>
  );
});

export default Config;
