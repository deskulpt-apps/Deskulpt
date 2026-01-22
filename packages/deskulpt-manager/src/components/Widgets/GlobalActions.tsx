import { Flex, IconButton } from "@radix-ui/themes";
import { LuFolderOpen, LuRepeat } from "react-icons/lu";
import { deskulptCore, deskulptWidgets } from "@deskulpt/bindings";
import { logger } from "@deskulpt/utils";

const GlobalActions = () => {
  return (
    <Flex gap="6" align="center" justify="center" pb="2" pr="4">
      <IconButton
        title="Refresh current widgets"
        size="1"
        variant="ghost"
        onClick={() =>
          deskulptWidgets.commands.refreshAll().catch(logger.error)
        }
      >
        <LuRepeat size={16} />
      </IconButton>
      <IconButton
        title="Open widgets directory"
        size="1"
        variant="ghost"
        onClick={() =>
          deskulptCore.commands.open("widgets").catch(logger.error)
        }
      >
        <LuFolderOpen size="16" />
      </IconButton>
    </Flex>
  );
};

export default GlobalActions;
