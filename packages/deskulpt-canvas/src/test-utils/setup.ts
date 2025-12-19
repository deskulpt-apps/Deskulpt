import { afterEach, beforeEach, vi } from "vitest";
import { cleanupTauriMocks, mockInvoke, setupTauriMocks } from "./tauri-mocks";

// Setup Tauri mocks before all tests
setupTauriMocks();

// Setup default command handlers
beforeEach(() => {
  // Default settings update handler
  mockInvoke.register("plugin:deskulpt-settings|update", () => null);

  // Default widget commands
  mockInvoke.register("plugin:deskulpt-widgets|complete_setup", () => null);
  mockInvoke.register("plugin:deskulpt-widgets|refresh", () => null);
  mockInvoke.register("plugin:deskulpt-widgets|refresh_all", () => null);
});

// Mock scrollIntoView for Radix UI Select component
beforeEach(() => {
  Element.prototype.scrollIntoView = vi.fn();
});

// Cleanup after each test
afterEach(() => {
  cleanupTauriMocks();
});
