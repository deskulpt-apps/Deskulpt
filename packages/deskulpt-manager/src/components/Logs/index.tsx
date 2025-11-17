import { Box, Button, Flex, ScrollArea, TextField } from "@radix-ui/themes";
import { memo, useCallback, useEffect, useState } from "react";
import { MdClear, MdDownload, MdRefresh } from "react-icons/md";
import { css } from "@emotion/react";
import { deskulptCore } from "@deskulpt/bindings";
import { toast } from "sonner";
import LogViewer from "./LogViewer";

type LogFileInfo = deskulptCore.LogFileInfo;
type LogEntry = deskulptCore.LogEntry;

const styles = {
  container: css({
    height: "100%",
    display: "flex",
    flexDirection: "column",
    gap: "12px",
    padding: "16px",
  }),
  header: css({
    display: "flex",
    justifyContent: "space-between",
    alignItems: "center",
  }),
  title: css({
    fontSize: "16px",
    fontWeight: 600,
  }),
  controlsGroup: css({
    display: "flex",
    gap: "8px",
    alignItems: "center",
  }),
  filters: css({
    display: "flex",
    gap: "12px",
  }),
  fileTray: css({
    borderRadius: "8px",
    border: "1px solid var(--gray-6)",
    padding: "8px",
    backgroundColor: "var(--gray-2)",
  }),
  fileRow: css({
    display: "flex",
    gap: "8px",
  }),
  fileButton: (active: boolean) =>
    css({
      display: "flex",
      flexDirection: "column",
      minWidth: "160px",
      padding: "8px 10px",
      borderRadius: "8px",
      border: "1px solid",
      borderColor: active ? "var(--accent-9)" : "var(--gray-6)",
      backgroundColor: active ? "var(--accent-3)" : "var(--color-surface)",
      textAlign: "left",
      cursor: "pointer",
      fontSize: "12px",
      lineHeight: 1.4,
    }),
  fileMeta: css({
    fontSize: "11px",
    color: "var(--gray-11)",
    marginTop: "2px",
  }),
  viewer: css({
    flex: 1,
    border: "1px solid var(--gray-6)",
    borderRadius: "8px",
    overflow: "hidden",
    display: "flex",
    flexDirection: "column",
  }),
  footer: css({
    padding: "8px 12px",
    fontSize: "12px",
    color: "var(--gray-11)",
    borderTop: "1px solid var(--gray-6)",
    display: "flex",
    justifyContent: "space-between",
  }),
};

const formatFields = (fields: LogEntry["fields"]) => {
  if (!fields) {
    return "";
  }
  try {
    return JSON.stringify(fields);
  } catch {
    return String(fields);
  }
};

const formatBytes = (bytes: number): string => {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
};

