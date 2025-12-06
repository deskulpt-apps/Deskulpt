import { deskulptWidgets } from "@deskulpt/bindings";
import { logger } from "@deskulpt/utils";
import { Box, Button, DropdownMenu } from "@radix-ui/themes";
import { useCallback, useState } from "react";
import { LuDownload } from "react-icons/lu";
import { toast } from "sonner";
import { RegistryWidgetReference } from "@deskulpt/bindings/src/deskulpt-widgets";

interface WidgetPrimaryActionsProps {
  action: "install" | "uninstall" | "upgrade";
  widget: deskulptWidgets.RegistryWidgetReference;
  beforeAction?: () => void;
}

const WidgetPrimaryActions = ({
  action,
  widget,
}: WidgetPrimaryActionsProps) => {
  const [isBusy, setIsBusy] = useState(false);

  const execute = useCallback(
    (
      fn: (widget: RegistryWidgetReference) => Promise<null>,
      messages: { success: string; failure: string },
    ) => {
      return async () => {
        const id = `@${widget.handle}.${widget.id}`;
        setIsBusy(true);
        await new Promise((resolve) => setTimeout(resolve, 1000));
        try {
          await fn(widget);
          toast.success(`${messages.success}: ${id}`);
        } catch (error) {
          logger.error(error);
          toast.error(`${messages.failure}: ${id}`);
        } finally {
          setIsBusy(false);
        }
      };
    },
    [widget],
  );

  const install = execute(deskulptWidgets.commands.install, {
    success: "Installed",
    failure: "Failed to install",
  });

  const uninstall = execute(deskulptWidgets.commands.uninstall, {
    success: "Uninstalled",
    failure: "Failed to uninstall",
  });

  const upgrade = execute(deskulptWidgets.commands.upgrade, {
    success: "Upgraded",
    failure: "Failed to upgrade",
  });

  return (
    <Box>
      {action === "install" && (
        <Button size="1" variant="surface" loading={isBusy} onClick={install}>
          <LuDownload /> Install
        </Button>
      )}

      {action === "uninstall" && (
        <Button size="1" variant="surface" loading={isBusy} onClick={uninstall}>
          Uninstall
        </Button>
      )}

      {action === "upgrade" && (
        <DropdownMenu.Root>
          <DropdownMenu.Trigger>
            <Button size="1" variant="surface" loading={isBusy}>
              Upgrade
              <DropdownMenu.TriggerIcon />
            </Button>
          </DropdownMenu.Trigger>
          <DropdownMenu.Content
            size="1"
            variant="soft"
            color="gray"
            align="end"
          >
            <DropdownMenu.Item onClick={upgrade}>Upgrade</DropdownMenu.Item>
            <DropdownMenu.Item onClick={uninstall}>Uninstall</DropdownMenu.Item>
          </DropdownMenu.Content>
        </DropdownMenu.Root>
      )}
    </Box>
  );
};

export default WidgetPrimaryActions;
