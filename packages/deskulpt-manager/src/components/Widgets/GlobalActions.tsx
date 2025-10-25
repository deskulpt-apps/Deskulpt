import { Flex, IconButton } from "@radix-ui/themes";
import { memo, useCallback } from "react";
import { LuFileScan, LuFolderOpen, LuRepeat } from "react-icons/lu";
import { deskulptCore, deskulptWidgets } from "@deskulpt/bindings";

interface GlobalActionsProps {
  length: number;
}

const GlobalActions = memo(({ length }: GlobalActionsProps) => {
  const refreshAction = useCallback(() => {
    deskulptWidgets.commands.bundle(null).catch(console.error);
  }, []);

  const rescanAction = useCallback(() => {
    deskulptWidgets.commands.rescan().catch(console.error);
  }, []);

  const openAction = useCallback(() => {
    deskulptCore.commands.openWidget(null).catch(console.error);
  }, []);

  return (
    <Flex gap="6" align="center" justify="center" pb="2" pr="4">
      <IconButton
        title="Refresh current widgets"
        size="1"
        variant="ghost"
        onClick={refreshAction}
        disabled={length === 0}
      >
        <LuRepeat size="16" />
      </IconButton>
      <IconButton
        title="Rescan widgets directory"
        size="1"
        variant="ghost"
        onClick={rescanAction}
      >
        <LuFileScan size="16" />
      </IconButton>
      <IconButton
        title="Open widgets directory"
        size="1"
        variant="ghost"
        onClick={openAction}
      >
        <LuFolderOpen size="16" />
      </IconButton>
    </Flex>
  );
});

export default GlobalActions;
