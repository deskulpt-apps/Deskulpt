import { Box, IconButton } from "@radix-ui/themes";
import { deskulptSettings } from "@deskulpt/bindings";
import { MdOutlineDarkMode, MdOutlineLightMode } from "react-icons/md";
import { useCallback } from "react";

interface ThemeTogglerProps {
  theme: deskulptSettings.Theme;
}

const ThemeToggler = ({ theme }: ThemeTogglerProps) => {
  const toggleTheme = useCallback(() => {
    deskulptSettings.commands
      .update({ theme: theme === "light" ? "dark" : "light" })
      .catch(console.error);
  }, [theme]);

  return (
    <Box position="absolute" right="3" top="4">
      <IconButton
        title="Toggle theme"
        variant="soft"
        size="1"
        onClick={toggleTheme}
      >
        {theme === "light" ? <MdOutlineLightMode /> : <MdOutlineDarkMode />}
      </IconButton>
    </Box>
  );
};

export default ThemeToggler;
