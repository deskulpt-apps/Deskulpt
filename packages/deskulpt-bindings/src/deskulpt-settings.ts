/*! Auto-generated via `cargo xtask bindings`. DO NOT EDIT! */

import { invoke } from "@tauri-apps/api/core";
import * as TauriEvent from "@tauri-apps/api/event";

// =============================================================================
// Types
// =============================================================================

/**
 * The canvas interaction mode.
 */
export type CanvasImode = 
/**
 * Auto mode.
 * 
 * Automatically switch between sink and float modes based on mouse
 * position, so that users will feel like the widgets and the desktop are
 * simultaneously interactable.
 */
"auto" | 
/**
 * Sink mode.
 * 
 * The canvas is click-through. Widgets are not interactable. The desktop
 * is interactable.
 */
"sink" | 
/**
 * Float mode.
 * 
 * The canvas is not click-through. Widgets are interactable. The desktop
 * is not interactable.
 */
"float"

/**
 * Deskulpt window enum.
 */
export type DeskulptWindow = 
/**
 * Deskulpt portal.
 */
"portal" | 
/**
 * Deskulpt canvas.
 */
"canvas"

/**
 * Full settings of the Deskulpt application.
 */
export type Settings = { 
/**
 * The application theme.
 */
theme: Theme; 
/**
 * The canvas interaction mode.
 */
canvasImode: CanvasImode; 
/**
 * The keyboard shortcuts.
 * 
 * This maps the actions to the shortcut strings that will trigger them.
 */
shortcuts: Partial<{ [key in ShortcutAction]: string }> }

/**
 * A patch for partial updates to [`Settings`].
 */
export type SettingsPatch = { 
/**
 * If not `None`, update [`Settings::theme`].
 */
theme?: Theme; 
/**
 * If not `None`, update [`Settings::canvas_imode`].
 */
canvasImode?: CanvasImode; 
/**
 * If not `None`, update [`Settings::shortcuts`].
 * 
 * Non-specified shortcuts will remain unchanged. If a shortcut value is
 * `None`, it means removing that shortcut. Otherwise, it means updating
 * or adding that shortcut.
 */
shortcuts?: Partial<{ [key in ShortcutAction]: string | null }> }

/**
 * Actions that can be bound to keyboard shortcuts.
 */
export type ShortcutAction = 
/**
 * Toggle the canvas interaction mode (imode).
 */
"toggleCanvasImode" | 
/**
 * Open Deskulpt portal.
 */
"openPortal"

/**
 * The light/dark theme of the application interface.
 */
export type Theme = "light" | "dark"

/**
 * Event for notifying frontend windows of a settings update.
 */
export type UpdateEvent = Settings

// =============================================================================
// Events
// =============================================================================

function makeEvent<T>(name: string) {
  return {
    /** The name of the event. */
    name,
    /** Listen for the event. */
    listen: (cb: TauriEvent.EventCallback<T>, options?: TauriEvent.Options) =>
      TauriEvent.listen(name, cb, options),
    /** Listen once for the event. */
    once: (cb: TauriEvent.EventCallback<T>, options?: TauriEvent.Options) =>
      TauriEvent.once(name, cb, options),
    /** Emit the event to all targets. */
    emit: (payload: T) => TauriEvent.emit(name, payload),
    /** Emit the event to a specific Deskulpt window. */
    emitTo: (window: DeskulptWindow, payload: T) =>
      TauriEvent.emitTo(window, name, payload),
  };
}

export namespace Events {
  export const update = makeEvent<UpdateEvent>("deskulpt-settings://update");
}

// =============================================================================
// Commands
// =============================================================================

export namespace Commands {
  /**
   * Update the settings with a patch.
   * 
   * Wrapper of [`crate::SettingsManager::update`].
   */
  export const update = (
    patch: SettingsPatch,
  ) => invoke<null>("plugin:deskulpt-settings|update", {
    patch,
  });
}
