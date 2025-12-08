// Side-effect import for jest-dom matchers
// eslint-disable-next-line import/no-unassigned-import
import "@testing-library/jest-dom";
import { cleanup } from "@testing-library/react";
import { afterEach, vi } from "vitest";

// Cleanup after each test
afterEach(() => {
  cleanup();
});

// Mock window.__DESKULPT_INTERNALS__
Object.defineProperty(window, "__DESKULPT_INTERNALS__", {
  value: {
    initialSettings: {
      theme: "light",
      canvasImode: "auto",
      shortcuts: {},
      widgets: {},
    },
    apisWrapper: `// Mock APIs wrapper`,
  },
  writable: true,
  configurable: true,
});

// Mock URL.createObjectURL and URL.revokeObjectURL
global.URL.createObjectURL = vi.fn(() => "blob:mock-url");
global.URL.revokeObjectURL = vi.fn();

// Mock ResizeObserver
global.ResizeObserver = vi.fn().mockImplementation(() => ({
  observe: vi.fn(),
  unobserve: vi.fn(),
  disconnect: vi.fn(),
}));

// Mock hasPointerCapture for Radix UI Select component
Object.defineProperty(Element.prototype, "hasPointerCapture", {
  value: vi.fn(() => false),
  writable: true,
  configurable: true,
});

// __VERSION__ is now defined via Vitest's define config in vitest.config.ts
