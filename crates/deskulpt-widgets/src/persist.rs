use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::pin::Pin;

use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Runtime};
use tokio::sync::mpsc;
use tokio::time::{Duration, Instant, Sleep};
use tracing::error;

use crate::WidgetsExt;
use crate::catalog::{WidgetCatalog, WidgetSettings};

/// Persisted representation of a widget.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[schemars(rename = "Widget")]
pub struct PersistedWidget {
    pub settings: WidgetSettings,
}

/// Persisted representation of the widget catalog.
#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "WidgetCatalog")]
pub struct PersistedWidgetCatalog(pub BTreeMap<String, PersistedWidget>);

impl PersistedWidgetCatalog {
    /// Load the persisted widget catalog from disk.
    ///
    /// If the file does not exist, empty catalog is returned. All other errors
    /// will be propagated.
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Default::default());
        }
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let catalog = serde_json::from_reader(reader)?;
        Ok(catalog)
    }
}

/// A view of the widget catalog for persistence.
///
/// The serialization format will follow the representation of
/// [`PersistedWidgetCatalog`].
#[derive(Debug)]
pub struct PersistedWidgetCatalogView<'a>(&'a WidgetCatalog);

impl<'a> PersistedWidgetCatalogView<'a> {
    /// Persist the widget catalog to disk.
    pub fn persist(&self, path: &Path) -> Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }
}

impl<'a> From<&'a WidgetCatalog> for PersistedWidgetCatalogView<'a> {
    fn from(catalog: &'a WidgetCatalog) -> Self {
        Self(catalog)
    }
}

impl<'a> Serialize for PersistedWidgetCatalogView<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct PersistedWidgetView<'a> {
            settings: &'a WidgetSettings,
        }

        let mut map = serializer.serialize_map(Some(self.0.0.len()))?;
        for (k, v) in self.0.0.iter() {
            map.serialize_entry(
                k,
                &PersistedWidgetView {
                    settings: &v.settings,
                },
            )?;
        }
        map.end()
    }
}

/// Debounce duration for persistence.
const PERSIST_DEBOUNCE: Duration = Duration::from_millis(500);

/// The worker for persisting widgets.
struct PersistWorker<R: Runtime> {
    /// The Tauri app handle.
    app_handle: AppHandle<R>,
    /// The receiver for incoming persist notifications.
    rx: mpsc::UnboundedReceiver<()>,
    /// Whether a persist task is pending.
    pending: bool,
    /// The debounce timer for persistence.
    debounce: Pin<Box<Sleep>>,
}

impl<R: Runtime> PersistWorker<R> {
    /// Create a new [`PersistWorker`] instance.
    fn new(app_handle: AppHandle<R>, rx: mpsc::UnboundedReceiver<()>) -> Self {
        Self {
            app_handle,
            rx,
            pending: false,
            debounce: Box::pin(tokio::time::sleep(PERSIST_DEBOUNCE)),
        }
    }

    /// Run the worker event loop.
    ///
    /// This function will run indefinitely until the worker channel is closed.
    async fn run(mut self) {
        loop {
            tokio::select! {
                _ = &mut self.debounce, if self.pending => {
                    self.on_deadline();
                },
                task = self.rx.recv() => match task {
                    Some(_) => self.handle_task(),
                    None => break,
                },
            }
        }
    }

    /// Fire the persist operation when the debounce timer elapses.
    fn on_deadline(&mut self) {
        self.pending = false;
        if let Err(e) = self.app_handle.widgets().persist() {
            error!("Failed to persist widgets: {e:?}");
        }
    }

    /// Handle an incoming persist task.
    fn handle_task(&mut self) {
        self.pending = true;
        self.debounce
            .as_mut()
            .reset(Instant::now() + PERSIST_DEBOUNCE);
    }
}

/// Handle for communicating with the persistence worker.
pub struct PersistWorkerHandle(mpsc::UnboundedSender<()>);

impl PersistWorkerHandle {
    /// Create a new [`PersistWorkerHandle`] instance.
    ///
    /// This immediately spawns a dedicated worker on Tauri's singleton async
    /// runtime that listens for incoming notifications and processes them with
    /// debouncing.
    pub fn new<R: Runtime>(app_handle: AppHandle<R>) -> Result<Self> {
        let (tx, rx) = mpsc::unbounded_channel();
        tauri::async_runtime::spawn(async move {
            PersistWorker::new(app_handle, rx).run().await;
        });
        Ok(Self(tx))
    }

    /// Instruct the worker to persist the widget catalog.
    ///
    /// This does not block. The task is sent to the worker for asynchronous
    /// processing and does not wait for completion. The worker will debounce
    /// multiple notifications within a short time frame. An error is returned
    /// only if task submission fails, but not if task processing fails.
    pub fn notify(&self) -> Result<()> {
        Ok(self.0.send(())?)
    }
}
