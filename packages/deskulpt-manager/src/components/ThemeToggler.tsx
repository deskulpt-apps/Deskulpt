import { Box, IconButton } from "@radix-ui/themes";
import { deskulptSettings } from "@deskulpt/bindings";
import { LuMoon, LuSun } from "react-icons/lu";
import { logger } from "@deskulpt/utils";

interface ThemeTogglerProps {
  theme: deskulptSettings.Theme;
}

const ThemeToggler = ({ theme }: ThemeTogglerProps) => {
  return (
    <Box position="absolute" right="3" top="4">
      <IconButton
        title="Toggle theme"
        variant="soft"
        size="1"
        onClick={() => {
          deskulptSettings.commands
            .update({ theme: theme === "light" ? "dark" : "light" })
            .catch(logger.error);
        }}
      >
        {theme === "light" ? <LuSun /> : <LuMoon />}
      </IconButton>
    </Box>
  );
};

export default ThemeToggler;
