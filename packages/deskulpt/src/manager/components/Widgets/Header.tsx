import { Badge, Button, Flex } from "@radix-ui/themes";
import { memo, useCallback } from "react";
import { LuFolderOpen, LuRepeat } from "react-icons/lu";
import { useWidgetsStore } from "../../hooks";
import { commands } from "../../../bindings";

interface HeaderProps {
  id: string;
}

const Header = memo(({ id }: HeaderProps) => {
  const type = useWidgetsStore((state) => state[id]?.type);

  const refreshAction = useCallback(() => {
    commands.core.bundleWidgets([id]).catch(console.error);
  }, [id]);

  const openAction = useCallback(() => {
    commands.core.openWidget(id).catch(console.error);
  }, [id]);

  return (
    <Flex align="center" justify="between">
      <Badge color={type === "ok" ? "gray" : "red"}>ID: {id}</Badge>
      <Flex align="center" gap="2">
        <Button
          title="Refresh this widget"
          size="1"
          variant="surface"
          onClick={refreshAction}
        >
          <LuRepeat /> Refresh
        </Button>
        <Button
          title="Open this widget folder"
          size="1"
          variant="surface"
          onClick={openAction}
        >
          <LuFolderOpen /> Edit
        </Button>
      </Flex>
    </Flex>
  );
});

export default Header;
