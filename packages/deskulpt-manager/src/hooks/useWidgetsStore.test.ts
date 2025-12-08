import { beforeEach, describe, expect, it } from "vitest";
import { act, renderHook } from "@testing-library/react";
import { useWidgetsStore } from "./useWidgetsStore";
import type { deskulptWidgets } from "@deskulpt/bindings";

describe("useWidgetsStore", () => {
  beforeEach(() => {
    // Clear the store completely
    const state = useWidgetsStore.getState();
    Object.keys(state).forEach((key) => {
      delete state[key as keyof typeof state];
    });
    useWidgetsStore.setState({});
  });

  it("initializes with empty catalog", () => {
    const { result } = renderHook(() => useWidgetsStore());

    expect(result.current).toEqual({});
  });

  it("allows updating widget catalog", () => {
    const { result } = renderHook(() => useWidgetsStore());

    const catalog: deskulptWidgets.WidgetCatalog = {
      "widget-1": {
        type: "ok",
        content: {
          name: "Test Widget",
          version: "1.0.0",
        },
      },
    };

    act(() => {
      useWidgetsStore.setState(catalog);
    });

    expect(result.current["widget-1"]).toBeDefined();
    expect(result.current["widget-1"]?.type).toBe("ok");
  });

  it("allows adding multiple widgets", () => {
    const { result } = renderHook(() => useWidgetsStore());

    act(() => {
      useWidgetsStore.setState({
        "widget-1": {
          type: "ok",
          content: { name: "Widget 1" },
        },
        "widget-2": {
          type: "ok",
          content: { name: "Widget 2" },
        },
      });
    });

    expect(Object.keys(result.current)).toHaveLength(2);
    expect(result.current["widget-1"]).toBeDefined();
    expect(result.current["widget-2"]).toBeDefined();
  });

  it("allows removing widgets", () => {
    renderHook(() => useWidgetsStore());

    act(() => {
      useWidgetsStore.setState({
        "widget-1": {
          type: "ok",
          content: { name: "Widget 1" },
        },
        "widget-2": {
          type: "ok",
          content: { name: "Widget 2" },
        },
      });
    });

    // Verify both widgets are present
    let state = useWidgetsStore.getState();
    expect(state["widget-1"]).toBeDefined();
    expect(state["widget-2"]).toBeDefined();
    expect(Object.keys(state)).toHaveLength(2);

    // Zustand's setState merges by default, so we need to use a function that returns a completely new state
    act(() => {
      useWidgetsStore.setState(
        () => ({
          "widget-1": {
            type: "ok",
            content: { name: "Widget 1" },
          },
        }),
        true, // replace: true - this should replace the entire state
      );
    });

    // Check state directly after update
    state = useWidgetsStore.getState();
    expect(state["widget-1"]).toBeDefined();
    expect(Object.keys(state)).toHaveLength(1);
    expect(Object.keys(state)).not.toContain("widget-2");
  });
});
