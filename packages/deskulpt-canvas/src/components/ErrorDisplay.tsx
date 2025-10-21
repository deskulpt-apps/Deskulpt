import { css } from "@emotion/react";
import { Box, Code, Dialog, ScrollArea, Text } from "@radix-ui/themes";
import { memo } from "react";

const styles = {
  trigger: css({ cursor: "pointer" }),
};

interface ErrorDisplayProps {
  id: string;
  error: string;
  message: string;
}

const ErrorDisplay = memo(({ id, error, message }: ErrorDisplayProps) => {
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
      </Dialog.Content>
    </Dialog.Root>
  );
});

export default ErrorDisplay;
