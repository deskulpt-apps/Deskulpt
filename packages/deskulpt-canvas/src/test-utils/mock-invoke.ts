/**
 * Mock implementation of Tauri's invoke function for IPC commands.
 * Stores command handlers that can be set up in tests.
 */
export class MockInvoke {
  private handlers = new Map<string, (args?: unknown) => unknown>();

  /**
   * Register a handler for a specific command.
   * @param command - The command name (e.g., "plugin:deskulpt-settings|update")
   * @param handler - Function that returns the response for this command
   */
  register(command: string, handler: (args?: unknown) => unknown) {
    this.handlers.set(command, handler);
  }

  /**
   * Register a default handler that will be used if no specific handler is found.
   */
  defaultHandler: ((command: string, args?: unknown) => unknown) | null = null;

  /**
   * Invoke a command (mock implementation).
   */
  invoke<T = unknown>(command: string, args?: unknown): Promise<T> {
    const handler = this.handlers.get(command);
    if (handler) {
      return Promise.resolve(handler(args) as T);
    }
    if (this.defaultHandler) {
      return Promise.resolve(this.defaultHandler(command, args) as T);
    }
    throw new Error(`No handler registered for command: ${command}`);
  }

  /**
   * Clear all registered handlers.
   */
  clear() {
    this.handlers.clear();
    this.defaultHandler = null;
  }
}
