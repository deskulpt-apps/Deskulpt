import { create } from "zustand";
import { FC } from "react";
import { DeskulptWidgets } from "@deskulpt/bindings";

interface WidgetProps {
  id: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

interface WidgetState {
  settings?: DeskulptWidgets.WidgetSettings;
  component?: FC<WidgetProps>;
  apisBlobUrl?: string;
  moduleBlobUrl?: string;
}

export const useWidgetsStore = create<Record<string, WidgetState>>(() => ({}));
