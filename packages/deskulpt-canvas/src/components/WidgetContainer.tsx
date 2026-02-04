import { useEffect, useRef, useState } from "react";
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
import { Box, Text } from "@radix-ui/themes";
import { useWidgetsStore } from "../hooks";
import { css } from "@emotion/react";
import { DeskulptWidgets } from "@deskulpt/bindings";

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

const WidgetContainer = ({ id }: WidgetContainerProps) => {
  const draggableRef = useRef<HTMLDivElement>(null);
  const resizeStartRef = useRef<WidgetGeometry>(null);

  // These non-null assertions are safe based on how App.tsx filters the IDs
  const Widget = useWidgetsStore((state) => state[id]!.component);
  const settings = useWidgetsStore((state) => state[id]!.settings!);

  // Local state to avoid jittery movement during dragging and resizing
  const [geometry, setGeometry] = useState({
    x: settings.x,
    y: settings.y,
    width: settings.width,
    height: settings.height,
  });

  useEffect(() => {
    setGeometry({
      x: settings.x,
      y: settings.y,
      width: settings.width,
      height: settings.height,
    });
  }, [settings]);

  const onDragStop = (_: DraggableEvent, data: DraggableData) => {
    setGeometry((prev) => prev && { ...prev, x: data.x, y: data.y });
    DeskulptWidgets.Commands.updateSettings(id, { x: data.x, y: data.y });
  };

  const onResizeStart: ResizeStartCallback = () => {
    resizeStartRef.current = { ...geometry };
  };

  const onResize: ResizeCallback = (_, direction, __, delta) => {
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
  };

  const onResizeStop: ResizeCallback = (_, direction, __, delta) => {
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
    DeskulptWidgets.Commands.updateSettings(id, newGeometry);
  };

  if (!settings.isLoaded) {
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
            onError={(error, info) => {
              logger.error(`Error rendering widget: ${id}`, {
                widgetId: id,
                error,
                info,
              });
            }}
            fallbackRender={({ error }) => (
              <ErrorDisplay
                id={id}
                error="Error in the widget component [React error boundary]"
                message={stringify(error)}
              />
            )}
          >
            {Widget === undefined ? (
              <Text>Loading...</Text>
            ) : (
              <Widget
                id={id}
                x={geometry.x}
                y={geometry.y}
                width={geometry.width}
                height={geometry.height}
              />
            )}
          </ErrorBoundary>
        </Resizable>
      </Box>
    </Draggable>
  );
};

export default WidgetContainer;
