import { create } from "zustand";
import { FC } from "react";

interface WidgetProps {
  id: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

interface WidgetState {
  component: FC<WidgetProps>;
  apisBlobUrl: string;
  moduleBlobUrl?: string;
}

export const useWidgetsStore = create<Record<string, WidgetState>>(() => ({}));
