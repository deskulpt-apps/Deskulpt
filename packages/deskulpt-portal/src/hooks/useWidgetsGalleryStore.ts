import { create } from "zustand";
import { DeskulptWidgets } from "@deskulpt/bindings";
import { logger } from "@deskulpt/utils";
import { toast } from "sonner";

interface WidgetPreviewData {
  reference: DeskulptWidgets.RegistryWidgetReference;
  version: string;
  preview: DeskulptWidgets.RegistryWidgetPreview;
}

interface WidgetVersionPickerData {
  handle: string;
  id: string;
  releases: DeskulptWidgets.RegistryEntryRelease[];
}

interface WidgetsGalleryState {
  widgets: DeskulptWidgets.RegistryEntry[];
  isFetching: boolean;
  inFlightOps: Set<string>;

  isPreviewOpen: boolean;
  previewData?: WidgetPreviewData;

  isVersionPickerOpen: boolean;
  versionPickerData?: WidgetVersionPickerData;
}

interface WidgetsGalleryActions {
  refresh: () => Promise<void>;
  addInFlightOp: (id: string) => void;
  removeInFlightOp: (id: string) => void;

  openPreview: (data: WidgetPreviewData) => void;
  closePreview: () => void;

  openVersionPicker: (data: WidgetVersionPickerData) => void;
  closeVersionPicker: () => void;
}

export const useWidgetsGalleryStore = create<
  WidgetsGalleryState & WidgetsGalleryActions
>((set) => ({
  widgets: [],
  isFetching: false,
  inFlightOps: new Set(),
  isPreviewOpen: false,
  isVersionPickerOpen: false,

  refresh: async () => {
    set({ widgets: [], isFetching: true });
    try {
      const index = await DeskulptWidgets.Commands.fetchRegistryIndex();
      set({ widgets: index.widgets });
    } catch (error) {
      logger.error(error);
      toast.error("Failed to load widgets gallery");
    } finally {
      set({ isFetching: false });
    }
  },

  addInFlightOp: (id: string) => {
    set((state) => {
      const newSet = new Set([...state.inFlightOps, id]);
      return { inFlightOps: newSet };
    });
  },

  removeInFlightOp: (id: string) => {
    set((state) => {
      const newSet = new Set(state.inFlightOps);
      newSet.delete(id);
      return { inFlightOps: newSet };
    });
  },

  openPreview: (data) => {
    set({ isPreviewOpen: true, previewData: data });
  },

  closePreview: () => {
    set({ isPreviewOpen: false, previewData: undefined });
  },

  openVersionPicker: (data) => {
    set({ isVersionPickerOpen: true, versionPickerData: data });
  },

  closeVersionPicker: () => {
    set({ isVersionPickerOpen: false, versionPickerData: undefined });
  },
}));
