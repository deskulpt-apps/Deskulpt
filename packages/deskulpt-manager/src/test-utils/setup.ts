import { afterEach, beforeEach, vi } from "vitest";
import {
  cleanupTauriMocks,
  mockClipboard,
  mockInvoke,
  mockOpener,
  setupTauriMocks,
} from "./tauri-mocks";

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

  // Default core commands
  mockInvoke.register("plugin:deskulpt-core|open", () => null);
  mockInvoke.register("plugin:deskulpt-core|log", () => null);
  mockInvoke.register("plugin:deskulpt-core|fetch_logs", () => ({
    entries: [],
    cursor: null,
    hasMore: false,
  }));
  mockInvoke.register("plugin:deskulpt-core|clear_logs", () => 0);
  mockInvoke.register("plugin:deskulpt-core|call_plugin", () => null);

  // Mock scrollIntoView for Radix UI Select component
  Element.prototype.scrollIntoView = vi.fn();
});

// Cleanup after each test
afterEach(() => {
  cleanupTauriMocks();
  mockClipboard.clear();
  mockOpener.clear();
});
