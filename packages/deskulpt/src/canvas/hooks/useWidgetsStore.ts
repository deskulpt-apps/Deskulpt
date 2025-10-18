import { create } from "zustand";
import { WidgetSettings } from "../../bindings";
import { FC } from "react";

interface WidgetProps extends WidgetSettings {
  id: string;
}

interface WidgetState {
  component: FC<WidgetProps>;
  apisBlobUrl: string;
  moduleBlobUrl?: string;
}

export const useWidgetsStore = create<Record<string, WidgetState>>(() => ({}));
