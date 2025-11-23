import { Box, Code, ScrollArea, Table } from "@radix-ui/themes";
import { useWidgetsStore } from "../../hooks";
import { memo } from "react";
import { css } from "@emotion/react";

const styles = {
  table: css({
    "--table-cell-padding": "var(--space-1) var(--space-2)",
    "--table-cell-min-height": 0,
    "& tr": { "--table-row-box-shadow": "none" },
    "& th": { color: "var(--gray-11)", width: "120px" },
  }),
};

interface ManifestProps {
  id: string;
}

const Manifest = memo(({ id }: ManifestProps) => {
  const manifest = useWidgetsStore((state) => state[id]);

  return (
    <ScrollArea asChild>
      <Box height="200px" pr="3" pb="3">
        {manifest?.type === "ok" ? (
          <Table.Root size="1" layout="fixed" css={styles.table}>
            <Table.Body>
              <Table.Row align="start">
                <Table.RowHeaderCell>Name</Table.RowHeaderCell>
                <Table.Cell>{manifest.content.name}</Table.Cell>
              </Table.Row>
              {manifest.content.version !== undefined && (
                <Table.Row align="start">
                  <Table.RowHeaderCell>Version</Table.RowHeaderCell>
                  <Table.Cell>{manifest.content.description}</Table.Cell>
                </Table.Row>
              )}
              {manifest.content.license !== undefined && (
                <Table.Row align="start">
                  <Table.RowHeaderCell>License</Table.RowHeaderCell>
                  <Table.Cell>{manifest.content.license}</Table.Cell>
                </Table.Row>
              )}
              {(manifest.content.authors ?? []).length > 0 && (
                <Table.Row align="start">
                  <Table.RowHeaderCell>Authors</Table.RowHeaderCell>
                  <Table.Cell>
                    {manifest.content.authors?.join(", ")}
                  </Table.Cell>
                </Table.Row>
              )}
              {manifest.content.description !== undefined && (
                <Table.Row align="start">
                  <Table.RowHeaderCell>Description</Table.RowHeaderCell>
                  <Table.Cell>{manifest.content.description}</Table.Cell>
                </Table.Row>
              )}
            </Table.Body>
          </Table.Root>
        ) : (
          <Box pl="2" m="0" asChild>
            <pre>
              <Code size="2" variant="ghost">
                {manifest?.content ?? "Widget not found."}
              </Code>
            </pre>
          </Box>
        )}
      </Box>
    </ScrollArea>
  );
});

export default Manifest;
