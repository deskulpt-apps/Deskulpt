import { Box, Button, Flex, ScrollArea, TextField } from "@radix-ui/themes";
import { memo, useCallback, useEffect, useMemo, useState } from "react";
import { MdClear, MdFolderOpen, MdRefresh } from "react-icons/md";
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
    alignItems: "flex-end",
    gap: "16px",
  }),
  headerMeta: css({
    display: "flex",
    flexDirection: "column",
    gap: "4px",
    flex: 1,
    minWidth: 0,
  }),
  headerMetaLabel: css({
    fontSize: "12px",
    textTransform: "uppercase",
    letterSpacing: "0.04em",
    color: "var(--gray-11)",
  }),
  headerMetaInfo: css({
    fontSize: "12px",
    color: "var(--gray-11)",
  }),
  controlsGroup: css({
    display: "flex",
    gap: "8px",
    alignItems: "center",
  }),
  filters: css({
    display: "flex",
    gap: "12px",
    flexWrap: "wrap",
  }),
  levelSelect: css({
    width: "220px",
    padding: "8px 10px",
    borderRadius: "8px",
    border: "1px solid var(--gray-6)",
    backgroundColor: "var(--color-surface)",
    fontSize: "12px",
    color: "inherit",
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
  noLogsHint: css({
    fontSize: "12px",
    color: "var(--gray-11)",
  }),
};

const formatBytes = (bytes: number): string => {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
};

const LEVEL_OPTIONS = [
  "All",
  "Trace",
  "Debug",
  "Info",
  "Warn",
  "Error",
] as const;

const Logs = memo(() => {
  const [logFiles, setLogFiles] = useState<LogFileInfo[]>([]);
  const [selectedFile, setSelectedFile] = useState<string>("");
  const [logEntries, setLogEntries] = useState<LogEntry[]>([]);
  const [filterLevel, setFilterLevel] = useState<string>(LEVEL_OPTIONS[0]);
  const [filterText, setFilterText] = useState<string>("");
  const [loading, setLoading] = useState(false);

  const loadLogFiles = useCallback(async () => {
    try {
      setLoading(true);
      const files = await deskulptCore.commands.listLogs();
      setLogFiles(files);
      setSelectedFile(files[0]?.name ?? "");
    } catch (error) {
      console.error("Failed to load log files:", error);
      toast.error("Failed to load log files");
    } finally {
      setLoading(false);
    }
  }, []);

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

  const handleOpenLogsDir = useCallback(async () => {
    try {
      await deskulptCore.commands.openLogsDir();
    } catch (error) {
      console.error("Failed to open logs directory:", error);
      toast.error("Failed to open logs directory");
    }
  }, []);

  const filteredEntries = useMemo(() => {
    return logEntries.filter((entry) => {
      const normalizedFilter = filterLevel.toUpperCase();
      if (
        filterLevel &&
        filterLevel !== "All" &&
        entry.level.toUpperCase() !== normalizedFilter
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
  }, [logEntries, filterLevel, filterText]);

  const latestFile = logFiles[0];

  return (
    <Box css={styles.container}>
      <Box css={styles.header}>
        <div css={styles.headerMeta}>
          <span css={styles.headerMetaLabel}>Newest log</span>
          {logFiles.length > 0 && latestFile ? (
            <>
              <strong>{latestFile.name}</strong>
              <span css={styles.headerMetaInfo}>
                {formatBytes(latestFile.size)} • {latestFile.modified}
              </span>
            </>
          ) : (
            <span css={styles.noLogsHint}>No log files yet.</span>
          )}
        </div>
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
            onClick={handleOpenLogsDir}
            disabled={logFiles.length === 0}
            aria-label="Open logs directory"
          >
            <MdFolderOpen size={16} />
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
        <select
          css={styles.levelSelect}
          value={filterLevel}
          onChange={(e) => setFilterLevel(e.currentTarget.value)}
        >
          {LEVEL_OPTIONS.map((option) => (
            <option key={option} value={option}>
              {option}
            </option>
          ))}
        </select>
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
