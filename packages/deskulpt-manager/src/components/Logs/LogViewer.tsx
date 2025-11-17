import { Box, Table } from "@radix-ui/themes";
import { memo } from "react";
import { css } from "@emotion/react";
import { deskulptCore } from "@deskulpt/bindings";

type LogEntry = deskulptCore.LogEntry;

const styles = {
  levelBadge: (level: string) => {
    const colors: Record<string, string> = {
      ERROR: "#ff6b6b",
      WARN: "#ffa500",
      INFO: "#4dabf7",
      DEBUG: "#a0aec0",
    };
    return css({
      padding: "2px 8px",
      borderRadius: "4px",
      fontSize: "12px",
      fontWeight: "bold",
      backgroundColor: colors[level] || "#a0aec0",
      color: "white",
    });
  },
  logRow: css({
    fontFamily: "monospace",
    fontSize: "12px",
  }),
  noLogs: css({
    textAlign: "center",
    padding: "32px",
    color: "var(--gray-11)",
  }),
};

interface LogViewerProps {
  selectedFile: string;
  entries: LogEntry[];
}

const LogViewer = memo(({ selectedFile, entries }: LogViewerProps) => {
  if (entries.length === 0) {
    return (
      <Box css={styles.noLogs}>
        {selectedFile ? "No matching log entries" : "No log files available"}
      </Box>
    );
  }

  return (
    <Table.Root>
      <Table.Header>
        <Table.Row>
          <Table.ColumnHeaderCell>Timestamp</Table.ColumnHeaderCell>
          <Table.ColumnHeaderCell width="80px">Level</Table.ColumnHeaderCell>
          <Table.ColumnHeaderCell>Message</Table.ColumnHeaderCell>
        </Table.Row>
      </Table.Header>
      <Table.Body>
        {entries.map((entry) => (
          <Table.Row
            key={`${entry.timestamp}-${entry.level}`}
            css={styles.logRow}
          >
            <Table.Cell style={{ fontSize: "11px", fontFamily: "monospace" }}>
              {entry.timestamp}
            </Table.Cell>
            <Table.Cell>
              <span css={styles.levelBadge(entry.level)}>{entry.level}</span>
            </Table.Cell>
            <Table.Cell
              title={
                entry.fields
                  ? `${entry.message}\n\nFields: ${entry.fields}`
                  : entry.message
              }
              style={{
                maxWidth: "400px",
                overflow: "hidden",
                textOverflow: "ellipsis",
                whiteSpace: "nowrap",
              }}
            >
              {entry.message}
            </Table.Cell>
          </Table.Row>
        ))}
      </Table.Body>
    </Table.Root>
  );
});

LogViewer.displayName = "LogViewer";

export default LogViewer;
