import { create } from "zustand";
import { FC } from "react";
import { WidgetSettings } from "@deskulpt/bindings/src/deskulpt-widgets";

interface WidgetProps {
  id: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

interface WidgetState {
  settings: WidgetSettings;
  component?: FC<WidgetProps>;
  apisBlobUrl?: string;
  moduleBlobUrl?: string;
}

export const useWidgetsStore = create<Record<string, WidgetState>>(() => ({}));
