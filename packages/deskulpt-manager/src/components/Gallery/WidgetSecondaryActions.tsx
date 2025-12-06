import { deskulptWidgets } from "@deskulpt/bindings";
import { DropdownMenu, Flex, IconButton } from "@radix-ui/themes";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { useCallback, useState } from "react";
import { LuCopy, LuEllipsis, LuExternalLink } from "react-icons/lu";
import { toast } from "sonner";

interface WidgetSecondaryActionsProps {
  widget: deskulptWidgets.RegistryWidgetReference;
  showPreview: (preview: deskulptWidgets.RegistryWidgetPreview) => void;
}

const WidgetSecondaryActions = ({
  widget,
  showPreview,
}: WidgetSecondaryActionsProps) => {
  const [isLoadingPreview, setIsLoadingPreview] = useState(false);

  const id = `@${widget.handle}.${widget.id}`;

  const preview = useCallback(async () => {
    setIsLoadingPreview(true);
    try {
      const previewData = await deskulptWidgets.commands.preview(widget);
      showPreview(previewData);
    } catch (error) {
      logger.error(error);
      toast.error("Failed to load preview.");
    } finally {
      setIsLoadingPreview(false);
    }
  }, [widget, showPreview]);

  const copyWidgetId = useCallback(() => {
    writeText(id).then(() => toast.success("Copied to clipboard."));
  }, [id]);

  return (
    <Flex align="center" gap="3" pr="1">
      <IconButton
        size="1"
        variant="ghost"
        onClick={preview}
        loading={isLoadingPreview}
      >
        <LuExternalLink size="16" />
      </IconButton>

      <DropdownMenu.Root>
        <DropdownMenu.Trigger>
          <IconButton size="1" variant="ghost">
            <LuEllipsis size="16" />
          </IconButton>
        </DropdownMenu.Trigger>
        <DropdownMenu.Content size="1" variant="soft" color="gray" align="end">
          <DropdownMenu.Item onClick={copyWidgetId}>
            <LuCopy /> Copy widget ID
          </DropdownMenu.Item>
        </DropdownMenu.Content>
      </DropdownMenu.Root>
    </Flex>
  );
};

export default WidgetSecondaryActions;
