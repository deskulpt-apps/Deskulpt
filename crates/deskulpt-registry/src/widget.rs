//! Utilities for fetching widgets from the GHCR widgets registry.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Result, bail};
use async_compression::tokio::bufread::GzipDecoder;
use deskulpt_manifest::ManifestMetadata;
use oci_client::manifest::OciDescriptor;
use oci_client::secrets::RegistryAuth;
use oci_client::{Client, Reference};
use serde::{Deserialize, Serialize};
use tokio::io::BufReader;
use tokio_tar::Archive;
use tokio_util::io::StreamReader;

/// A reference to a widget in the registry.
///
/// These information uniquely and immutably identify a widget package in the
/// widgets registry.
#[derive(Debug, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct WidgetReference {
    /// The publisher handle.
    handle: String,
    /// The widget ID.
    ///
    /// Note that this ID is unique only within the publisher's namespace.
    id: String,
    /// The SHA-256 digest of the widget package.
    digest: String,
}

impl WidgetReference {
    /// Get the local ID of the widget.
    ///
    /// It is in the format `@handle.id` in order to be globally unique, valid
    /// as a file name, and human-readable. The prefixing `@` is used to avoid
    /// *accidental* name collisions with purely local widgets.
    pub fn local_id(&self) -> String {
        format!("@{}.{}", self.handle, self.id)
    }
}

/// A descriptor for a widget in the registry.
#[derive(Debug)]
struct WidgetDescriptor {
    /// The full OCI reference of the widget package.
    reference: Reference,
    /// The layer descriptor of the widget package.
    ///
    /// There should be only one layer in the package. This layer contains the
    /// actual widget files, compressed as a gzipped tarball.
    layer: OciDescriptor,
    /// The annotations of the widget package containing widget metadata.
    annotations: Option<BTreeMap<String, String>>,
}

/// Preview information about a widget in the registry.
#[derive(Debug, Default, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct WidgetPreview {
    /// The local ID of the widget.
    ///
    /// See [`RegistryWidgetReference::local_id`] for details.
    id: String,
    /// The size of the widget package in bytes.
    size: u64,
    /// The URL of the widget package in the registry.
    registry_url: String,
    /// The creation datetime of the widget package, in ISO 8601 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = String)]
    created: Option<String>,
    /// The git repository URL of the widget source code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[specta(type = String)]
    git: Option<String>,
    /// More metadata in the widget manifest.
    #[serde(flatten)]
    metadata: ManifestMetadata,
}

/// A fetcher for widgets from the registry.
///
/// Use [`RegistryWidgetFetcher::default`] to create a new instance, which will
/// create a new OCI client internally.
#[derive(Default)]
pub struct WidgetFetcher(Client);

impl WidgetFetcher {
    /// The base URL of the widgets registry in GHCR.
    const REGISTRY_BASE: &str = "ghcr.io/deskulpt-apps/widgets";

    /// The expected artifact type of the widget packages.
    const EXPECTED_ARTIFACT_TYPE: &str = "application/vnd.deskulpt.widget.v1";

    /// Fetch the descriptor of a widget from the registry.
    ///
    /// This does not download the actual widget files, only the metadata. It
    /// verifies that the artifact type, number of layers, and media type of the
    /// layer are as expected.
    async fn fetch(&self, widget: &WidgetReference) -> Result<WidgetDescriptor> {
        let reference: Reference = format!(
            "{}/{}/{}@{}",
            Self::REGISTRY_BASE,
            widget.handle,
            widget.id,
            widget.digest
        )
        .parse()?;

        let (manifest, _) = self
            .0
            .pull_image_manifest(&reference, &RegistryAuth::Anonymous)
            .await?;

        if manifest.artifact_type.as_deref() != Some(Self::EXPECTED_ARTIFACT_TYPE) {
            bail!(
                "Expected artifact type {}, got {:?}",
                Self::EXPECTED_ARTIFACT_TYPE,
                manifest.artifact_type
            );
        }

        let num_layers = manifest.layers.len();
        if num_layers != 1 {
            bail!("Expected only one layer; got {num_layers}",);
        }

        // Safe to unwrap because we have checked that there is one element
        let layer = manifest.layers.into_iter().next().unwrap();
        if !layer.media_type.ends_with("tar+gzip") {
            bail!("Expected gzip-compressed tar; got {}", layer.media_type);
        }

        Ok(WidgetDescriptor {
            reference,
            layer,
            annotations: manifest.annotations,
        })
    }

    /// Install a widget from the registry into the given directory.
    pub async fn install(&self, dir: &Path, widget: &WidgetReference) -> Result<()> {
        let WidgetDescriptor {
            reference, layer, ..
        } = self.fetch(widget).await?;

        let sized_stream = self.0.pull_blob_stream(&reference, &layer).await?;
        let reader = StreamReader::new(sized_stream.stream);

        let buf = BufReader::new(reader);
        let gz = GzipDecoder::new(buf);
        let mut ar = Archive::new(gz);
        ar.unpack(dir).await?;

        Ok(())
    }

    /// Preview metadata about a widget in the registry.
    ///
    /// This does not download the actual widget files, but only fetches the
    /// widget package metadata.
    pub async fn preview(&self, widget: &WidgetReference) -> Result<WidgetPreview> {
        let WidgetDescriptor {
            reference,
            layer,
            annotations,
        } = self.fetch(widget).await?;

        let mut preview = WidgetPreview {
            id: widget.local_id(),
            size: layer.size as u64,
            registry_url: format!("https://{reference}"),
            ..Default::default()
        };

        if let Some(mut annotations) = annotations {
            preview.created = annotations.remove("org.opencontainers.image.created");
            preview.git = annotations
                .remove("org.opencontainers.image.source")
                .and_then(|source| source.split('@').next().map(|s| s.to_string()));

            // Manifest fields
            preview.metadata.name = annotations
                .remove("org.opencontainers.image.title")
                .unwrap_or_default();
            preview.metadata.version = annotations.remove("org.opencontainers.image.version");
            preview.metadata.authors = annotations
                .remove("org.opencontainers.image.authors")
                .and_then(|authors| serde_json::from_str(&authors).ok());
            preview.metadata.license = annotations.remove("org.opencontainers.image.licenses");
            preview.metadata.description =
                annotations.remove("org.opencontainers.image.description");
            preview.metadata.homepage = annotations.remove("org.opencontainers.image.url");
        }

        Ok(preview)
    }
}
