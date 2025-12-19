import {
  ErrorInfo,
  memo,
  useCallback,
  useEffect,
  useRef,
  useState,
} from "react";
import { flushSync } from "react-dom";
import Draggable, { DraggableData, DraggableEvent } from "react-draggable";
import {
  NumberSize,
  Resizable,
  ResizeCallback,
  ResizeDirection,
  ResizeStartCallback,
} from "re-resizable";
import { ErrorBoundary } from "react-error-boundary";
import ErrorDisplay from "./ErrorDisplay";
import { logger, stringify } from "@deskulpt/utils";
import { LuGripVertical } from "react-icons/lu";
import { Box } from "@radix-ui/themes";
import { useSettingsStore, useWidgetsStore } from "../hooks";
import { css } from "@emotion/react";
import { deskulptSettings } from "@deskulpt/bindings";

const styles = {
  wrapper: css({
    "&:hover": {
      ".handle": { opacity: 1 },
      boxShadow:
        "0 0 20px var(--gray-a7), 0 0 40px var(--gray-a5), 0 0 60px var(--gray-a3), inset 0 0 20px var(--gray-a2)",
    },
  }),
  handle: css({
    cursor: "grab",
    opacity: 0,
    zIndex: 2,
    transition: "opacity 200ms ease-in-out",
  }),
  container: css({
    color: "var(--gray-12)",
    zIndex: 1,
  }),
};

interface WidgetGeometry {
  x: number;
  y: number;
  width: number;
  height: number;
}

interface WidgetContainerProps {
  id: string;
}

function computeResizedGeometry(
  geometry: WidgetGeometry,
  direction: ResizeDirection,
  delta: NumberSize,
): WidgetGeometry {
  const { x, y, width, height } = geometry;
  let newX = x;
  let newY = y;
  const newWidth = width + delta.width;
  const newHeight = height + delta.height;

  // If resizing from top and/or left edges, we need to adjust position
  // accordingly to make sure their opposite edges stay in place
  switch (direction) {
    case "top":
    case "topRight":
      newY = y - delta.height;
      break;
    case "left":
    case "bottomLeft":
      newX = x - delta.width;
      break;
    case "topLeft":
      newX = x - delta.width;
      newY = y - delta.height;
      break;
  }

  return { x: newX, y: newY, width: newWidth, height: newHeight };
}

const WidgetContainer = memo(({ id }: WidgetContainerProps) => {
  const draggableRef = useRef<HTMLDivElement>(null);
  const resizeStartRef = useRef<WidgetGeometry>(null);

  // This non-null assertion is safe because the IDs are obtained from the keys
  // of the widgets store
  const { component: Widget } = useWidgetsStore((state) => state[id]!);

  const settings = useSettingsStore((state) => state.widgets[id]);

  // Local state to avoid jittery movement during dragging and resizing
  const [geometry, setGeometry] = useState(
    settings === undefined
      ? undefined
      : {
          x: settings.x,
          y: settings.y,
          width: settings.width,
          height: settings.height,
        },
  );

  useEffect(() => {
    if (settings === undefined) {
      return;
    }
    setGeometry({
      x: settings.x,
      y: settings.y,
      width: settings.width,
      height: settings.height,
    });
  }, [settings]);

  const onDragStop = useCallback(
    (_: DraggableEvent, data: DraggableData) => {
      setGeometry((prev) => prev && { ...prev, x: data.x, y: data.y });
      deskulptSettings.commands.update({
        widgets: { [id]: { x: data.x, y: data.y } },
      });
    },
    [id],
  );

  const onResizeStart: ResizeStartCallback = useCallback(() => {
    if (geometry === undefined) {
      return;
    }
    resizeStartRef.current = { ...geometry };
  }, [geometry]);

  const onResize: ResizeCallback = useCallback((_, direction, __, delta) => {
    if (resizeStartRef.current === null) {
      return;
    }
    const newGeometry = computeResizedGeometry(
      resizeStartRef.current,
      direction,
      delta,
    );

    // Force position and size changes to land in the same frame to avoid
    // visual glitches
    flushSync(() => {
      setGeometry(newGeometry);
    });
  }, []);

  const onResizeStop: ResizeCallback = useCallback(
    (_, direction, __, delta) => {
      if (resizeStartRef.current === null) {
        return;
      }

      // We recompute with delta instead of using local state because at time
      // this callback is triggered, we cannot guarantee that the local state
      // updates has all been flushed due to react's asynchronous state updates;
      // using delta also reduces the dependency array of this callback
      const newGeometry = computeResizedGeometry(
        resizeStartRef.current,
        direction,
        delta,
      );
      deskulptSettings.commands.update({ widgets: { [id]: newGeometry } });
    },
    [id],
  );

  const onRenderError = useCallback(
    (error: Error, info: ErrorInfo) => {
      logger.error(`Error rendering widget: ${id}`, {
        widgetId: id,
        error,
        info,
      });
    },
    [id],
  );

  // Do not render anything if the widget is not fully configured; there could
  // be a gap between widget and settings updates, but they should eventually be
  // in sync
  if (settings === undefined || geometry === undefined || !settings.isLoaded) {
    return null;
  }

  return (
    <Draggable
      nodeRef={draggableRef}
      position={{ x: geometry.x, y: geometry.y }}
      onStop={onDragStop}
      bounds="body"
      handle=".handle"
    >
      <Box
        ref={draggableRef}
        overflow="hidden"
        position="absolute"
        css={styles.wrapper}
        style={{ zIndex: settings.zIndex }}
      >
        <Box
          className="handle"
          position="absolute"
          top="1"
          right="1"
          css={styles.handle}
          asChild
        >
          <LuGripVertical size={20} />
        </Box>
        <Resizable
          size={{ width: geometry.width, height: geometry.height }}
          onResizeStart={onResizeStart}
          onResize={onResize}
          onResizeStop={onResizeStop}
          css={styles.container}
          style={{ opacity: settings.opacity / 100 }}
        >
          <ErrorBoundary
            resetKeys={[Widget]}
            onError={onRenderError}
            fallbackRender={({ error }) => (
              <ErrorDisplay
                id={id}
                error="Error in the widget component [React error boundary]"
                message={stringify(error)}
              />
            )}
          >
            <Widget
              id={id}
              x={geometry.x}
              y={geometry.y}
              width={geometry.width}
              height={geometry.height}
            />
          </ErrorBoundary>
        </Resizable>
      </Box>
    </Draggable>
  );
});

export default WidgetContainer;
