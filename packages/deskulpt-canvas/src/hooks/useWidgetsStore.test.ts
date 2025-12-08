import { beforeEach, describe, expect, it } from "vitest";
import { act, renderHook } from "@testing-library/react";
import { useWidgetsStore } from "./useWidgetsStore";

const MockComponent = () => null;

describe("useWidgetsStore", () => {
  beforeEach(() => {
    useWidgetsStore.setState({});
  });

  it("initializes with empty store", () => {
    const { result } = renderHook(() => useWidgetsStore());

    expect(result.current).toEqual({});
  });

  it("allows adding widget components", () => {
    const { result } = renderHook(() => useWidgetsStore());

    act(() => {
      useWidgetsStore.setState({
        "widget-1": {
          component: MockComponent,
          apisBlobUrl: "blob:mock-apis",
        },
      });
    });

    expect(result.current["widget-1"]).toBeDefined();
    expect(result.current["widget-1"]?.component).toBe(MockComponent);
  });
});
