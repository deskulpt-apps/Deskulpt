import { Box, Button, DropdownMenu } from "@radix-ui/themes";
import { LuDownload } from "react-icons/lu";
import { DeskulptWidgets } from "@deskulpt/bindings";
import { useInstallWidget } from "../../hooks";

interface WidgetPrimaryActionsProps {
  reference: DeskulptWidgets.WidgetReference;
  version: string;
}

const WidgetPrimaryActions = ({
  reference,
  version,
}: WidgetPrimaryActionsProps) => {
  const { status, isInFlight, install, uninstall, upgrade } = useInstallWidget(
    reference,
    version,
  );

  return (
    <Box>
      {status === "not-installed" && (
        <Button
          size="1"
          variant="surface"
          loading={isInFlight}
          onClick={install}
        >
          <LuDownload /> Install
        </Button>
      )}

      {status === "installed" && (
        <Button
          size="1"
          variant="surface"
          loading={isInFlight}
          onClick={uninstall}
        >
          Uninstall
        </Button>
      )}

      {status === "upgrade-available" && (
        <DropdownMenu.Root>
          <DropdownMenu.Trigger>
            <Button size="1" variant="surface" loading={isInFlight}>
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
            <DropdownMenu.Item disabled={isInFlight} onClick={upgrade}>
              Upgrade
            </DropdownMenu.Item>
            <DropdownMenu.Item disabled={isInFlight} onClick={uninstall}>
              Uninstall
            </DropdownMenu.Item>
          </DropdownMenu.Content>
        </DropdownMenu.Root>
      )}
    </Box>
  );
};

export default WidgetPrimaryActions;
