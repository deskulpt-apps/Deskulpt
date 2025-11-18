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
      TRACE: "#9f7aea",
    };
    return css({
      padding: "1px 6px",
      borderRadius: "999px",
      fontSize: "11px",
      fontWeight: 600,
      letterSpacing: "0.02em",
      textTransform: "uppercase",
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

const padNumber = (value: number) => String(value).padStart(2, "0");

const formatTimestamp = (timestamp: string): string => {
  const numeric = Number(timestamp);
  let date: Date | null = null;

  if (!Number.isNaN(numeric)) {
    const candidate = timestamp.length <= 10 ? numeric * 1000 : numeric;
    const parsed = new Date(candidate);
    if (!Number.isNaN(parsed.getTime())) {
      date = parsed;
    }
  }

  if (!date) {
    const parsed = new Date(timestamp);
    if (!Number.isNaN(parsed.getTime())) {
      date = parsed;
    }
  }

  if (date) {
    const year = date.getFullYear();
    const month = padNumber(date.getMonth() + 1);
    const day = padNumber(date.getDate());
    const hours = padNumber(date.getHours());
    const minutes = padNumber(date.getMinutes());
    const seconds = padNumber(date.getSeconds());
    return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
  }

  const snapshot = timestamp.match(
    /^(\d{4}-\d{2}-\d{2})[ T](\d{2}:\d{2}:\d{2})/,
  );
  if (snapshot) {
    return `${snapshot[1]} ${snapshot[2]}`;
  }

  return timestamp.slice(0, 19);
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
              {formatTimestamp(entry.timestamp)}
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
                whiteSpace: "pre-wrap",
                wordBreak: "break-word",
                overflowWrap: "anywhere",
                lineHeight: 1.4,
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
