import { create } from "zustand";
import { deskulptCore } from "@deskulpt/bindings";
import { FC } from "react";

interface WidgetProps extends deskulptCore.WidgetSettings {
  id: string;
}

interface WidgetState {
  component: FC<WidgetProps>;
  apisBlobUrl: string;
  moduleBlobUrl?: string;
}

export const useWidgetsStore = create<Record<string, WidgetState>>(() => ({}));
