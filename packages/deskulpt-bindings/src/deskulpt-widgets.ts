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
 * The widgets registry index.
 */
export type Index = { 
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
widgets: IndexEntry[] }

/**
 * An entry for a widget in the registry.
 */
export type IndexEntry = { 
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
authors: ManifestAuthor[]; 
/**
 * A short description of the widget.
 */
description: string; 
/**
 * The releases of the widget, ordered from newest to oldest.
 */
releases: IndexEntryRelease[] }

/**
 * An entry for a specific release of a widget in the registry.
 */
export type IndexEntryRelease = { 
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
 * Author information in a manifest.
 */
export type ManifestAuthor = 
/**
 * The name of the author.
 * 
 * If a string is given, it will be deserialized into this variant.
 */
string | 
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
url?: string }

/**
 * A result-like binary outcome.
 * 
 * This represents the outcome of an operation that can either succeed with a
 * value of type `T` or fail with an error message.
 */
export type Outcome<T> = { type: "ok"; content: T } | { type: "err"; content: string }

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
export type UpdateEvent = Widgets

export type Widget = { manifest: Outcome<WidgetManifest>; settings: WidgetSettings }

/**
 * Deskulpt widget manifest.
 */
export type WidgetManifest = 
/**
 * The metadata of the widget.
 */
({ 
/**
 * The display name of the item.
 */
name: string; 
/**
 * The version of the item.
 */
version?: string; 
/**
 * The authors of the item.
 */
authors?: ManifestAuthor[]; 
/**
 * The license of the item.
 */
license?: string; 
/**
 * A short description of the item.
 */
description?: string; 
/**
 * URL to the homepage of the item.
 */
homepage?: string }) & { 
/**
 * The entry module of the widget that exports the widget component.
 * 
 * This is a path relative to the root of the widget.
 */
entry: string }

/**
 * Preview information about a widget in the registry.
 */
export type WidgetPreview = 
/**
 * More metadata in the widget manifest.
 */
({ 
/**
 * The display name of the item.
 */
name: string; 
/**
 * The version of the item.
 */
version?: string; 
/**
 * The authors of the item.
 */
authors?: ManifestAuthor[]; 
/**
 * The license of the item.
 */
license?: string; 
/**
 * A short description of the item.
 */
description?: string; 
/**
 * URL to the homepage of the item.
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
export type WidgetReference = { 
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
 * Per-widget settings.
 */
export type WidgetSettings = { 
/**
 * The leftmost x-coordinate in pixels.
 */
x: number; 
/**
 * The topmost y-coordinate in pixels.
 */
y: number; 
/**
 * The width in pixels.
 */
width: number; 
/**
 * The height in pixels.
 */
height: number; 
/**
 * The opacity in percentage.
 */
opacity: number; 
/**
 * The z-index.
 * 
 * Higher z-index means the widget will be rendered above those with lower
 * z-index. Widgets with the same z-index can have arbitrary rendering
 * order. The allowed range is from -999 to 999.
 */
zIndex: number; 
/**
 * Whether the widget should be loaded on the canvas or not.
 */
isLoaded: boolean }

/**
 * A patch for partial updates to [`Settings`].
 */
export type WidgetSettingsPatch = { 
/**
 * If not `None`, update [`Settings::x`].
 */
x?: number; 
/**
 * If not `None`, update [`Settings::y`].
 */
y?: number; 
/**
 * If not `None`, update [`Settings::width`].
 */
width?: number; 
/**
 * If not `None`, update [`Settings::height`].
 */
height?: number; 
/**
 * If not `None`, update [`Settings::opacity`].
 */
opacity?: number; 
/**
 * If not `None`, update [`Settings::z_index`].
 */
zIndex?: number; 
/**
 * If not `None`, update [`Settings::is_loaded`].
 */
isLoaded?: boolean }

export type Widgets = { [key in string]: Widget }

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

  export const fetchRegistryIndex = () => invoke<Index>("plugin:deskulpt-widgets|fetch_registry_index");


  export const install = (
    widget: WidgetReference,
  ) => invoke<null>("plugin:deskulpt-widgets|install", {
    widget,
  });


  export const preview = (
    widget: WidgetReference,
  ) => invoke<WidgetPreview>("plugin:deskulpt-widgets|preview", {
    widget,
  });


  export const read = () => invoke<Widgets>("plugin:deskulpt-widgets|read");


  export const refresh = (
    id: string,
  ) => invoke<null>("plugin:deskulpt-widgets|refresh", {
    id,
  });


  export const refreshAll = () => invoke<null>("plugin:deskulpt-widgets|refresh_all");


  export const uninstall = (
    widget: WidgetReference,
  ) => invoke<null>("plugin:deskulpt-widgets|uninstall", {
    widget,
  });


  export const updateSettings = (
    id: string,
    patch: WidgetSettingsPatch,
  ) => invoke<null>("plugin:deskulpt-widgets|update_settings", {
    id,
    patch,
  });


  export const upgrade = (
    widget: WidgetReference,
  ) => invoke<null>("plugin:deskulpt-widgets|upgrade", {
    widget,
  });
}
