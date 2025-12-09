import { beforeEach, describe, expect, it } from "vitest";
import { act, renderHook } from "@testing-library/react";
import { useSettingsStore } from "./useSettingsStore";

describe("useSettingsStore", () => {
  beforeEach(() => {
    useSettingsStore.setState({
      theme: "light",
      canvasImode: "auto",
      shortcuts: {},
      widgets: {},
    });
  });

  it("initializes with default settings", () => {
    const { result } = renderHook(() => useSettingsStore());

    expect(result.current.theme).toBe("light");
    expect(result.current.canvasImode).toBe("auto");
    expect(result.current.shortcuts).toEqual({});
    expect(result.current.widgets).toEqual({});
  });

  it("allows updating theme", () => {
    const { result } = renderHook(() => useSettingsStore());

    act(() => {
      useSettingsStore.setState({ theme: "dark" });
    });

    expect(result.current.theme).toBe("dark");
  });

  it("allows updating canvas interaction mode", () => {
    const { result } = renderHook(() => useSettingsStore());

    act(() => {
      useSettingsStore.setState({ canvasImode: "float" });
    });

    expect(result.current.canvasImode).toBe("float");
  });

  it("allows updating shortcuts", () => {
    const { result } = renderHook(() => useSettingsStore());

    act(() => {
      useSettingsStore.setState({
        shortcuts: { toggleCanvasImode: "Ctrl+T" },
      });
    });

    expect(result.current.shortcuts.toggleCanvasImode).toBe("Ctrl+T");
  });

  it("allows updating widgets", () => {
    const { result } = renderHook(() => useSettingsStore());

    act(() => {
      useSettingsStore.setState({
        widgets: {
          "widget-1": {
            x: 100,
            y: 200,
            width: 300,
            height: 400,
            opacity: 80,
            zIndex: 10,
            isLoaded: true,
          },
        },
      });
    });

    expect(result.current.widgets["widget-1"]).toBeDefined();
    expect(result.current.widgets["widget-1"]?.x).toBe(100);
  });

  it("allows partial updates", () => {
    const { result } = renderHook(() => useSettingsStore());

    act(() => {
      useSettingsStore.setState((state) => ({
        ...state,
        theme: "dark",
      }));
    });

    expect(result.current.theme).toBe("dark");
    expect(result.current.canvasImode).toBe("auto"); // Unchanged
  });
});
