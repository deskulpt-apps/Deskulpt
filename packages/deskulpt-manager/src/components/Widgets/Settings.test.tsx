import { beforeEach, describe, expect, it, vi } from "vitest";
import {
  renderWithProviders,
  screen,
  userEvent,
  waitFor,
} from "../../test-utils/test-helpers";
import { mockInvoke } from "../../test-utils/tauri-mocks";
import Settings from "./Settings";

describe("Widget Settings", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders widget settings inputs", () => {
    renderWithProviders(<Settings id="widget-1" />, {
      initialSettings: {
        widgets: {
          "widget-1": {
            x: 100,
            y: 200,
            width: 300,
            height: 400,
            opacity: 80,
            isLoaded: true,
          },
        },
      },
    });

    // Check that inputs are rendered with correct values
    const inputs = screen.getAllByRole("spinbutton");
    expect(inputs).toHaveLength(5); // x, y, width, height, opacity
  });

  it("updates position when x value changes", async () => {
    const user = userEvent.setup();
    const updateSpy = vi.fn(() => null);
    mockInvoke.register("plugin:deskulpt-settings|update", updateSpy);

    renderWithProviders(<Settings id="widget-1" />, {
      initialSettings: {
        widgets: {
          "widget-1": {
            x: 100,
            y: 200,
            width: 300,
            height: 400,
            opacity: 80,
            isLoaded: true,
          },
        },
      },
    });

    const inputs = screen.getAllByRole("spinbutton");
    const xInput = inputs[0];
    if (xInput) {
      await user.clear(xInput);
      await user.type(xInput, "150");
    }

    // Wait for debounce/defer - IntegerInput calls onValueChange on change
    await waitFor(
      () => {
        expect(updateSpy).toHaveBeenCalled();
      },
      { timeout: 2000 },
    );
  });

  it("updates position when y value changes", async () => {
    const user = userEvent.setup();
    const updateSpy = vi.fn(() => null);
    mockInvoke.register("plugin:deskulpt-settings|update", updateSpy);

    renderWithProviders(<Settings id="widget-1" />, {
      initialSettings: {
        widgets: {
          "widget-1": {
            x: 100,
            y: 200,
            width: 300,
            height: 400,
            opacity: 80,
            isLoaded: true,
          },
        },
      },
    });

    const inputs = screen.getAllByRole("spinbutton");
    const yInput = inputs[1];
    if (yInput) {
      await user.clear(yInput);
      await user.type(yInput, "250");
    }

    await waitFor(
      () => {
        expect(updateSpy).toHaveBeenCalled();
      },
      { timeout: 2000 },
    );
  });

  it("updates size when width value changes", async () => {
    const user = userEvent.setup();
    const updateSpy = vi.fn(() => null);
    mockInvoke.register("plugin:deskulpt-settings|update", updateSpy);

    renderWithProviders(<Settings id="widget-1" />, {
      initialSettings: {
        widgets: {
          "widget-1": {
            x: 100,
            y: 200,
            width: 300,
            height: 400,
            opacity: 80,
            isLoaded: true,
          },
        },
      },
    });

    const inputs = screen.getAllByRole("spinbutton");
    const widthInput = inputs[2];
    if (widthInput) {
      await user.clear(widthInput);
      await user.type(widthInput, "350");
    }

    await waitFor(
      () => {
        expect(updateSpy).toHaveBeenCalled();
      },
      { timeout: 2000 },
    );
  });

  it("updates size when height value changes", async () => {
    const user = userEvent.setup();
    const updateSpy = vi.fn(() => null);
    mockInvoke.register("plugin:deskulpt-settings|update", updateSpy);

    renderWithProviders(<Settings id="widget-1" />, {
      initialSettings: {
        widgets: {
          "widget-1": {
            x: 100,
            y: 200,
            width: 300,
            height: 400,
            opacity: 80,
            isLoaded: true,
          },
        },
      },
    });

    const inputs = screen.getAllByRole("spinbutton");
    const heightInput = inputs[3];
    if (heightInput) {
      await user.clear(heightInput);
      await user.type(heightInput, "450");
    }

    await waitFor(
      () => {
        expect(updateSpy).toHaveBeenCalled();
      },
      { timeout: 2000 },
    );
  });

  it("updates opacity when opacity value changes", async () => {
    const user = userEvent.setup();
    const updateSpy = vi.fn(() => null);
    mockInvoke.register("plugin:deskulpt-settings|update", updateSpy);

    renderWithProviders(<Settings id="widget-1" />, {
      initialSettings: {
        widgets: {
          "widget-1": {
            x: 100,
            y: 200,
            width: 300,
            height: 400,
            opacity: 80,
            isLoaded: true,
          },
        },
      },
    });

    const inputs = screen.getAllByRole("spinbutton");
    const opacityInput = inputs[4];
    if (opacityInput) {
      await user.clear(opacityInput);
      await user.type(opacityInput, "90");
    }

    await waitFor(
      () => {
        expect(updateSpy).toHaveBeenCalled();
      },
      { timeout: 2000 },
    );
  });
});
