import { memo, useCallback, useEffect, useRef, useState } from "react";
import { flushSync } from "react-dom";
import Draggable, { DraggableData, DraggableEvent } from "react-draggable";
import { Resizable, ResizeCallback, ResizeStartCallback } from "re-resizable";
import { ErrorBoundary } from "react-error-boundary";
import ErrorDisplay from "./ErrorDisplay";
import { stringifyError } from "@deskulpt/utils";
import { LuGripVertical } from "react-icons/lu";
import { Box } from "@radix-ui/themes";
import { useSettingsStore, useWidgetsStore } from "../hooks";
import { css } from "@emotion/react";
import { deskulptSettings } from "@deskulpt/bindings";

const styles = {
  wrapper: css({
    "&:hover": { ".handle": { opacity: 1 } },
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

interface WidgetContainerProps {
  id: string;
}

const WidgetContainer = memo(({ id }: WidgetContainerProps) => {
  const draggableRef = useRef<HTMLDivElement>(null);
  const resizeStartRef = useRef({ x: 0, y: 0, width: 0, height: 0 });

  // This non-null assertion is safe because the IDs are obtained from the keys
  // of the widgets store
  const { component: Widget } = useWidgetsStore((state) => state[id]!);

  const settings = useSettingsStore((state) => state.widgets[id]);
  const opacity = settings?.opacity;

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

  const onResize: ResizeCallback = useCallback(
    (_, direction, __, delta) => {
      if (geometry === undefined) {
        return;
      }

      const { x, y, width, height } = resizeStartRef.current;
      let newX = x;
      let newY = y;
      const newWidth = width + delta.width;
      const newHeight = height + delta.height;

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

      // Force position and size changes to land in the same frame to avoid
      // visual glitches
      flushSync(() => {
        setGeometry({ x: newX, y: newY, width: newWidth, height: newHeight });
      });
    },
    [geometry],
  );

  const onResizeStop: ResizeCallback = useCallback(() => {
    deskulptSettings.commands.update({ widgets: { [id]: { ...geometry } } });
  }, [id, geometry]);

  // Do not render anything if the widget is not fully configured; there could
  // be a gap between widget and settings updates, but they should eventually be
  // in sync
  if (geometry === undefined || opacity === undefined) {
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
          style={{ opacity: opacity / 100 }}
        >
          <ErrorBoundary
            resetKeys={[Widget]}
            fallbackRender={({ error }) => (
              <ErrorDisplay
                id={id}
                error="Error in the widget component [React error boundary]"
                message={stringifyError(error)}
              />
            )}
          >
            <Widget
              id={id}
              x={geometry.x}
              y={geometry.y}
              width={geometry.width}
              height={geometry.height}
              opacity={opacity}
            />
          </ErrorBoundary>
        </Resizable>
      </Box>
    </Draggable>
  );
});

export default WidgetContainer;
