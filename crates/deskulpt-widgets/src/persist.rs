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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[schemars(rename = "Widget")]
pub struct PersistedWidget {
    pub settings: WidgetSettings,
}

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "WidgetCatalog")]
pub struct PersistedWidgetCatalog(pub BTreeMap<String, PersistedWidget>);

impl PersistedWidgetCatalog {
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

#[derive(Debug)]
pub struct PersistedWidgetCatalogView<'a>(&'a WidgetCatalog);

impl<'a> PersistedWidgetCatalogView<'a> {
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

struct PersistWorker<R: Runtime> {
    app_handle: AppHandle<R>,
    rx: mpsc::UnboundedReceiver<()>,
    pending: bool,
    debounce: Pin<Box<Sleep>>,
}

impl<R: Runtime> PersistWorker<R> {
    fn new(app_handle: AppHandle<R>, rx: mpsc::UnboundedReceiver<()>) -> Self {
        Self {
            app_handle,
            rx,
            pending: false,
            debounce: Box::pin(tokio::time::sleep(PERSIST_DEBOUNCE)),
        }
    }

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

    fn on_deadline(&mut self) {
        self.pending = false;
        if let Err(e) = self.app_handle.widgets().persist() {
            error!("Failed to persist widgets: {e:?}");
        }
    }

    fn handle_task(&mut self) {
        self.pending = true;
        self.debounce
            .as_mut()
            .reset(Instant::now() + PERSIST_DEBOUNCE);
    }
}

pub struct PersistWorkerHandle(mpsc::UnboundedSender<()>);

impl PersistWorkerHandle {
    pub fn new<R: Runtime>(app_handle: AppHandle<R>) -> Result<Self> {
        let (tx, rx) = mpsc::unbounded_channel();
        tauri::async_runtime::spawn(async move {
            PersistWorker::new(app_handle, rx).run().await;
        });
        Ok(Self(tx))
    }

    pub fn notify(&self) -> Result<()> {
        Ok(self.0.send(())?)
    }
}
