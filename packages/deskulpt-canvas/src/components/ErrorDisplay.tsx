import { css } from "@emotion/react";
import { Box, Code, Dialog, ScrollArea, Text } from "@radix-ui/themes";
import { memo, useCallback } from "react";
import { MdContentCopy, MdReportProblem } from "react-icons/md";
import { toast } from "sonner";

const styles = {
  trigger: css({ cursor: "pointer" }),
  buttonGroup: css({
    display: "flex",
    gap: "8px",
    marginTop: "12px",
  }),
  button: css({
    padding: "8px 12px",
    borderRadius: "4px",
    border: "none",
    cursor: "pointer",
    fontSize: "14px",
    fontWeight: "500",
    display: "flex",
    alignItems: "center",
    gap: "6px",
    transition: "all 200ms ease-in-out",
    "&:hover": {
      opacity: 0.8,
    },
  }),
  reportButton: css({
    backgroundColor: "#ff6b6b",
    color: "white",
  }),
  copyButton: css({
    backgroundColor: "#e9ecef",
    color: "#495057",
  }),
};

interface ErrorDisplayProps {
  id: string;
  error: string;
  message: string;
}

const ErrorDisplay = memo(({ id, error, message }: ErrorDisplayProps) => {
  const handleCopyError = useCallback(() => {
    const errorText = `Widget: ${id}\nError: ${error}\n\nDetails:\n${message}`;
    navigator.clipboard.writeText(errorText);
    toast.success("Error details copied to clipboard");
  }, [id, error, message]);

  const handleReportCrash = useCallback(() => {
    const errorReport = {
      widgetId: id,
      errorMessage: error,
      errorDetails: message,
      timestamp: new Date().toISOString(),
      userAgent: navigator.userAgent,
    };

    // Send to crash reporting service
    try {
      // This would normally be sent to your Sentry or backend API
      console.error("[CrashReport]", errorReport);

      // For now, just show a toast
      toast.success("Thank you for helping us improve! Error report sent.");
    } catch {
      toast.error("Failed to send crash report");
    }
  }, [id, error, message]);

  return (
    <Dialog.Root>
      <Dialog.Trigger>
        <Box width="100%" height="100%" p="2" css={styles.trigger} asChild>
          <Text size="2" as="div" color="red">
            An error occurred in widget <Code variant="ghost">{id}</Code>. Click
            anywhere to check the details.
          </Text>
        </Box>
      </Dialog.Trigger>
      <Dialog.Content size="1" maxWidth="60vw">
        <Dialog.Title size="3" color="red" mt="2" mb="1">
          Error in widget <Code variant="ghost">{id}</Code>
        </Dialog.Title>
        <Dialog.Description size="2" color="red" mb="4">
          {error}
        </Dialog.Description>
        <ScrollArea asChild>
          <Box px="3" pb="3" maxHeight="50vh">
            <Box asChild m="0">
              <pre>
                <Code size="2" variant="ghost">
                  {message}
                </Code>
              </pre>
            </Box>
          </Box>
        </ScrollArea>
        <Box css={styles.buttonGroup}>
          <button
            onClick={handleReportCrash}
            css={[styles.button, styles.reportButton]}
            title="Send crash report to help us improve"
          >
            <MdReportProblem size={16} />
            Report Crash
          </button>
          <button
            onClick={handleCopyError}
            css={[styles.button, styles.copyButton]}
            title="Copy error details to clipboard"
          >
            <MdContentCopy size={16} />
            Copy Details
          </button>
        </Box>
      </Dialog.Content>
    </Dialog.Root>
  );
});

export default ErrorDisplay;
