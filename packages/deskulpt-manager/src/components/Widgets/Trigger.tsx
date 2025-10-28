import { Box, Flex, Tabs, Text } from "@radix-ui/themes";
import { useWidgetsStore } from "../../hooks";
import { memo } from "react";
import { css } from "@emotion/react";

const styles = {
  trigger: css({
    justifyContent: "start",
    height: "35px",
    // Move the active bar indicator from bottom to left
    "&::before": {
      top: "10%",
      bottom: "10%",
      left: 0,
      right: "unset",
      height: "unset",
      width: "2px",
    },
  }),
  indicator: css({
    borderRadius: "var(--radius-thumb)",
    backgroundColor: "var(--green-10)",
  }),
  indicatorInvalid: css({ backgroundColor: "var(--red-10)" }),
};

interface TriggerProps {
  id: string;
  value: string;
}

const Trigger = memo(({ id, value }: TriggerProps) => {
  const config = useWidgetsStore((state) => state[id]);

  return (
    <Tabs.Trigger value={value} css={styles.trigger}>
      <Flex align="center" gap="3">
        <Box
          width="6px"
          height="6px"
          css={[
            styles.indicator,
            config?.type !== "ok" && styles.indicatorInvalid,
          ]}
        />
        <Text>{id}</Text>
      </Flex>
    </Tabs.Trigger>
  );
});

export default Trigger;
