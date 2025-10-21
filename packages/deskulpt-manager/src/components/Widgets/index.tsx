import { Flex, ScrollArea, Separator, Tabs, Text } from "@radix-ui/themes";
import { useWidgetsStore } from "../../hooks";
import { memo } from "react";
import { useShallow } from "zustand/shallow";
import Trigger from "./Trigger";
import GlobalActions from "./GlobalActions";
import Config from "./Config";
import Settings from "./Settings";
import Header from "./Header";
import { css } from "@emotion/react";

const styles = {
  tabList: css({ width: "25%", height: "100%", boxShadow: "none" }),
  tabContent: css({ boxShadow: "inset 1px 0 0 0 var(--gray-a5)" }),
};

const WidgetsTab = memo(() => {
  const ids = useWidgetsStore(useShallow((state) => Object.keys(state)));

  return (
    <Tabs.Root orientation="vertical" defaultValue="tab0" asChild>
      <Flex height="100%">
        <Tabs.List css={styles.tabList}>
          <Flex direction="column" width="100%" gap="4">
            <ScrollArea scrollbars="vertical" asChild>
              <Flex direction="column">
                {ids.map((id, index) => (
                  <Trigger key={id} id={id} value={`tab${index}`} />
                ))}
              </Flex>
            </ScrollArea>
            <Separator size="4" />
            <GlobalActions length={ids.length} />
          </Flex>
        </Tabs.List>
        {ids.length === 0 ? (
          <Flex
            direction="column"
            align="center"
            justify="center"
            width="75%"
            css={styles.tabContent}
          >
            <Text size="2">No widgets available</Text>
          </Flex>
        ) : (
          ids.map((id, index) => (
            <Tabs.Content
              key={id}
              value={`tab${index}`}
              css={styles.tabContent}
              asChild
            >
              <Flex height="100%" direction="column" pl="2" gap="2" width="75%">
                <Header id={id} />
                <Config id={id} />
                <Separator size="4" />
                <Settings id={id} />
              </Flex>
            </Tabs.Content>
          ))
        )}
      </Flex>
    </Tabs.Root>
  );
});

export default WidgetsTab;
