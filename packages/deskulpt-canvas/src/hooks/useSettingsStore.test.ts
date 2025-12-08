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
  });

  it("allows updating theme", () => {
    const { result } = renderHook(() => useSettingsStore());

    act(() => {
      useSettingsStore.setState({ theme: "dark" });
    });

    expect(result.current.theme).toBe("dark");
  });

  it("allows updating widget settings", () => {
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
          },
        },
      });
    });

    expect(result.current.widgets["widget-1"]).toBeDefined();
  });
});
