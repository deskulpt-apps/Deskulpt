use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use reqwest::header::{ETAG, IF_NONE_MATCH};
use reqwest::{Client, Response, StatusCode};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, instrument};

use crate::catalog::WidgetManifestAuthor;

#[derive(Debug, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
struct RegistryEntryRelease {
    version: String,
    published_at: String,
    digest: String,
}

#[derive(Debug, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
struct RegistryEntry {
    handle: String,
    id: String,
    name: String,
    #[serde(deserialize_with = "RegistryEntry::deserialize_authors")]
    authors: Vec<WidgetManifestAuthor>,
    description: String,
    releases: Vec<RegistryEntryRelease>,
}

impl RegistryEntry {
    fn deserialize_authors<'de, D>(
        deserializer: D,
    ) -> std::result::Result<Vec<WidgetManifestAuthor>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        serde_json::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RegistryIndex {
    api: i32,
    generated_at: String,
    widgets: Vec<RegistryEntry>,
}

pub struct RegistryIndexFetcher {
    client: Client,
    cache_dir: PathBuf,
    cache_path: PathBuf,
    etag_path: PathBuf,
}

impl RegistryIndexFetcher {
    const URL: &str = "https://cdn.jsdelivr.net/gh/deskulpt-apps/widgets@registry/index.json";

    pub fn new(cache_dir: &Path) -> Self {
        Self {
            client: Client::new(),
            cache_dir: cache_dir.to_path_buf(),
            cache_path: cache_dir.join("widgets-registry-index.json"),
            etag_path: cache_dir.join("widgets-registry-index.etag"),
        }
    }

    #[instrument(skip_all, name = "widgets.fetch_registry_index")]
    pub async fn fetch(&self) -> Result<RegistryIndex> {
        tokio::fs::create_dir_all(&self.cache_dir)
            .await
            .context("Failed to create cache directory")?;

        let cached_etag = self.read_etag().await.unwrap_or_else(|e| {
            error!(error = ?e, "Failed to read cached etag");
            None
        });

        let mut request = self.client.get(Self::URL);
        if let Some(etag) = cached_etag {
            debug!(%etag, "If-None-Match");
            request = request.header(IF_NONE_MATCH, etag);
        }

        debug!("Fetching widgets registry index...");
        let response = request.send().await.context("Failed to send request")?;

        match response.status() {
            StatusCode::OK => self.handle_ok(response).await,
            StatusCode::NOT_MODIFIED => self.handle_not_modified().await,
            status => {
                bail!("Fetching failed with status code {status}");
            },
        }
    }

    fn read_cache(&self) -> Result<RegistryIndex> {
        let cache = File::open(&self.cache_path).context("Failed to open cache")?;
        let index =
            serde_json::from_reader(cache).context("Failed to read and deserialize cache")?;
        Ok(index)
    }

    async fn read_etag(&self) -> Result<Option<String>> {
        match tokio::fs::read_to_string(&self.etag_path).await {
            Ok(etag) => Ok(Some(etag.trim().to_string())),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    async fn handle_ok(&self, response: Response) -> Result<RegistryIndex> {
        let etag = response
            .headers()
            .get(ETAG)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_string());

        let body = response
            .bytes()
            .await
            .context("Failed to read response body")?;
        let index = serde_json::from_slice(&body).context("Failed to deserialize response body")?;

        match tokio::fs::write(&self.cache_path, &body).await {
            Ok(_) => debug!(
                path = %self.cache_path.display(),
                "Cached widgets registry index",
            ),
            Err(e) => error!(
                error = ?e,
                path = %self.cache_path.display(),
                "Failed to cache widgets registry index",
            ),
        }

        if let Some(etag) = etag {
            match tokio::fs::write(&self.etag_path, &etag).await {
                Ok(_) => debug!(
                    path = %self.etag_path.display(),
                    "Cached widgets registry index etag",
                ),
                Err(e) => error!(
                    error = ?e,
                    path = %self.etag_path.display(),
                    "Failed to cache widgets registry index etag",
                ),
            }
        }

        Ok(index)
    }

    async fn handle_not_modified(&self) -> Result<RegistryIndex> {
        match self.read_cache() {
            Ok(index) => {
                debug!("Widgets registry index not modified; using cache");
                return Ok(index);
            },
            Err(e) => error!(
                error = ?e,
                "Received 304 Not Modified but failed to read from cache; retrying fresh fetch",
            ),
        }

        let response = self
            .client
            .get(Self::URL)
            .send()
            .await
            .context("Failed to send request")?;

        match response.status() {
            StatusCode::OK => self.handle_ok(response).await,
            status => bail!("Fetching failed with status code {status}"),
        }
    }
}
