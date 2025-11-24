import { memo } from "react";
import { css } from "@emotion/react";
import { deskulptCore } from "@deskulpt/bindings";

type LogEntry = deskulptCore.LogEntry;

const styles = {
  container: css({
    padding: "8px 0",
  }),
  logEntry: css({
    display: "flex",
    gap: "12px",
    padding: "4px 0",
    fontFamily: "monospace",
    fontSize: "12px",
    lineHeight: 1.5,
    borderBottom: "1px solid var(--gray-3)",
    "&:last-child": {
      borderBottom: "none",
    },
  }),
  timestamp: css({
    flexShrink: 0,
    width: "160px",
    color: "var(--gray-11)",
    fontSize: "11px",
  }),
  level: (level: string) => {
    const colors: Record<string, string> = {
      ERROR: "#ff6b6b",
      WARN: "#ffa500",
      INFO: "#4dabf7",
      DEBUG: "#a0aec0",
      TRACE: "#9f7aea",
    };
    return css({
      flexShrink: 0,
      width: "60px",
      fontSize: "11px",
      fontWeight: 600,
      textTransform: "uppercase",
      color: colors[level] || "#a0aec0",
    });
  },
  message: css({
    flex: 1,
    minWidth: 0,
    whiteSpace: "pre-wrap",
    wordBreak: "break-word",
    overflowWrap: "anywhere",
  }),
  noLogs: css({
    textAlign: "center",
    padding: "32px",
    color: "var(--gray-11)",
    fontSize: "12px",
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
  const numEntries = entries.length;
  if (numEntries === 0) {
    return (
      <div css={styles.noLogs}>
        {selectedFile ? "No matching log entries" : "No log files available"}
      </div>
    );
  }

  return (
    <div css={styles.container}>
      {Array.from(
        { length: numEntries },
        (_, index) => numEntries - index - 1,
      ).map((index) => {
        const entry = entries[index]!;
        return (
          <div
            key={`${entry.timestamp}-${entry.level}-${entry.message.slice(0, 20)}`}
            css={styles.logEntry}
            title={
              entry.fields
                ? `${entry.message}\n\nFields: ${entry.fields}`
                : entry.message
            }
          >
            <span css={styles.timestamp}>
              {formatTimestamp(entry.timestamp)}
            </span>
            <span css={styles.level(entry.level)}>{entry.level}</span>
            <span css={styles.message}>{entry.message}</span>
          </div>
        );
      })}
    </div>
  );
});

LogViewer.displayName = "LogViewer";

export default LogViewer;
