import { createElement, useEffect } from "react";
import { useWidgetsStore } from "./useWidgetsStore";
import { logger, stringify } from "@deskulpt/utils";
import { DeskulptWidgets } from "@deskulpt/bindings";
import ErrorDisplay from "../components/ErrorDisplay";

const BASE_URL = new URL(import.meta.url).origin;
const RAW_APIS_URL = new URL("/gen/raw-apis.js", BASE_URL).href;

export const useRenderWidgetListener = () => {
  useEffect(() => {
    const unlisten = DeskulptWidgets.Events.render.listen(async (event) => {
      const { id, report } = event.payload;

      if (report.type === "err") {
        useWidgetsStore.setState(
          (state) => ({
            ...state,
            [id]: {
              ...state[id],
              component: () =>
                createElement(ErrorDisplay, {
                  id,
                  error: "Error bundling the widget",
                  message: report.content,
                }),
            },
          }),
          true,
        );
        return;
      }

      const widget = useWidgetsStore.getState()[id];

      // APIs blob URL can be reused if it already exists because the contents
      // are dependent only on widget ID
      let apisBlobUrl: string;
      if (widget?.apisBlobUrl === undefined) {
        const apisCode = window.__DESKULPT_INTERNALS__.apisWrapper
          .replaceAll("__DESKULPT_WIDGET_ID__", id)
          .replaceAll("__RAW_APIS_URL__", RAW_APIS_URL);
        const apisBlob = new Blob([apisCode], {
          type: "application/javascript",
        });
        apisBlobUrl = URL.createObjectURL(apisBlob);
      } else {
        apisBlobUrl = widget.apisBlobUrl;
      }

      // Module blob URL must be recreated every time and old one must be
      // revoked if exists
      if (widget?.moduleBlobUrl !== undefined) {
        URL.revokeObjectURL(widget.moduleBlobUrl);
      }
      let moduleCode = report.content
        .replaceAll("__DESKULPT_BASE_URL__", BASE_URL)
        .replaceAll("__DESKULPT_APIS_BLOB_URL__", apisBlobUrl);
      const moduleBlob = new Blob([moduleCode], {
        type: "application/javascript",
      });
      const moduleBlobUrl = URL.createObjectURL(moduleBlob);

      let module: any;
      try {
        module = await import(/* @vite-ignore */ moduleBlobUrl);
        if (module.default === undefined) {
          throw new Error("Widget module has no default export");
        }
      } catch (error) {
        URL.revokeObjectURL(moduleBlobUrl);
        useWidgetsStore.setState(
          (state) => ({
            ...state,
            [id]: {
              ...state[id],
              component: () =>
                createElement(ErrorDisplay, {
                  id,
                  error: "Error importing the widget module",
                  message: stringify(error),
                }),
              apisBlobUrl,
            },
          }),
          true,
        );
        return;
      }

      useWidgetsStore.setState(
        (state) => ({
          ...state,
          [id]: {
            ...state[id],
            component: module.default,
            apisBlobUrl,
            moduleBlobUrl,
          },
        }),
        true,
      );
    });

    return () => {
      unlisten.then((f) => f()).catch(logger.error);
    };
  }, []);
};
