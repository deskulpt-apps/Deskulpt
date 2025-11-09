import { create } from "zustand";
import { deskulptSettings } from "@deskulpt/bindings";
import { FC } from "react";

interface WidgetProps extends deskulptSettings.WidgetSettings {
  id: string;
}

interface WidgetState {
  component: FC<WidgetProps>;
  apisBlobUrl: string;
  moduleBlobUrl?: string;
}

export const useWidgetsStore = create<Record<string, WidgetState>>(() => ({}));