const Logs = memo(() => {
  const [logFiles, setLogFiles] = useState<LogFileInfo[]>([]);
  const [selectedFile, setSelectedFile] = useState<string>("");
  const [logEntries, setLogEntries] = useState<LogEntry[]>([]);
  const [filterLevel, setFilterLevel] = useState<string>("");
  const [filterText, setFilterText] = useState<string>("");
  const [loading, setLoading] = useState(false);

  const loadLogFiles = useCallback(async () => {
    try {
      setLoading(true);
      const files = await deskulptCore.commands.listLogs();
      setLogFiles(files);
      if (files.length > 0 && !selectedFile) {
        setSelectedFile(files[0]?.name ?? "");
      }
    } catch (error) {
      console.error("Failed to load log files:", error);
      toast.error("Failed to load log files");
    } finally {
      setLoading(false);
    }
  }, [selectedFile]);

  const loadLogEntries = useCallback(async (filename: string) => {
    try {
      setLoading(true);
      const entries = await deskulptCore.commands.readLog(filename, 1000);
      setLogEntries(entries);
    } catch (error) {
      console.error("Failed to load log entries:", error);
      toast.error("Failed to load log entries");
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void loadLogFiles();
  }, [loadLogFiles]);

  useEffect(() => {
    if (selectedFile) {
      void loadLogEntries(selectedFile);
    }
  }, [selectedFile, loadLogEntries]);

  const handleClearLogs = useCallback(async () => {
    if (!confirm("Are you sure you want to delete all log files?")) {
      return;
    }
    try {
      await deskulptCore.commands.clearLogs();
      toast.success("Log files cleared");
      setSelectedFile("");
      setLogEntries([]);
      void loadLogFiles();
    } catch (error) {
      console.error("Failed to clear logs:", error);
      toast.error("Failed to clear logs");
    }
  }, [loadLogFiles]);

  const filteredEntries = logEntries.filter((entry) => {
    if (
      filterLevel &&
      !entry.level.toUpperCase().includes(filterLevel.toUpperCase())
    ) {
      return false;
    }
    if (
      filterText &&
      !entry.message.toLowerCase().includes(filterText.toLowerCase())
    ) {
      return false;
    }
    return true;
  });

  const handleExportLogs = useCallback(() => {
    const csv = [
      ["Timestamp", "Level", "Message", "Fields"],
      ...filteredEntries.map((entry) => [
        entry.timestamp,
        entry.level,
        entry.message,
        formatFields(entry.fields),
      ]),
    ]
      .map((row) =>
        row.map((cell) => `"${String(cell).replaceAll('"', '""')}"`).join(","),
      )
      .join("\n");

    const blob = new Blob([csv], { type: "text/csv" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `logs-${new Date().toISOString()}.csv`;
    a.click();
    URL.revokeObjectURL(url);
  }, [filteredEntries]);

  return (
    <Box css={styles.container}>
      <Box css={styles.header}>
        <div css={styles.title}>Application Logs ({logFiles.length} files)</div>
        <Flex css={styles.controlsGroup}>
          <Button
            size="1"
            onClick={() => void loadLogFiles()}
            disabled={loading}
          >
            <MdRefresh size={16} />
          </Button>
          <Button
            size="1"
            onClick={handleExportLogs}
            disabled={filteredEntries.length === 0}
          >
            <MdDownload size={16} />
          </Button>
          <Button
            size="1"
            color="red"
            onClick={handleClearLogs}
            disabled={logFiles.length === 0}
          >
            <MdClear size={16} />
          </Button>
        </Flex>
      </Box>

      <div css={styles.filters}>
        <TextField.Root
          placeholder="Filter by message..."
          value={filterText}
          onChange={(e) => setFilterText(e.currentTarget.value)}
          style={{ flex: 1 }}
        />
        <TextField.Root
          placeholder="Filter by level (ERROR, WARN, INFO, DEBUG)"
          value={filterLevel}
          onChange={(e) => setFilterLevel(e.currentTarget.value)}
          style={{ width: "220px" }}
        />
      </div>

      <div css={styles.fileTray}>
        <ScrollArea>
          <div css={styles.fileRow}>
            {logFiles.length === 0 ? (
              <span style={{ fontSize: "12px", color: "var(--gray-11)" }}>
                No log files yet.
              </span>
            ) : (
              logFiles.map((file) => (
                <button
                  key={file.name}
                  css={styles.fileButton(selectedFile === file.name)}
                  onClick={() => setSelectedFile(file.name)}
                >
                  <strong>{file.name}</strong>
                  <span css={styles.fileMeta}>
                    {formatBytes(file.size)} • {file.modified}
                  </span>
                </button>
              ))
            )}
          </div>
        </ScrollArea>
      </div>

      <div css={styles.viewer}>
        <ScrollArea style={{ flex: 1 }}>
          <LogViewer selectedFile={selectedFile} entries={filteredEntries} />
        </ScrollArea>
        <div css={styles.footer}>
          <span>
            Showing {filteredEntries.length} of {logEntries.length} entries
          </span>
          {selectedFile && <span>{selectedFile}</span>}
        </div>
      </div>
    </Box>
  );
});

Logs.displayName = "Logs";

export default Logs;
