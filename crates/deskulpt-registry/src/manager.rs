//! Deskulpt registry manager and its APIs.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use deskulpt_common::event::Event;
use tauri::{AppHandle, Manager, Runtime};

/// Manager for Deskulpt registry.
pub struct RegistryManager<R: Runtime> {
    /// The Tauri app handle.
    pub(crate) app_handle: AppHandle<R>,
}

impl<R: Runtime> RegistryManager<R> {
    /// Initialize the [`RegistryManager`].
    pub(crate) fn new(app_handle: AppHandle<R>) -> Result<Self> {
        Ok(Self { app_handle })
    }
}
