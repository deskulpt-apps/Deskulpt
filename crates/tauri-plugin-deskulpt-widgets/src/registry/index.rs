//! Utilities for interacting with the widgets registry index.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use reqwest::header::{ETAG, IF_NONE_MATCH};
use reqwest::{Client, Response, StatusCode};
use serde::{Deserialize, Serialize};

use crate::catalog::WidgetManifestAuthor;

/// An entry for a specific release of a widget in the registry.
#[derive(Debug, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
struct RegistryEntryRelease {
    /// The version string of the release.
    version: String,
    /// The publication datetime of the release, in ISO 8601 format.
    published_at: String,
    /// The SHA-256 digest of the release package.
    ///
    /// This is used to verify integrity but also an immutable identifier for
    /// uniquely locating the released widget package.
    digest: String,
}

/// An entry for a widget in the registry.
#[derive(Debug, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
struct RegistryEntry {
    /// The publisher handle.
    handle: String,
    /// The widget ID.
    ///
    /// Note that this ID is unique only within the publisher's namespace.
    id: String,
    /// The name of the widget.
    name: String,
    /// The authors of the widget.
    authors: Vec<WidgetManifestAuthor>,
    /// A short description of the widget.
    description: String,
    /// The releases of the widget, ordered from newest to oldest.
    releases: Vec<RegistryEntryRelease>,
}

/// The widgets registry index.
#[derive(Debug, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RegistryIndex {
    /// The API version.
    api: i32,
    /// The datetime when the index was generated, in ISO 8601 format.
    generated_at: String,
    /// The list of widgets in the registry.
    widgets: Vec<RegistryEntry>,
}

/// A fetcher for the widgets registry index.
pub struct RegistryIndexFetcher {
    /// The HTTP client.
    client: Client,
    /// The cache directory.
    cache_dir: PathBuf,
    /// The path to the cached index file.
    cache_path: PathBuf,
    /// The path to the cached etag file.
    etag_path: PathBuf,
}

impl RegistryIndexFetcher {
    /// The static URL of the widgets registry index.
    const URL: &str = "https://cdn.jsdelivr.net/gh/deskulpt-apps/widgets@registry/index.json";

    /// Create a new [`RegistryIndexFetcher`] instance.
    ///
    /// This will automatically assign cache paths within the given cache
    /// directory. A new HTTP client will be created to perform requests.
    pub fn new(cache_dir: &Path) -> Self {
        Self {
            client: Client::new(),
            cache_dir: cache_dir.to_path_buf(),
            cache_path: cache_dir.join("widgets-registry-index.json"),
            etag_path: cache_dir.join("widgets-registry-index.etag"),
        }
    }

    /// Fetch the widgets registry index.
    ///
    /// This will use a cached etag to perform a conditional GET request. If the
    /// registry index has not changed since the last fetch, the cached version
    /// will be used if available and valid. Otherwise, a fresh copy will be
    /// fetched and cached.
    #[tracing::instrument(skip_all, level = "debug")]
    pub async fn fetch(&self) -> Result<RegistryIndex> {
        tokio::fs::create_dir_all(&self.cache_dir)
            .await
            .context("Failed to create cache directory")?;

        let cached_etag = self.read_etag().await.unwrap_or_else(|e| {
            tracing::warn!(
                error = ?e,
                path = %self.etag_path.display(),
                "Failed to read cached etag; proceeding without it",
            );
            None
        });

        let mut request = self.client.get(Self::URL);
        if let Some(etag) = cached_etag {
            tracing::debug!(%etag, "Using cached etag");
            request = request.header(IF_NONE_MATCH, etag);
        }

        let response = request
            .send()
            .await
            .context("Failed to send HTTP request")?;

        match response.status() {
            StatusCode::OK => self.handle_ok(response).await,
            StatusCode::NOT_MODIFIED => self.handle_not_modified().await,
            status => {
                bail!("HTTP request failed with status code {status}");
            },
        }
    }

    /// Read the cached registry index from disk.
    async fn read_cache(&self) -> Result<RegistryIndex> {
        let cache = tokio::fs::read(&self.cache_path)
            .await
            .context("Failed to read cache")?;
        let index = serde_json::from_slice(&cache).context("Failed to deserialize cache")?;
        Ok(index)
    }

    /// Read the cached etag from disk.
    ///
    /// Specially, if the etag file does not exists, this returns `Ok(None)`
    /// instead of an error.
    async fn read_etag(&self) -> Result<Option<String>> {
        match tokio::fs::read_to_string(&self.etag_path).await {
            Ok(etag) => Ok(Some(etag.trim().to_string())),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Handle a 200 OK response.
    ///
    /// This will read the response body, deserialize it, and cache both the
    /// body and the etag (if present) to disk. Failure to cache will not be
    /// treated as an error.
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
            Ok(_) => tracing::debug!(path = %self.cache_path.display(), "Cached registry index"),
            Err(e) => tracing::warn!(
                error = ?e,
                path = %self.cache_path.display(),
                "Failed to cache registry index",
            ),
        }

        if let Some(etag) = etag {
            match tokio::fs::write(&self.etag_path, &etag).await {
                Ok(_) => tracing::debug!(path = %self.etag_path.display(), "Cached etag"),
                Err(e) => tracing::warn!(
                    error = ?e,
                    path = %self.etag_path.display(),
                    "Failed to cache etag",
                ),
            }
        }

        Ok(index)
    }

    /// Handle a 304 Not Modified response.
    ///
    /// This will attempt to read the cached index from disk. If that fails, it
    /// will fall back to performing a fresh fetch.
    async fn handle_not_modified(&self) -> Result<RegistryIndex> {
        match self.read_cache().await {
            Ok(index) => {
                tracing::debug!("Widgets registry index not modified; using cache");
                return Ok(index);
            },
            Err(e) => tracing::warn!(
                error = ?e,
                path = %self.cache_path.display(),
                "Received 304 Not Modified but failed to read from cache; retrying fresh fetch",
            ),
        }

        let response = self
            .client
            .get(Self::URL)
            .send()
            .await
            .context("Failed to send HTTP request")?;

        match response.status() {
            StatusCode::OK => self.handle_ok(response).await,
            status => bail!("Fetching failed with status code {status}"),
        }
    }
}
