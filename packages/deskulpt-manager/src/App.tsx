import { Box, Flex, Theme as RadixTheme, Tabs } from "@radix-ui/themes";
import { Toaster } from "sonner";
import {
  useInitialRescan,
  useSettingsStore,
  useUpdateSettingsListener,
  useUpdateWidgetCatalogListener,
} from "./hooks";
import About from "./components/About";
import Widgets from "./components/Widgets";
import Settings from "./components/Settings";
import ThemeToggler from "./components/ThemeToggler";

const tabs = [
  { value: "widgets", label: "Widgets", content: <Widgets /> },
  { value: "settings", label: "Settings", content: <Settings /> },
  { value: "about", label: "About", content: <About /> },
];

const App = () => {
  const theme = useSettingsStore((state) => state.theme);

  useInitialRescan();
  useUpdateSettingsListener();
  useUpdateWidgetCatalogListener();

  return (
    <RadixTheme appearance={theme} accentColor="indigo" grayColor="slate">
      <Toaster
        position="bottom-center"
        theme={theme}
        gap={6}
        toastOptions={{
          style: {
            color: "var(--gray-12)",
            borderColor: "var(--gray-6)",
            backgroundColor: "var(--gray-2)",
            padding: "var(--space-2) var(--space-4)",
          },
        }}
      />
      <ThemeToggler theme={theme} />
      <Tabs.Root defaultValue="widgets" asChild>
        <Flex direction="column" gap="2" height="100%" p="2">
          <Tabs.List>
            {tabs.map((tab) => (
              <Tabs.Trigger key={tab.value} value={tab.value}>
                {tab.label}
              </Tabs.Trigger>
            ))}
          </Tabs.List>
          <Box p="1" height="calc(100% - var(--space-8))">
            {tabs.map((tab) => (
              <Tabs.Content key={tab.value} value={tab.value} asChild>
                <Box height="100%">{tab.content}</Box>
              </Tabs.Content>
            ))}
          </Box>
        </Flex>
      </Tabs.Root>
    </RadixTheme>
  );
};

export default App;
