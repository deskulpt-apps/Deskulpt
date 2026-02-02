import { Box, Flex, Theme as RadixTheme, Tabs } from "@radix-ui/themes";
import { Toaster } from "sonner";
import About from "./components/About";
import Widgets from "./components/Widgets";
import Settings from "./components/Settings";
import ThemeToggler from "./components/ThemeToggler";
import Gallery from "./components/Gallery";
import Logs from "./components/Logs";
import { SettingsStoreProvider, WidgetsStoreProvider } from "./hooks";
import LoadingScreen from "./components/LoadingScreen";

const tabs = [
  {
    value: "widgets",
    label: "Widgets",
    render: () => (
      <WidgetsStoreProvider fallback={<LoadingScreen />}>
        <Widgets />
      </WidgetsStoreProvider>
    ),
  },
  {
    value: "settings",
    label: "Settings",
    render: () => (
      <SettingsStoreProvider fallback={<LoadingScreen />}>
        <Settings />
      </SettingsStoreProvider>
    ),
  },
  {
    value: "gallery",
    label: "Gallery",
    render: () => (
      <WidgetsStoreProvider fallback={<LoadingScreen />}>
        <Gallery />
      </WidgetsStoreProvider>
    ),
  },
  { value: "logs", label: "Logs", render: () => <Logs /> },
  { value: "about", label: "About", render: () => <About /> },
];

const App = () => {
  const theme = window.__DESKULPT_INTERNALS__.initialSettings.theme;

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
            {tabs.map(({ value, label }) => (
              <Tabs.Trigger key={value} value={value}>
                {label}
              </Tabs.Trigger>
            ))}
          </Tabs.List>
          <Box p="1" height="calc(100% - var(--space-8))">
            {tabs.map(({ value, render }) => (
              <Tabs.Content key={value} value={value} asChild>
                <Box height="100%">{render()}</Box>
              </Tabs.Content>
            ))}
          </Box>
        </Flex>
      </Tabs.Root>
    </RadixTheme>
  );
};

export default App;
