/*! Auto-generated via `cargo xtask bindings`. DO NOT EDIT! */

import { invoke } from "@tauri-apps/api/core";
import * as TauriEvent from "@tauri-apps/api/event";

// =============================================================================
// Types
// =============================================================================

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
 * A result-like binary outcome.
 * 
 * This represents the outcome of an operation that can either succeed with a
 * value of type `T` or fail with an error message.
 */
export type Outcome<T> = { type: "ok"; content: T } | { type: "err"; content: string }

/**
 * An entry for a widget in the registry.
 */
export type RegistryEntry = { 
/**
 * The publisher handle.
 */
handle: string; 
/**
 * The widget ID.
 * 
 * Note that this ID is unique only within the publisher's namespace.
 */
id: string; 
/**
 * The name of the widget.
 */
name: string; 
/**
 * The authors of the widget.
 */
authors: WidgetManifestAuthor[]; 
/**
 * A short description of the widget.
 */
description: string; 
/**
 * The releases of the widget, ordered from newest to oldest.
 */
releases: RegistryEntryRelease[] }

/**
 * An entry for a specific release of a widget in the registry.
 */
export type RegistryEntryRelease = { 
/**
 * The version string of the release.
 */
version: string; 
/**
 * The publication datetime of the release, in ISO 8601 format.
 */
publishedAt: string; 
/**
 * The SHA-256 digest of the release package.
 * 
 * This is used to verify integrity but also an immutable identifier for
 * uniquely locating the released widget package.
 */
digest: string }

/**
 * The widgets registry index.
 */
export type RegistryIndex = { 
/**
 * The API version.
 */
api: number; 
/**
 * The datetime when the index was generated, in ISO 8601 format.
 */
generatedAt: string; 
/**
 * The list of widgets in the registry.
 */
widgets: RegistryEntry[] }

/**
 * Preview information about a widget in the registry.
 */
export type RegistryWidgetPreview = 
/**
 * More information as in the widget manifest.
 */
({ 
/**
 * The display name of the widget.
 */
name: string; 
/**
 * The version of the widget.
 */
version?: string; 
/**
 * The authors of the widget.
 */
authors?: WidgetManifestAuthor[]; 
/**
 * The license of the widget.
 */
license?: string; 
/**
 * A short description of the widget.
 */
description?: string; 
/**
 * URL to the homepage of the widget.
 */
homepage?: string }) & { 
/**
 * The local ID of the widget.
 * 
 * See [`RegistryWidgetReference::local_id`] for details.
 */
id: string; 
/**
 * The size of the widget package in bytes.
 */
size: number; 
/**
 * The URL of the widget package in the registry.
 */
registryUrl: string; 
/**
 * The creation datetime of the widget package, in ISO 8601 format.
 */
created?: string; 
/**
 * The git repository URL of the widget source code.
 */
git?: string }

/**
 * A reference to a widget in the registry.
 * 
 * These information uniquely and immutably identify a widget package in the
 * widgets registry.
 */
export type RegistryWidgetReference = { 
/**
 * The publisher handle.
 */
handle: string; 
/**
 * The widget ID.
 * 
 * Note that this ID is unique only within the publisher's namespace.
 */
id: string; 
/**
 * The SHA-256 digest of the widget package.
 */
digest: string }

/**
 * Event for reporting the rendering result of a widget to the canvas.
 */
export type RenderEvent = { 
/**
 * The ID of the widget.
 */
id: string; 
/**
 * Either the code string to render or a bundling error message.
 */
report: Outcome<string> }

/**
 * Event for notifying frontend windows of a widget catalog update.
 */
export type UpdateEvent = WidgetCatalog

/**
 * The catalog of Deskulpt widgets.
 * 
 * This keeps a mapping from widget IDs to their manifests (if valid) or error
 * messages (if invalid).
 */
export type WidgetCatalog = { [key in string]: Outcome<WidgetManifest> }

/**
 * Deskulpt widget manifest.
 */
export type WidgetManifest = { 
/**
 * The display name of the widget.
 */
name: string; 
/**
 * The version of the widget.
 */
version?: string; 
/**
 * The authors of the widget.
 */
authors?: WidgetManifestAuthor[]; 
/**
 * The license of the widget.
 */
license?: string; 
/**
 * A short description of the widget.
 */
description?: string; 
/**
 * URL to the homepage of the widget.
 */
homepage?: string }

/**
 * An author of a Deskulpt widget.
 */
export type WidgetManifestAuthor = 
/**
 * An extended author with name, email, and homepage.
 * 
 * If an object is given, it will be deserialized into this variant.
 */
{ 
/**
 * The name of the author.
 */
name: string; 
/**
 * An optional email of the author.
 */
email?: string; 
/**
 * An optional URL to the homepage of the author.
 */
homepage?: string } | 
/**
 * The name of the author.
 * 
 * If a string is given, it will be deserialized into this variant.
 */
string

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
  export const render = makeEvent<RenderEvent>("deskulpt-widgets://render");
  export const update = makeEvent<UpdateEvent>("deskulpt-widgets://update");
}

// =============================================================================
// Commands
// =============================================================================

export namespace Commands {
  /**
   * Fetch the widgets registry index.
   * 
   * This command is a wrapper of
   * [`crate::WidgetsManager::fetch_registry_index`].
   */
  export const fetchRegistryIndex = () => invoke<RegistryIndex>("plugin:deskulpt-widgets|fetch_registry_index");

  /**
   * Install a widget from the registry.
   * 
   * This command is a wrapper of [`crate::WidgetsManager::install`].
   */
  export const install = (
    widget: RegistryWidgetReference,
  ) => invoke<null>("plugin:deskulpt-widgets|install", {
    widget,
  });

  /**
   * Preview a widget from the registry.
   * 
   * This command is a wrapper of [`crate::WidgetsManager::preview`].
   */
  export const preview = (
    widget: RegistryWidgetReference,
  ) => invoke<RegistryWidgetPreview>("plugin:deskulpt-widgets|preview", {
    widget,
  });

  /**
   * Refresh a specific widget by its ID.
   * 
   * This command is a wrapper of [`crate::WidgetsManager::refresh`].
   */
  export const refresh = (
    id: string,
  ) => invoke<null>("plugin:deskulpt-widgets|refresh", {
    id,
  });

  /**
   * Refresh all widgets.
   * 
   * This command is a wrapper of [`crate::WidgetsManager::refresh_all`].
   */
  export const refreshAll = () => invoke<null>("plugin:deskulpt-widgets|refresh_all");

  /**
   * Uninstall a widget from the registry.
   * 
   * This command is a wrapper of [`crate::WidgetsManager::uninstall`].
   */
  export const uninstall = (
    widget: RegistryWidgetReference,
  ) => invoke<null>("plugin:deskulpt-widgets|uninstall", {
    widget,
  });

  /**
   * Upgrade a widget from the registry.
   * 
   * This command is a wrapper of [`crate::WidgetsManager::upgrade`].
   */
  export const upgrade = (
    widget: RegistryWidgetReference,
  ) => invoke<null>("plugin:deskulpt-widgets|upgrade", {
    widget,
  });
}
