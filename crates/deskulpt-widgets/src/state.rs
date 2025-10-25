use std::sync::Mutex;

use tauri::{AppHandle, Runtime};

use crate::config::Catalog;

#[derive(Default)]
struct WidgetsInner {
    catalog: Catalog,
}

/// Managed state for Deskulpt widgets.
pub struct Widgets<R: Runtime> {
    app_handle: AppHandle<R>,
    inner: Mutex<WidgetsInner>,
}

impl<R: Runtime> Widgets<R> {
    pub fn new(app_handle: AppHandle<R>) -> Self {
        Self {
            app_handle,
            inner: Default::default(),
        }
    }
}
