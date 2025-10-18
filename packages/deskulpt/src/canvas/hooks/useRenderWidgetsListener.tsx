import { createElement, useEffect, useRef } from "react";
import { useWidgetsStore } from "./useWidgetsStore";
import { stringifyError } from "../../utils/stringifyError";
import { commands, events } from "../../bindings";
import ErrorDisplay from "../components/ErrorDisplay";

const BASE_URL = new URL(import.meta.url).origin;
const RAW_APIS_URL = new URL("/gen/raw-apis.js", BASE_URL).href;

export function useRenderWidgetsListener() {
  const hasInited = useRef(false);

  useEffect(() => {
    const unlisten = events.renderWidgets.listen(async (event) => {
      const widgets = useWidgetsStore.getState();

      const promises = Object.entries(event.payload).map(async ([id, code]) => {
        let apisBlobUrl;
        if (id in widgets) {
          // APIs blob URL can be reused because the contents are dependent only
          // on widget ID; the code blob URL will definitely change on re-render
          // so we revoke it here
          const widget = widgets[id]!;
          apisBlobUrl = widget.apisBlobUrl;
          if (widget.moduleBlobUrl !== undefined) {
            URL.revokeObjectURL(widget.moduleBlobUrl);
          }
        } else {
          const apisCode = window.__DESKULPT_CANVAS_INTERNALS__.apisWrapper
            .replaceAll("__DESKULPT_WIDGET_ID__", id)
            .replaceAll("__RAW_APIS_URL__", RAW_APIS_URL);
          const apisBlob = new Blob([apisCode], {
            type: "application/javascript",
          });
          apisBlobUrl = URL.createObjectURL(apisBlob);
        }

        if (code.type === "err") {
          useWidgetsStore.setState(
            (state) => ({
              ...state,
              [id]: {
                component: () =>
                  createElement(ErrorDisplay, {
                    id,
                    error: "Error bundling the widget",
                    message: code.content,
                  }),
                apisBlobUrl,
              },
            }),
            true,
          );
          return;
        }

        let moduleCode = code.content
          .replaceAll("__DESKULPT_BASE_URL__", BASE_URL)
          .replaceAll("__DESKULPT_APIS_BLOB_URL__", apisBlobUrl);
        const moduleBlob = new Blob([moduleCode], {
          type: "application/javascript",
        });
        const moduleBlobUrl = URL.createObjectURL(moduleBlob);
        let module;
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
                component: () =>
                  createElement(ErrorDisplay, {
                    id,
                    error: "Error importing the widget module",
                    message: stringifyError(error),
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
              component: module.default,
              apisBlobUrl,
              moduleBlobUrl,
            },
          }),
          true,
        );
      });

      await Promise.all(promises);
    });

    if (!hasInited.current) {
      // Set the canvas as ready to render only once
      commands.core
        .setRenderReady()
        .then(() => {
          hasInited.current = true;
        })
        .catch(console.error);
    }

    return () => {
      unlisten.then((f) => f()).catch(console.error);
    };
  }, []);
}
