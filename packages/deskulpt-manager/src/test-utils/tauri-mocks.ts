import { vi } from "vitest";
import type { EventCallback } from "@tauri-apps/api/event";
import { MockInvoke } from "./mock-invoke";
import { MockEventSystem } from "./mock-event-system";
import { MockClipboard } from "./mock-clipboard";
import { MockOpener } from "./mock-opener";

// Re-export classes for backward compatibility
export { MockInvoke } from "./mock-invoke";
export { MockEventSystem } from "./mock-event-system";
export { MockClipboard } from "./mock-clipboard";
export { MockOpener } from "./mock-opener";

/**
 * Global mock instances.
 */
export const mockInvoke = new MockInvoke();
export const mockEventSystem = new MockEventSystem();
export const mockClipboard = new MockClipboard();
export const mockOpener = new MockOpener();

/**
 * Setup Tauri API mocks.
 * Call this in test setup files.
 */
export function setupTauriMocks() {
  // Mock @tauri-apps/api/core
  vi.mock("@tauri-apps/api/core", () => ({
    invoke: (command: string, args?: unknown) =>
      mockInvoke.invoke(command, args),
  }));

  // Mock @tauri-apps/api/event
  vi.mock("@tauri-apps/api/event", () => ({
    listen: <T>(
      eventName: string,
      cb: EventCallback<T>,
      options?: { target?: string },
    ) => mockEventSystem.makeEvent<T>(eventName).listen(cb, options),
    once: <T>(
      eventName: string,
      cb: EventCallback<T>,
      options?: { target?: string },
    ) => mockEventSystem.makeEvent<T>(eventName).once(cb, options),
    emit: (eventName: string, payload?: unknown) =>
      mockEventSystem.emitTo(eventName, payload),
    emitTo: (window: string, eventName: string, payload?: unknown) =>
      mockEventSystem.emitTo(eventName, payload, window),
  }));

  // Mock @tauri-apps/plugin-clipboard-manager
  vi.mock("@tauri-apps/plugin-clipboard-manager", () => ({
    writeText: (text: string) => mockClipboard.writeText(text),
    readText: () => mockClipboard.readText(),
  }));

  // Mock @tauri-apps/plugin-opener
  vi.mock("@tauri-apps/plugin-opener", () => ({
    openUrl: (url: string) => mockOpener.openUrl(url),
  }));
}

/**
 * Clean up all mocks.
 * Call this in test teardown.
 */
export function cleanupTauriMocks() {
  mockInvoke.clear();
  mockEventSystem.clear();
  mockClipboard.clear();
  mockOpener.clear();
}
