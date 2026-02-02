import { DeskulptWidgets } from "@deskulpt/bindings";
import { logger } from "@deskulpt/utils";
import { DropdownMenu, Flex, IconButton } from "@radix-ui/themes";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { useState } from "react";
import { LuCopy, LuDownload, LuEllipsis, LuEye } from "react-icons/lu";
import { toast } from "sonner";
import { useWidgetsGalleryStore } from "../../hooks";

interface WidgetSecondaryActionsProps {
  reference: DeskulptWidgets.WidgetReference;
  version: string;
  releases: DeskulptWidgets.IndexEntryRelease[];
}

const WidgetSecondaryActions = ({
  reference,
  version,
  releases,
}: WidgetSecondaryActionsProps) => {
  const [isLoadingPreview, setIsLoadingPreview] = useState(false);
  const openPreview = useWidgetsGalleryStore((state) => state.openPreview);
  const openVersionPicker = useWidgetsGalleryStore(
    (state) => state.openVersionPicker,
  );

  const id = `@${reference.handle}.${reference.id}`;

  const preview = async () => {
    setIsLoadingPreview(true);
    try {
      const previewData = await DeskulptWidgets.Commands.preview(reference);
      openPreview({ reference, version, preview: previewData });
    } catch (error) {
      logger.error(error);
      toast.error("Failed to load preview.");
    } finally {
      setIsLoadingPreview(false);
    }
  };

  const pickVersion = () => {
    openVersionPicker({
      handle: reference.handle,
      id: reference.id,
      releases,
    });
  };

  const copyWidgetId = () => {
    writeText(id).then(() => toast.success("Copied to clipboard."));
  };

  return (
    <Flex align="center" gap="3" pr="1">
      <IconButton
        size="1"
        variant="ghost"
        onClick={preview}
        loading={isLoadingPreview}
      >
        <LuEye size={16} />
      </IconButton>

      <DropdownMenu.Root>
        <DropdownMenu.Trigger>
          <IconButton size="1" variant="ghost">
            <LuEllipsis size={16} />
          </IconButton>
        </DropdownMenu.Trigger>
        <DropdownMenu.Content size="1" variant="soft" color="gray" align="end">
          <DropdownMenu.Item onClick={copyWidgetId}>
            <LuCopy /> Copy widget ID
          </DropdownMenu.Item>
          <DropdownMenu.Item
            disabled={releases.length <= 1}
            onClick={pickVersion}
          >
            <LuDownload /> Install another version
          </DropdownMenu.Item>
        </DropdownMenu.Content>
      </DropdownMenu.Root>
    </Flex>
  );
};

export default WidgetSecondaryActions;
