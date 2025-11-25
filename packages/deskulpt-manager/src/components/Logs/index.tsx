import { Button, ScrollArea, TextField } from "@radix-ui/themes";
import { memo, useCallback, useEffect, useMemo, useState } from "react";
import { MdFolderOpen } from "react-icons/md";
import { css } from "@emotion/react";
import { deskulptCore } from "@deskulpt/bindings";
import { toast } from "sonner";
import LogViewer from "./LogViewer";
import { logger } from "@deskulpt/utils";

type LogFileInfo = deskulptCore.LogFileInfo;
type LogEntry = deskulptCore.LogEntry;

const styles = {
  container: css({
    height: "100%",
    display: "flex",
    flexDirection: "column",
    gap: "8px",
    padding: "12px",
  }),
  header: css({
    display: "flex",
    justifyContent: "space-between",
    alignItems: "center",
    gap: "12px",
    paddingBottom: "8px",
  }),
  headerMeta: css({
    display: "flex",
    flexDirection: "column",
    gap: "2px",
    flex: 1,
    minWidth: 0,
  }),
  headerMetaLabel: css({
    fontSize: "11px",
    color: "var(--gray-11)",
  }),
  headerMetaInfo: css({
    fontSize: "11px",
    color: "var(--gray-11)",
  }),
  filters: css({
    display: "flex",
    gap: "8px",
    flexWrap: "wrap",
  }),
  textField: css({
    boxShadow: "none !important",
    border: "none !important",
    backgroundColor: "transparent !important",
    "& > div": {
      boxShadow: "none !important",
      backgroundColor: "transparent !important",
    },
    "& input": {
      background: "transparent !important",
      backgroundImage: "none !important",
      border: "0.5px solid var(--gray-8) !important",
      borderRadius: "4px !important",
      padding: "6px 8px !important",
      fontSize: "12px !important",
      lineHeight: "1.5 !important",
      height: "auto !important",
      minHeight: "0 !important",
      boxSizing: "border-box",
      boxShadow: "none !important",
      color: "inherit !important",
    },
    "& input:hover": {
      border: "0.5px solid var(--gray-8) !important",
      boxShadow: "none !important",
    },
    "& input:focus": {
      border: "1px solid var(--gray-8) !important",
      boxShadow: "none !important",
      outline: "none !important",
    },
  }),
  levelSelectWrapper: css({
    position: "relative",
    width: "180px",
  }),
  levelSelect: css({
    width: "100%",
    padding: "6px 28px 6px 8px",
    fontSize: "12px !important",
    lineHeight: "1.5 !important",
    color: "inherit !important",
    backgroundColor: "transparent !important",
    backgroundImage: "none !important",
    border: "0.5px solid var(--gray-8) !important",
    borderRadius: "4px !important",
    outline: "none !important",
    appearance: "none",
    WebkitAppearance: "none",
    MozAppearance: "none",
    cursor: "pointer",
    boxSizing: "border-box",
    boxShadow: "none !important",
    "&:hover": {
      border: "0.5px solid var(--gray-8) !important",
      boxShadow: "none !important",
    },
    "&:focus": {
      outline: "none !important",
      border: "1px solid var(--gray-8) !important",
      boxShadow: "none !important",
    },
  }),
  selectChevron: css({
    position: "absolute",
    right: "8px",
    top: "50%",
    transform: "translateY(-50%)",
    pointerEvents: "none",
    color: "var(--gray-11)",
    fontSize: "10px",
  }),
  viewer: css({
    flex: 1,
    overflow: "hidden",
    display: "flex",
    flexDirection: "column",
  }),
  footer: css({
    padding: "4px 0",
    fontSize: "11px",
    color: "var(--gray-11)",
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

  const loadLogFiles = useCallback(async () => {
    try {
      const files = await deskulptCore.commands.listLogs();
      setLogFiles(files);
      setSelectedFile(files[0]?.name ?? "");
    } catch (error) {
      logger.error("Failed to load log files", { error });
      toast.error("Failed to load log files");
    }
  }, []);

  const loadLogEntries = useCallback(async (filename: string) => {
    try {
      const entries = await deskulptCore.commands.readLog(filename, 1000);
      setLogEntries(entries);
    } catch (error) {
      logger.error("Failed to load log entries", { error });
      toast.error("Failed to load log entries");
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

  const handleOpenLogsDir = useCallback(async () => {
    try {
      await deskulptCore.commands.openLogsDir();
    } catch (error) {
      logger.error("Failed to open logs directory", { error });
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
    <div css={styles.container}>
      <div css={styles.header}>
        <div css={styles.headerMeta}>
          {logFiles.length > 0 && latestFile ? (
            <>
              <span css={styles.headerMetaLabel}>{latestFile.name}</span>
              <span css={styles.headerMetaInfo}>
                {formatBytes(latestFile.size)}
              </span>
            </>
          ) : (
            <span css={styles.noLogsHint}>No log files yet.</span>
          )}
        </div>
        <Button
          size="1"
          variant="ghost"
          onClick={handleOpenLogsDir}
          disabled={logFiles.length === 0}
          aria-label="Open logs directory"
        >
          <MdFolderOpen size={16} />
        </Button>
      </div>

      <div css={styles.filters}>
        <TextField.Root
          placeholder="Filter by message..."
          value={filterText}
          onChange={(e) => setFilterText(e.currentTarget.value)}
          variant="surface"
          css={styles.textField}
          style={{ flex: 1 }}
        />
        <div css={styles.levelSelectWrapper}>
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
          <span css={styles.selectChevron}>▼</span>
        </div>
      </div>

      <div css={styles.viewer}>
        <ScrollArea style={{ flex: 1 }}>
          <LogViewer selectedFile={selectedFile} entries={filteredEntries} />
        </ScrollArea>
        <div css={styles.footer}>
          <span>
            {filteredEntries.length} of {logEntries.length} entries
          </span>
        </div>
      </div>
    </div>
  );
});

Logs.displayName = "Logs";

export default Logs;
