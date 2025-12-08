import type { Event, EventCallback } from "@tauri-apps/api/event";

/**
 * Mock implementation of Tauri's event system.
 * Simulates event listeners and emitters.
 */
export class MockEventSystem {
  private listeners = new Map<string, Set<EventCallback<unknown>>>();
  private onceListeners = new Map<string, Set<EventCallback<unknown>>>();

  /**
   * Create a mock event object.
   */
  makeEvent<T>(name: string) {
    return {
      name,
      listen: (
        cb: EventCallback<T>,
        _options?: { target?: string },
      ): Promise<() => void> => {
        if (!this.listeners.has(name)) {
          this.listeners.set(name, new Set());
        }
        this.listeners.get(name)!.add(cb as EventCallback<unknown>);

        return Promise.resolve(() => {
          this.listeners.get(name)?.delete(cb as EventCallback<unknown>);
        });
      },
      once: (
        cb: EventCallback<T>,
        _options?: { target?: string },
      ): Promise<() => void> => {
        if (!this.onceListeners.has(name)) {
          this.onceListeners.set(name, new Set());
        }
        this.onceListeners.get(name)!.add(cb as EventCallback<unknown>);

        return Promise.resolve(() => {
          this.onceListeners.get(name)?.delete(cb as EventCallback<unknown>);
        });
      },
      emit: (payload: T): Promise<void> => {
        return this.emitTo(name, payload);
      },
      emitTo: (window: string, payload: T): Promise<void> => {
        return this.emitTo(name, payload, window);
      },
    };
  }

  /**
   * Emit an event to all listeners.
   */
  emitTo(eventName: string, payload: unknown, target?: string): Promise<void> {
    const event: Event<unknown> = {
      event: eventName,
      id: Date.now(),
      payload,
      target: target || "main",
      windowLabel: target || "main",
    };

    // Call regular listeners
    const listeners = this.listeners.get(eventName);
    if (listeners) {
      for (const listener of listeners) {
        listener(event);
      }
    }

    // Call once listeners and remove them
    const onceListeners = this.onceListeners.get(eventName);
    if (onceListeners) {
      for (const listener of onceListeners) {
        listener(event);
      }
      onceListeners.clear();
    }
  }

  /**
   * Clear all event listeners.
   */
  clear() {
    this.listeners.clear();
    this.onceListeners.clear();
  }
}
