import { beforeEach, describe, expect, it, vi } from "vitest";
import { waitFor } from "@testing-library/react";
import { mockEventSystem } from "../test-utils/tauri-mocks";
import { useShowToastListener } from "./useShowToastListener";
import { deskulptCore } from "@deskulpt/bindings";
import { renderWithProviders, screen } from "../test-utils/test-helpers";

describe("useShowToastListener", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("listens for success toast events", async () => {
    const TestComponent = () => {
      useShowToastListener();
      return null;
    };

    renderWithProviders(<TestComponent />);

    // Emit success toast event
    await mockEventSystem.emitTo(deskulptCore.events.showToast.name, {
      type: "success",
      content: "Operation successful",
    });

    await waitFor(() => {
      expect(screen.getByText("Operation successful")).toBeInTheDocument();
    });
  });

  it("listens for error toast events", async () => {
    const TestComponent = () => {
      useShowToastListener();
      return null;
    };

    renderWithProviders(<TestComponent />);

    // Emit error toast event
    await mockEventSystem.emitTo(deskulptCore.events.showToast.name, {
      type: "error",
      content: "Operation failed",
    });

    await waitFor(() => {
      expect(screen.getByText("Operation failed")).toBeInTheDocument();
    });
  });
});
